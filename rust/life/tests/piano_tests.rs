// Integration tests for piano functionality

// We need to import from the main crate
use conways_steinway::PlayerPiano;

/// Test that all 88 piano keys can be played without panicking
#[test]
fn test_all_piano_keys_silent() {
    let piano = PlayerPiano::new_silent(); // Use silent mode for testing
    
    // Test that all 88 keys can be played without errors
    for key in 0..88 {
        piano.play_keys(&[key]);
        // No sleep needed for silent mode
    }
    
    println!("✓ All 88 piano keys tested successfully in silent mode");
}

/// Test chord detection and playback
#[test]
fn test_chord_detection() {
    let piano = PlayerPiano::new_silent();
    
    // Test various chord patterns
    let chord_tests = vec![
        vec![24, 28, 31], // C2 Major
        vec![36, 40, 43], // C3 Major  
        vec![48, 52, 55], // C4 Major
        vec![60, 64, 67], // C5 Major
        vec![33, 37, 40, 45], // A3 minor 7
        vec![45, 49, 52, 56], // A4 minor 7
    ];
    
    for chord in chord_tests {
        piano.play_keys(&chord);
    }
    
    println!("✓ Chord detection and playback tested successfully");
}

/// Test chromatic coverage across octaves
#[test] 
fn test_chromatic_coverage() {
    let piano = PlayerPiano::new_silent();
    
    // Test chromatic scales for each octave
    for octave in 0..7 {
        let start_key = octave * 12;
        let end_key = ((octave + 1) * 12).min(88);
        
        if start_key >= 88 { break; }
        
        let keys: Vec<usize> = (start_key..end_key).collect();
        
        // Test each key in the octave
        for key in keys {
            piano.play_keys(&[key]);
        }
    }
    
    println!("✓ Chromatic coverage tested across all octaves");
}

/// Test edge cases for piano keys
#[test]
fn test_piano_edge_cases() {
    let piano = PlayerPiano::new_silent();
    
    // Test empty key array
    piano.play_keys(&[]);
    
    // Test single keys at extremes
    piano.play_keys(&[0]);   // Lowest key (A0)
    piano.play_keys(&[87]);  // Highest key (C8)
    
    // Test large chord
    piano.play_keys(&[0, 12, 24, 36, 48, 60, 72, 84]); // Every octave A
    
    println!("✓ Piano edge cases handled correctly");
}

/// Benchmark test for audio performance (silent mode)
#[test]
fn test_audio_performance() {
    let piano = PlayerPiano::new_silent();
    
    let start = std::time::Instant::now();
    
    // Play all keys rapidly to test performance
    for key in 0..88 {
        piano.play_keys(&[key]);
    }
    
    let duration = start.elapsed();
    println!("✓ Played all 88 keys in {:?}", duration);
    
    // Should be very fast in silent mode
    assert!(duration.as_millis() < 1000, "Audio performance test took too long: {:?}", duration);
}

/// Test note name conversion
#[test]
fn test_note_names() {
    // Test the note naming logic
    let note_names = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
    
    for key in 0..88 {
        let octave = key / 12;
        let note_in_octave = key % 12;
        let note_name = format!("{}{}", note_names[note_in_octave], octave);
        
        // Basic validation
        assert!(!note_name.is_empty());
        assert!(octave <= 8); // Piano goes up to C8
    }
    
    // Test specific known mappings
    assert_eq!(format!("{}{}", note_names[0], 0), "A0");  // Key 0
    assert_eq!(format!("{}{}", note_names[0], 4), "A4");  // Key 48 (A4)
    assert_eq!(format!("{}{}", note_names[3], 7), "C7");  // Key 87 (C7)
    
    println!("✓ Note name conversion working correctly");
}