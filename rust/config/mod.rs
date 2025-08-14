// Conway's Steinway Configuration Module
//
// This module handles all configuration loading and management for the application.

pub mod types;
pub mod loader;

// Re-export core types and functions for easier access
pub use types::{Config, BoardType, GenerationLimit};
pub use loader::{get_config_path, get_default_config_file, load_config};