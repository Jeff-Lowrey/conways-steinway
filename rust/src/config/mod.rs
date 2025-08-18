// Configuration module for Conway's Steinway
// Re-exports configuration types and functions

pub mod types;
mod loader;

pub use types::{Config, BoardType, GenerationLimit, LogDestinationType, DEFAULT_LOG_FILE, DEFAULT_LOG_SUBDIR};
// We don't need to re-export this as it's not used outside the module