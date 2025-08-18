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
            policy::size::SizeBasedTriggerPolicy,
            policy::compound::roll::fixed_window::FixedWindowRoller,
        },
    },
    encode::{pattern::PatternEncoder, json::JsonEncoder},
    config::{Appender, Config, Root, Logger},
    filter::threshold::ThresholdFilter,
};
use std::path::{Path, PathBuf};
use std::fs;

use crate::config::{Config as AppConfig, VALID_LOG_LEVELS, DEFAULT_LOG_FILE};
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
    
    // Always create a console appender
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(CONSOLE_PATTERN)))
        .build();
    
    let console_level = parse_level(&config.log_console_level);
    
    // Start building configuration with console appender
    let mut builder = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(console_level)))
                .build("console", Box::new(console))
        );
    
    let mut root_builder = Root::builder().appender("console");
    
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
            let trigger = SizeBasedTriggerPolicy::new(size_limit);
            
            // Create compound policy that combines trigger and roller
            let policy = CompoundPolicy::new(
                Box::new(trigger),
                Box::new(roller),
            );
            
            // Build the rolling file appender
            let rolling_file = RollingFileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(FILE_PATTERN)))
                .build(log_file_path, Box::new(policy))?;
            
            // Add the rolling file appender to config
            builder = builder.appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(file_level)))
                    .build("rolling_file", Box::new(rolling_file))
            );
            
            root_builder = root_builder.appender("rolling_file");
        } else {
            // Simple file appender without rotation
            let file = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(FILE_PATTERN)))
                .build(log_file_path)?;
            
            // Add the file appender to config
            builder = builder.appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(file_level)))
                    .build("file", Box::new(file))
            );
            
            root_builder = root_builder.appender("file");
        }
    }
    
    // Determine the maximum log level
    let root_level = parse_level(&config.log_level);
    let config = builder.build(root_builder.build(root_level))?;
    
    // Initialize the logging system
    log4rs::init_config(config)?;
    
    // Log confirmation message
    if config.log_to_file {
        let path = get_log_file_path(config);
        info!("Logging to file: {}", path.display());
    }
    
    Ok(())
}

// Helper function to get the log file path
fn get_log_file_path(config: &AppConfig) -> PathBuf {
    match &config.log_file_path {
        Some(path) => path.clone(),
        None => {
            // Get the project root directory by finding the directory containing the logs folder
            let mut path = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            
            // If we're in the rust directory, go up one level
            if path.ends_with("rust") {
                path.pop();
            }
            
            // Add logs directory and file name
            path.push("logs");
            path.push(DEFAULT_LOG_FILE);
            path
        }
    }
}