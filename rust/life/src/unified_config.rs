use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

// Import from config.rs to maintain backward compatibility
use crate::config::{Config, BoardType, GenerationLimit};


pub fn load_unified_config(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    // Create a default config, then use the Config's built-in properties parser
    let mut config = Config::default();
    config.load_from_file(path)?;
    
    // Store the config file path
    config.config_file = Some(path.clone());
    
    Ok(config)
}
