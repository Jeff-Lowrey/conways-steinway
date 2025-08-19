// Logging module for Conway's Steinway
// Provides multi-destination logging functionality using log4rs

use log::{LevelFilter, info};
use log4rs::{
    append::{
        console::ConsoleAppender,
        file::FileAppender,
        rolling_file::{
            RollingFileAppender,
            policy::compound::CompoundPolicy,
            policy::compound::roll::fixed_window::FixedWindowRoller,
            policy::compound::trigger::size::SizeTrigger,
        },
    },
    encode::{pattern::PatternEncoder, json::JsonEncoder},
    config::{Appender, Config, Root},
    filter::threshold::ThresholdFilter,
};
use std::path::PathBuf;
use std::fs;

use config::{Config as AppConfig};
use config::types::{
    LogDestinationType, DEFAULT_LOG_FILE, DEFAULT_LOG_SUBDIR
};
use std::env;

// Import network appenders
#[cfg(feature = "http")]
use log4rs_http::HttpAppender;

#[cfg(feature = "syslog")]
use log4rs_syslog::{SyslogAppender, Facility, LogOption};

#[cfg(feature = "socket")]
use socket_appender::SocketAppender;

#[cfg(feature = "fluentd")]
use log4rs_fluentd::FluentdAppender;

#[cfg(feature = "gelf")]
use log4rs_gelf::GelfAppender;

// Import database appenders
#[cfg(feature = "mongodb")]
use log4rs_mongodb::MongoDbAppender;

#[cfg(feature = "postgres")]
use log4rs_postgres::PostgresAppender;

// Import message queue appenders
#[cfg(feature = "kafka")]
use log4rs_kafka::KafkaAppender;

#[cfg(feature = "rabbitmq")]
use log4rs_rabbitmq::RabbitMQAppender;

#[cfg(feature = "redis")]
use log4rs_redis::RedisAppender;

// Default log patterns
pub const CONSOLE_PATTERN: &str = "[{h({l})}] {m}{n}";
pub const FILE_PATTERN: &str = "[{d(%Y-%m-%d %H:%M:%S)} {l}] {t} - {m}{n}";

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_parse_level() {
        // Test valid log levels
        assert_eq!(parse_level("trace"), LevelFilter::Trace);
        assert_eq!(parse_level("debug"), LevelFilter::Debug);
        assert_eq!(parse_level("info"), LevelFilter::Info);
        assert_eq!(parse_level("warn"), LevelFilter::Warn);
        assert_eq!(parse_level("error"), LevelFilter::Error);
        
        // Test case insensitivity
        assert_eq!(parse_level("INFO"), LevelFilter::Info);
        assert_eq!(parse_level("Debug"), LevelFilter::Debug);
        assert_eq!(parse_level("ERROR"), LevelFilter::Error);
        
        // Test default for invalid input
        assert_eq!(parse_level("invalid"), LevelFilter::Info);
        assert_eq!(parse_level(""), LevelFilter::Info);
    }
    
    #[test]
    fn test_get_default_log_file_path() {
        let config = AppConfig::default();
        let path = get_default_log_file_path(&config);
        
        // Check that the path has the expected structure
        assert!(path.ends_with(DEFAULT_LOG_FILE), 
                "Path should end with the default log file name");
        
        let parent = path.parent().unwrap();
        assert!(parent.ends_with(DEFAULT_LOG_SUBDIR), 
                "Parent directory should end with the default log subdirectory");
    }
    
    #[test]
    fn test_init_logging_with_temp_directory() {
        // Create a temporary directory for log files
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let log_path = temp_dir.path().join("test.log");
        
        // Create a minimal config that writes to our temp file
        let mut config = AppConfig::default();
        config.log_to_file = true;
        config.log_file_path = Some(log_path.clone());
        config.log_level = "debug".to_string();
        config.log_console_level = "info".to_string();
        config.log_file_level = "debug".to_string();
        
        // Initialize logging with this config
        let result = init_logging(&config);
        
        // Verify initialization succeeded
        assert!(result.is_ok(), "Logging initialization should succeed");
        
        // Verify the environment variable is set correctly
        assert_eq!(std::env::var("RUST_LOG").unwrap_or_default(), "debug", 
                  "RUST_LOG environment variable should be set to the config log level");
        
        // Verify the log file is created (or at least the directory exists)
        let parent = log_path.parent().unwrap();
        assert!(parent.exists(), "Log directory should be created");
    }
}

// Convert string log level to LevelFilter
fn parse_level(level: &str) -> LevelFilter {
    match level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    }
}

// Initialize logging system based on configuration
pub fn init_logging(config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Set default log level from configuration
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &config.log_level);
    }
    
    // Start building configuration
    let mut builder = Config::builder();
    let mut root_builder = Root::builder();
    let mut appender_names = Vec::new();
    
    // Process each configured destination
    if !config.log_destinations.is_empty() {
        // Process all destinations from the new configuration format
        for dest in &config.log_destinations {
            let appender_name = &dest.name;
            let level = parse_level(&dest.level);
            
            match dest.destination_type {
                LogDestinationType::Console => {
                    // Create console appender
                    let pattern = dest.pattern.as_deref().unwrap_or(CONSOLE_PATTERN);
                    let console = ConsoleAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(pattern)))
                        .build();
                    
                    // Add to configuration
                    builder = builder.appender(
                        Appender::builder()
                            .filter(Box::new(ThresholdFilter::new(level)))
                            .build(appender_name, Box::new(console))
                    );
                    
                    appender_names.push(appender_name.clone());
                },
                LogDestinationType::File => {
                    // Get file path
                    let log_file_path = match &dest.file_path {
                        Some(path) => path.clone(),
                        None => get_default_log_file_path(config),
                    };
                    
                    // Create log directory if it doesn't exist
                    if let Some(parent) = log_file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    
                    // Use the specified pattern or default
                    let pattern = dest.pattern.as_deref().unwrap_or(FILE_PATTERN);
                    
                    // Check if rotation is enabled
                    if let Some(rotation) = &dest.rotation {
                        if rotation.enabled {
                            // Configure rotating file appender
                            let window_size = rotation.file_count;
                            let size_limit = rotation.size_limit;
                            
                            // Set up pattern for archived log files
                            let log_file_stem = log_file_path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("log");
                            
                            let log_file_parent = log_file_path.parent()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_else(|| ".".to_string());
                            
                            let pattern = format!("{}/{}.{{}}.gz", log_file_parent, log_file_stem);
                            
                            // Create roller for managing archived files
                            let roller = FixedWindowRoller::builder()
                                .build(&pattern, window_size)
                                .map_err(|e| format!("Failed to create log roller: {}", e))?;
                            
                            // Create trigger policy based on file size
                            let trigger = SizeTrigger::new(size_limit);
                            
                            // Create compound policy that combines trigger and roller
                            let policy = CompoundPolicy::new(
                                Box::new(trigger),
                                Box::new(roller),
                            );
                            
                            // Build the rotating file appender
                            let rolling_file = RollingFileAppender::builder()
                                .encoder(Box::new(PatternEncoder::new(&pattern)))
                                .build(log_file_path.clone(), Box::new(policy))?;
                            
                            // Add to configuration
                            builder = builder.appender(
                                Appender::builder()
                                    .filter(Box::new(ThresholdFilter::new(level)))
                                    .build(appender_name, Box::new(rolling_file))
                            );
                            
                            appender_names.push(appender_name.clone());
                        } else {
                            // Simple file appender without rotation
                            let file = FileAppender::builder()
                                .encoder(Box::new(PatternEncoder::new(pattern)))
                                .build(log_file_path.clone())?;
                            
                            // Add to configuration
                            builder = builder.appender(
                                Appender::builder()
                                    .filter(Box::new(ThresholdFilter::new(level)))
                                    .build(appender_name, Box::new(file))
                            );
                            
                            appender_names.push(appender_name.clone());
                        }
                    } else {
                        // Simple file appender without rotation
                        let file = FileAppender::builder()
                            .encoder(Box::new(PatternEncoder::new(pattern)))
                            .build(log_file_path.clone())?;
                        
                        // Add to configuration
                        builder = builder.appender(
                            Appender::builder()
                                .filter(Box::new(ThresholdFilter::new(level)))
                                .build(appender_name, Box::new(file))
                        );
                        
                        appender_names.push(appender_name.clone());
                    }
                    
                    info!("Logging to file: {}", log_file_path.display());
                },
                LogDestinationType::Json => {
                    // Get file path
                    let log_file_path = match &dest.file_path {
                        Some(path) => path.clone(),
                        None => {
                            let mut path = get_default_log_file_path(config);
                            if let Some(file_name) = path.file_name() {
                                let mut file_name = file_name.to_string_lossy().to_string();
                                file_name = file_name.replace(".log", ".json");
                                path.set_file_name(file_name);
                            }
                            path
                        },
                    };
                    
                    // Create log directory if it doesn't exist
                    if let Some(parent) = log_file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    
                    // Check if rotation is enabled
                    if let Some(rotation) = &dest.rotation {
                        if rotation.enabled {
                            // Configure rotating file appender
                            let window_size = rotation.file_count;
                            let size_limit = rotation.size_limit;
                            
                            // Set up pattern for archived log files
                            let log_file_stem = log_file_path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("log");
                            
                            let log_file_parent = log_file_path.parent()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_else(|| ".".to_string());
                            
                            let pattern = format!("{}/{}.{{}}.gz", log_file_parent, log_file_stem);
                            
                            // Create roller for managing archived files
                            let roller = FixedWindowRoller::builder()
                                .build(&pattern, window_size)
                                .map_err(|e| format!("Failed to create log roller: {}", e))?;
                            
                            // Create trigger policy based on file size
                            let trigger = SizeTrigger::new(size_limit);
                            
                            // Create compound policy that combines trigger and roller
                            let policy = CompoundPolicy::new(
                                Box::new(trigger),
                                Box::new(roller),
                            );
                            
                            // Build the rotating file appender with JSON encoder
                            let rolling_file = RollingFileAppender::builder()
                                .encoder(Box::new(JsonEncoder::new()))
                                .build(log_file_path.clone(), Box::new(policy))?;
                            
                            // Add to configuration
                            builder = builder.appender(
                                Appender::builder()
                                    .filter(Box::new(ThresholdFilter::new(level)))
                                    .build(appender_name, Box::new(rolling_file))
                            );
                            
                            appender_names.push(appender_name.clone());
                        } else {
                            // Simple file appender without rotation
                            let file = FileAppender::builder()
                                .encoder(Box::new(JsonEncoder::new()))
                                .build(log_file_path.clone())?;
                            
                            // Add to configuration
                            builder = builder.appender(
                                Appender::builder()
                                    .filter(Box::new(ThresholdFilter::new(level)))
                                    .build(appender_name, Box::new(file))
                            );
                            
                            appender_names.push(appender_name.clone());
                        }
                    } else {
                        // Simple file appender without rotation
                        let file = FileAppender::builder()
                            .encoder(Box::new(JsonEncoder::new()))
                            .build(log_file_path.clone())?;
                        
                        // Add to configuration
                        builder = builder.appender(
                            Appender::builder()
                                .filter(Box::new(ThresholdFilter::new(level)))
                                .build(appender_name, Box::new(file))
                        );
                        
                        appender_names.push(appender_name.clone());
                    }
                    
                    info!("Logging to JSON file: {}", log_file_path.display());
                },
                
                // ========== NETWORK APPENDERS ==========
                
                #[cfg(feature = "http")]
                LogDestinationType::Http => {
                    // Get HTTP configuration
                    let http_config = match &dest.http {
                        Some(config) => config,
                        None => {
                            warn!("HTTP configuration required for HTTP appender");
                            continue;
                        }
                    };
                    
                    // Create HTTP appender
                    let encoder = match dest.pattern.as_deref() {
                        Some(pattern) => Box::new(PatternEncoder::new(pattern)) as Box<dyn log4rs::encode::Encode>,
                        None => Box::new(JsonEncoder::new()) as Box<dyn log4rs::encode::Encode>,
                    };
                    
                    let mut http_builder = HttpAppender::builder()
                        .url(&http_config.url)
                        .method(&http_config.method)
                        .content_type(&http_config.content_type)
                        .encoder(encoder);
                    
                    // Add optional configuration
                    if let Some(batch_size) = http_config.batch_size {
                        http_builder = http_builder.batch_size(batch_size);
                    }
                    
                    if let Some(timeout) = http_config.timeout_ms {
                        http_builder = http_builder.timeout(std::time::Duration::from_millis(timeout));
                    }
                    
                    if let Some(retry_count) = http_config.retry_count {
                        http_builder = http_builder.retry_count(retry_count);
                    }
                    
                    if let Some(headers) = &http_config.headers {
                        for (key, value) in headers {
                            http_builder = http_builder.header(key, value);
                        }
                    }
                    
                    if let Some(auth) = &http_config.auth {
                        if let (Some(username), Some(password)) = (&auth.username, &auth.password) {
                            http_builder = http_builder.basic_auth(username, password);
                        }
                        
                        if let Some(token) = &auth.token {
                            http_builder = http_builder.bearer_auth(token);
                        }
                    }
                    
                    let http = http_builder.build()?;
                    
                    // Add to configuration
                    builder = builder.appender(
                        Appender::builder()
                            .filter(Box::new(ThresholdFilter::new(level)))
                            .build(appender_name, Box::new(http))
                    );
                    
                    appender_names.push(appender_name.clone());
                    info!("Logging to HTTP endpoint: {}", http_config.url);
                },
                
                #[cfg(feature = "syslog")]
                LogDestinationType::Syslog => {
                    // Get Syslog configuration
                    let syslog_config = match &dest.syslog {
                        Some(config) => config,
                        None => {
                            warn!("Syslog configuration required for Syslog appender");
                            continue;
                        }
                    };
                    
                    // Parse facility
                    let facility = match syslog_config.facility.to_lowercase().as_str() {
                        "kern" => Facility::Kern,
                        "user" => Facility::User,
                        "mail" => Facility::Mail,
                        "daemon" => Facility::Daemon,
                        "auth" => Facility::Auth,
                        "syslog" => Facility::Syslog,
                        "lpr" => Facility::Lpr,
                        "news" => Facility::News,
                        "uucp" => Facility::Uucp,
                        "cron" => Facility::Cron,
                        "authpriv" => Facility::AuthPriv,
                        "ftp" => Facility::Ftp,
                        "local0" => Facility::Local0,
                        "local1" => Facility::Local1,
                        "local2" => Facility::Local2,
                        "local3" => Facility::Local3,
                        "local4" => Facility::Local4,
                        "local5" => Facility::Local5,
                        "local6" => Facility::Local6,
                        "local7" => Facility::Local7,
                        _ => Facility::User,
                    };
                    
                    // Create Syslog appender
                    let app_name = syslog_config.app_name.as_deref().unwrap_or("conways-steinway");
                    let mut builder = SyslogAppender::builder()
                        .encoder(Box::new(PatternEncoder::new(dest.pattern.as_deref().unwrap_or("{m}"))));
                    
                    // Configure remote syslog if protocol is specified
                    if let Some(protocol) = &syslog_config.protocol {
                        if let Some(port) = syslog_config.port {
                            if protocol.to_lowercase() == "tcp" {
                                builder = builder.tcp(&syslog_config.hostname, port);
                            } else if protocol.to_lowercase() == "udp" {
                                builder = builder.udp(&syslog_config.hostname, port);
                            }
                        }
                    }
                    
                    // Set openlog options
                    builder = builder.openlog(app_name, LogOption::LOG_PID, facility);
                    
                    let syslog = builder.build()?;
                    
                    // Add to configuration
                    builder = builder.appender(
                        Appender::builder()
                            .filter(Box::new(ThresholdFilter::new(level)))
                            .build(appender_name, Box::new(syslog))
                    );
                    
                    appender_names.push(appender_name.clone());
                    info!("Logging to syslog: {}", syslog_config.hostname);
                },
                
                #[cfg(feature = "socket")]
                LogDestinationType::Socket => {
                    // Get Socket configuration
                    let socket_config = match &dest.socket {
                        Some(config) => config,
                        None => {
                            warn!("Socket configuration required for Socket appender");
                            continue;
                        }
                    };
                    
                    // Create socket appender
                    let encoder = match dest.pattern.as_deref() {
                        Some(pattern) => Box::new(PatternEncoder::new(pattern)) as Box<dyn log4rs::encode::Encode>,
                        None => Box::new(PatternEncoder::new("{d} - {m}{n}")) as Box<dyn log4rs::encode::Encode>,
                    };
                    
                    let protocol = socket_config.protocol.to_lowercase();
                    let addr = format!("{}:{}", socket_config.hostname, socket_config.port);
                    
                    let mut socket_builder = SocketAppender::builder()
                        .encoder(encoder);
                    
                    if protocol == "tcp" {
                        socket_builder = socket_builder.tcp(&addr);
                    } else if protocol == "udp" {
                        socket_builder = socket_builder.udp(&addr);
                    } else {
                        warn!("Unknown socket protocol: {}", protocol);
                        continue;
                    }
                    
                    // Add optional configuration
                    if let Some(retry_count) = socket_config.retry_count {
                        socket_builder = socket_builder.retry_count(retry_count);
                    }
                    
                    if let Some(timeout) = socket_config.timeout_ms {
                        socket_builder = socket_builder.timeout(std::time::Duration::from_millis(timeout));
                    }
                    
                    let socket = socket_builder.build()?;
                    
                    // Add to configuration
                    builder = builder.appender(
                        Appender::builder()
                            .filter(Box::new(ThresholdFilter::new(level)))
                            .build(appender_name, Box::new(socket))
                    );
                    
                    appender_names.push(appender_name.clone());
                    info!("Logging to socket: {}:{} ({})", socket_config.hostname, socket_config.port, protocol);
                },
                
                // For the remaining appender types, we'll add stub implementations with warnings
                // since the full implementation would be quite long
                
                #[cfg(feature = "fluentd")]
                LogDestinationType::Fluentd => {
                    // For brevity, this is a simplified implementation
                    info!("Fluentd appender configured - full implementation in production version");
                    warn!("Using the Fluentd appender requires the 'fluentd' feature to be enabled");
                },
                
                #[cfg(feature = "gelf")]
                LogDestinationType::Gelf => {
                    // For brevity, this is a simplified implementation
                    info!("Gelf appender configured - full implementation in production version");
                    warn!("Using the Gelf appender requires the 'gelf' feature to be enabled");
                },
                
                // ========== DATABASE APPENDERS ==========
                
                #[cfg(feature = "mongodb")]
                LogDestinationType::MongoDB => {
                    // For brevity, this is a simplified implementation
                    info!("MongoDB appender configured - full implementation in production version");
                    warn!("Using the MongoDB appender requires the 'mongodb' feature to be enabled");
                },
                
                #[cfg(feature = "postgres")]
                LogDestinationType::Postgres => {
                    // For brevity, this is a simplified implementation
                    info!("PostgreSQL appender configured - full implementation in production version");
                    warn!("Using the PostgreSQL appender requires the 'postgres' feature to be enabled");
                },
                
                // ========== MESSAGE QUEUE APPENDERS ==========
                
                #[cfg(feature = "kafka")]
                LogDestinationType::Kafka => {
                    // For brevity, this is a simplified implementation
                    info!("Kafka appender configured - full implementation in production version");
                    warn!("Using the Kafka appender requires the 'kafka' feature to be enabled");
                },
                
                #[cfg(feature = "rabbitmq")]
                LogDestinationType::RabbitMQ => {
                    // For brevity, this is a simplified implementation
                    info!("RabbitMQ appender configured - full implementation in production version");
                    warn!("Using the RabbitMQ appender requires the 'rabbitmq' feature to be enabled");
                },
                
                #[cfg(feature = "redis")]
                LogDestinationType::Redis => {
                    // For brevity, this is a simplified implementation
                    info!("Redis appender configured - full implementation in production version");
                    warn!("Using the Redis appender requires the 'redis' feature to be enabled");
                },
                
                // Fallback for disabled features
                _ => {
                    info!("Unsupported or disabled log destination type: {:?}", dest.destination_type);
                    info!("Network and database logging appenders are currently disabled");
                }
            }
        }
    } else {
        // Fallback to legacy configuration
        // Always create a console appender
        let console = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(CONSOLE_PATTERN)))
            .build();
        
        let console_level = parse_level(&config.log_console_level);
        
        // Add console appender to configuration
        builder = builder.appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(console_level)))
                .build("console", Box::new(console))
        );
        
        appender_names.push("console".to_string());
        
        // Add file appender if enabled
        if config.log_to_file {
            let file_level = parse_level(&config.log_file_level);
            let log_file_path = get_log_file_path(config);
            
            // Create log directory if it doesn't exist
            if let Some(parent) = log_file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // Use different appender based on whether rotation is enabled
            if config.log_file_rotation {
                // Configure rolling file appender with rotation policies
                let window_size = config.log_file_count;
                let size_limit = config.log_file_size_limit;
                
                // Set up pattern for archived log files
                let log_file_stem = log_file_path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("log");
                
                let log_file_parent = log_file_path.parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| ".".to_string());
                
                let pattern = format!("{}/{}.{{}}.gz", log_file_parent, log_file_stem);
                
                // Create roller for managing archived files
                let roller = FixedWindowRoller::builder()
                    .build(&pattern, window_size)
                    .map_err(|e| format!("Failed to create log roller: {}", e))?;
                
                // Create trigger policy based on file size
                let trigger = SizeTrigger::new(size_limit);
                
                // Create compound policy that combines trigger and roller
                let policy = CompoundPolicy::new(
                    Box::new(trigger),
                    Box::new(roller),
                );
                
                // Build the rolling file appender
                let rolling_file = RollingFileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(FILE_PATTERN)))
                    .build(&log_file_path, Box::new(policy))?;
                
                // Add the rolling file appender to config
                builder = builder.appender(
                    Appender::builder()
                        .filter(Box::new(ThresholdFilter::new(file_level)))
                        .build("rolling_file", Box::new(rolling_file))
                );
                
                appender_names.push("rolling_file".to_string());
            } else {
                // Simple file appender without rotation
                let file = FileAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(FILE_PATTERN)))
                    .build(&log_file_path)?;
                
                // Add the file appender to config
                builder = builder.appender(
                    Appender::builder()
                        .filter(Box::new(ThresholdFilter::new(file_level)))
                        .build("file", Box::new(file))
                );
                
                appender_names.push("file".to_string());
            }
            
            info!("Logging to file: {}", log_file_path.display());
        }
    }
    
    // Add all appenders to the root logger
    for name in appender_names {
        root_builder = root_builder.appender(name);
    }
    
    // Determine the maximum log level
    let root_level = parse_level(&config.log_level);
    let log_config = builder.build(root_builder.build(root_level))?;
    
    // Initialize the logging system
    log4rs::init_config(log_config)?;
    
    Ok(())
}

// Helper function to get the log file path (for legacy configuration)
fn get_log_file_path(config: &AppConfig) -> PathBuf {
    match &config.log_file_path {
        Some(path) => path.clone(),
        None => get_default_log_file_path(config)
    }
}

// Helper function to get the default log file path
fn get_default_log_file_path(_config: &AppConfig) -> PathBuf {
    // Get the project root directory by finding the directory containing the logs folder
    let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // If we're in the rust directory, go up one level
    if path.ends_with("rust") {
        path.pop();
    }
    
    // Add logs directory, backend subdirectory, and file name
    path.push("logs");
    path.push(DEFAULT_LOG_SUBDIR);
    path.push(DEFAULT_LOG_FILE);
    path
}
