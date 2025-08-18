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
use config::types::{LogDestinationType, DEFAULT_LOG_FILE, DEFAULT_LOG_SUBDIR};
use std::env;

// Default log patterns
const CONSOLE_PATTERN: &str = "[{h({l})}] {m}{n}";
const FILE_PATTERN: &str = "[{d(%Y-%m-%d %H:%M:%S)} {l}] {t} - {m}{n}";

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