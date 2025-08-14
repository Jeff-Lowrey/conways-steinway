// Configuration loader for Conway's Steinway
// Loads configuration from the global config directory

use std::path::PathBuf;
use std::env;

// Re-export Config, BoardType, and GenerationLimit from the config module
pub use crate::config::{Config, BoardType, GenerationLimit};

// Import unified config loader
use crate::unified_config::load_unified_config;

pub fn get_config_path() -> PathBuf {
    // Try to find the config directory relative to the current executable
    let mut config_path = match env::current_exe() {
        Ok(exe_path) => {
            let mut path = exe_path;
            // Navigate up to project root then to config
            path.pop(); // Remove executable name
            if path.ends_with("debug") || path.ends_with("release") {
                path.pop(); // Remove debug/release
                path.pop(); // Remove target
                path.pop(); // Remove rust/life
            }
            path.push("config");
            path
        },
        Err(_) => {
            // Fallback to assuming we're in the project root
            let mut path = PathBuf::from(".");
            path.push("config");
            path
        }
    };
    
    config_path
}

pub fn get_default_config_file() -> PathBuf {
    let mut path = get_config_path();
    path.push("conways_steinway.properties");
    path
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config = Config::from_args_and_env()?;
    
    // If no config file was specified via command line, try the default location
    if config.config_file.is_none() {
        let default_config = get_default_config_file();
        if default_config.exists() {
            config.config_file = Some(default_config.clone());
            
            // Try to load from the unified config file first
            match load_unified_config(&default_config) {
                Ok(unified_config) => {
                    // Take values from unified config but preserve command line args
                    // We merge by preserving the current values that might have been set by command line
                    let board_type = config.board_type;
                    let audio_enabled = config.audio_enabled;
                    let generations = config.generations.clone();
                    let step_delay_ms = config.step_delay_ms;
                    let tempo_bpm = config.tempo_bpm;
                    
                    // Replace config with unified config
                    config = unified_config;
                    
                    // Restore values that were explicitly set via command line
                    if board_type != BoardType::Random {
                        config.board_type = board_type;
                    }
                    
                    // Only restore audio_enabled if it was explicitly disabled
                    if !audio_enabled {
                        config.audio_enabled = false;
                    }
                    
                    // For other options, we'll prefer command line args when they differ from defaults
                    if generations != GenerationLimit::Unlimited {
                        config.generations = generations;
                    }
                    
                    if step_delay_ms != 200 {
                        config.step_delay_ms = step_delay_ms;
                    }
                    
                    if tempo_bpm.is_some() {
                        config.tempo_bpm = tempo_bpm;
                    }
                },
                Err(e) => {
                    // Fall back to traditional config loading
                    eprintln!("Note: Using legacy config format. Error with unified format: {}", e);
                    if let Err(e) = config.load_from_file(&default_config) {
                        eprintln!("Warning: Error loading default config file: {}", e);
                    }
                }
            }
        }
    }
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_paths() {
        // Test that the paths at least return something
        let config_path = get_config_path();
        
        // The path should end with config
        assert!(config_path.ends_with("config") || 
                config_path.to_string_lossy().contains("config"));
        
        let config_file = get_default_config_file();
        assert!(config_file.ends_with("conways_steinway.toml"));
    }
}
