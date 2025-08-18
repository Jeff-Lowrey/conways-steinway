// Integration tests for audio logic
// These tests verify audio functionality without actually producing sound

use audio::PlayerPiano;

#[test]
fn test_silent_piano_basic() {
    // Create a silent piano
    let piano = PlayerPiano::new_silent();
    
    // Basic operations should not panic
    piano.play_keys(&[]);
    piano.play_keys(&[0]);
    piano.play_keys(&[87]);  // Highest key
    piano.play_keys(&[1, 5, 10, 15]);  // Multiple keys
    
    // Since we're using a silent piano, we can't verify the sound output,
    // but we can verify the code executes without panicking
}

#[test]
fn test_piano_key_range() {
    // Create a silent piano
    let piano = PlayerPiano::new_silent();
    
    // Test valid key ranges
    for key in 0..88 {
        piano.play_keys(&[key]);  // Should not panic for valid piano keys
    }
    
    // Test edge cases - these should not panic
    piano.play_keys(&[0, 87]);  // Lowest and highest keys
    piano.play_keys(&[0, 1, 2, 3, 4, 5]);  // Multiple consecutive keys
    
    // Test all keys at once - should not panic
    let all_keys: Vec<usize> = (0..88).collect();
    piano.play_keys(&all_keys);
}