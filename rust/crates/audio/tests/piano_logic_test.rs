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

#[test]
fn test_real_audio_engine_initialization() {
    // This test verifies that we can initialize a real audio engine
    // with the configured audio sample paths
    
    // Create a real piano (not silent)
    let _piano = PlayerPiano::new();
    
    // Just verify initialization doesn't panic
    // We won't actually play notes to avoid making noise during tests
    
    // Note: We can't test disable/enable methods here as they're
    // only available when the #[cfg(test)] attribute is set at the PlayerPiano module level
    
    println!("Successfully verified real audio engine initialization");
}
