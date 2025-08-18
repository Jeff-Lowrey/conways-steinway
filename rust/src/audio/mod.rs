// Audio module for Conway's Steinway
// Re-exports audio components

mod audio_engine;
mod piano_player;

// Re-export what's needed
pub use piano_player::PlayerPiano;