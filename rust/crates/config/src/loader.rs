// Configuration loader for Conway's Steinway
// Loads configuration from the config directory

// Module currently only used in tests
#[cfg(test)]
use std::path::PathBuf;
#[cfg(test)]
use std::env;

// Import from types.rs (only used in tests)
#[cfg(test)]
use super::types::Config;

// Utility function to find configuration path
#[cfg(test)]
fn get_config_path() -> PathBuf {
    // Try to find the config directory relative to the current executable
    match env::current_exe() {
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
    }
}

// Helper to find default config file location
#[cfg(test)]
fn get_default_config_file() -> PathBuf {
    let mut path = get_config_path();
    path.push("conways_steinway.toml");
    path
}

// Function to load configuration from args and file
#[cfg(test)]
#[allow(dead_code)]
fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
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