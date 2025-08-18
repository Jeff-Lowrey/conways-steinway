// Configuration loader for Conway's Steinway
// Loads configuration from the config directory

use std::path::PathBuf;
use std::env;

// Import from types.rs
use super::types::Config;

pub fn get_config_path() -> PathBuf {
    // Try to find the config directory relative to the current executable
    let mut config_path = match env::current_exe() {
        Ok(exe_path) => {
            let mut path = exe_path;
            // Navigate up to project root then to config/rust
            path.pop(); // Remove executable name
            if path.ends_with("debug") || path.ends_with("release") {
                path.pop(); // Remove debug/release
                path.pop(); // Remove target
                path.pop(); // Remove rust/life
            }
            path.push("config");
            path.push("rust");
            path
        },
        Err(_) => {
            // Fallback to assuming we're in the project root
            let mut path = PathBuf::from(".");
            path.push("config");
            path.push("rust");
            path
        }
    };
    
    config_path
}

pub fn get_default_config_file() -> PathBuf {
    let mut path = get_config_path();
    path.push("conways_steinway.toml");
    path
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config = Config::from_args_and_env()?;
    
    // If no config file was specified via command line, try the default location
    if config.config_file.is_none() {
        let default_config = get_default_config_file();
        if default_config.exists() {
            config.config_file = Some(default_config.clone());
            // Try to load from the default config file
            if let Err(e) = config.load_from_file(&default_config) {
                eprintln!("Warning: Error loading default config file: {}", e);
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
        
        // The path should end with config/rust
        assert!(config_path.ends_with("config/rust") || 
                config_path.to_string_lossy().contains("config/rust"));
        
        let config_file = get_default_config_file();
        assert!(config_file.ends_with("conways_steinway.toml"));
    }
}