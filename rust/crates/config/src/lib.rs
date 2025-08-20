// Config module library for Conway's Steinway
// Provides configuration loading and validation functionality

pub mod loader;
pub mod types;

// Re-export commonly used types for convenience
pub use types::{Config, BoardType, GenerationLimit};
