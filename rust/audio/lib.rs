// Audio module library for Conway's Steinway
// Provides piano audio synthesis and playback functionality

pub mod audio;
pub mod piano_player;

// Re-export commonly used types for convenience
pub use audio::{AudioEngine, NullAudioEngine, AudioPlayer};
pub use piano_player::PlayerPiano;