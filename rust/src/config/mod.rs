// Configuration module for Conway's Steinway
// Re-exports configuration types and functions

mod types;
mod loader;

pub use types::{Config, BoardType, GenerationLimit, VALID_LOG_LEVELS};
pub use loader::load_config;