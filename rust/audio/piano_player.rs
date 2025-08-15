use super::audio::{AudioPlayer, AudioEngine, NullAudioEngine};
use log::{info, debug};

pub struct PlayerPiano {
    audio_engine: Box<dyn AudioPlayer>,
}

impl PlayerPiano {
    pub fn new() -> Self {
        PlayerPiano {
            audio_engine: Box::new(AudioEngine::new()),
        }
    }

    pub fn new_silent() -> Self {
        PlayerPiano {
            audio_engine: Box::new(NullAudioEngine::new()),
        }
    }

    pub fn play_keys(&self, keys: &[usize]) {
        if keys.is_empty() {
            info!("♪ Silence");
            return;
        }

        // Check if this looks like a chord pattern
        let is_chord = self.is_chord_pattern(keys);
        
        let mut key_str = String::new();
        for (i, &key) in keys.iter().enumerate() {
            if i > 0 { key_str.push_str(", "); }
            key_str.push_str(&format!("{}", key + 1));
        }
        
        if is_chord {
            info!("♫ Playing chord: {}", key_str);
        } else {
            info!("♪ Playing piano keys: {}", key_str);
        }

        self.audio_engine.play_piano_keys(keys);
    }

    fn is_chord_pattern(&self, keys: &[usize]) -> bool {
        if keys.len() < 3 {
            return false;
        }
        
        let mut sorted_keys: Vec<usize> = keys.to_vec();
        sorted_keys.sort();
        
        // Check for triads (3 notes)
        if sorted_keys.len() >= 3 {
            for i in 0..=sorted_keys.len()-3 {
                let root = sorted_keys[i];
                let third = sorted_keys[i+1];
                let fifth = sorted_keys[i+2];
                
                let interval1 = third.saturating_sub(root);
                let interval2 = fifth.saturating_sub(root);
                
                // Major chord: 4 and 7 semitones
                // Minor chord: 3 and 7 semitones
                // Diminished chord: 3 and 6 semitones
                // Augmented chord: 4 and 8 semitones
                if (interval1 == 3 || interval1 == 4) && 
                   (interval2 >= 6 && interval2 <= 8) {
                    return true;
                }
            }
        }
        
        // Check for dense clusters (many consecutive notes)
        if sorted_keys.len() >= 5 {
            let mut consecutive_count = 1;
            for i in 1..sorted_keys.len() {
                if sorted_keys[i] - sorted_keys[i-1] <= 2 {
                    consecutive_count += 1;
                    if consecutive_count >= 5 {
                        return true;
                    }
                } else {
                    consecutive_count = 1;
                }
            }
        }
        
        false
    }

    pub fn disable_audio(&mut self) {
        self.audio_engine = Box::new(NullAudioEngine::new());
    }

    pub fn enable_audio(&mut self) {
        self.audio_engine = Box::new(AudioEngine::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_piano_creation() {
        // Test both audio and silent modes
        let _piano_audio = PlayerPiano::new();
        let piano_silent = PlayerPiano::new_silent();
        
        // Basic smoke test - should not panic
        piano_silent.play_keys(&[]);
        piano_silent.play_keys(&[48]);
    }

    #[test]
    fn test_chord_detection() {
        let piano = PlayerPiano::new_silent();
        
        // Test chord detection logic
        assert!(piano.is_chord_pattern(&[24, 28, 31])); // C Major
        assert!(piano.is_chord_pattern(&[24, 27, 31])); // C minor
        assert!(piano.is_chord_pattern(&[48, 49, 50, 51, 52])); // Dense cluster
        
        assert!(!piano.is_chord_pattern(&[]));         // Empty
        assert!(!piano.is_chord_pattern(&[24]));       // Single note
        assert!(!piano.is_chord_pattern(&[24, 25]));   // Two notes only
    }

    #[test]
    fn test_play_keys_edge_cases() {
        let piano = PlayerPiano::new_silent();
        
        // Test various edge cases
        piano.play_keys(&[]);           // Empty array
        piano.play_keys(&[0]);          // Lowest key
        piano.play_keys(&[87]);         // Highest key
        piano.play_keys(&[0, 87]);      // Extremes together
        
        // Test large arrays
        let all_keys: Vec<usize> = (0..88).collect();
        piano.play_keys(&all_keys);
        
        // Test repeated keys
        piano.play_keys(&[48, 48, 48]);
    }

    #[test]
    fn test_chord_vs_individual_note_detection() {
        let piano = PlayerPiano::new_silent();
        
        // These should be detected as chords (output should show ♫)
        let chord_patterns = vec![
            vec![24, 28, 31],     // C Major triad
            vec![36, 40, 43],     // C Major in higher octave
            vec![48, 52, 55],     // Another C Major
            vec![33, 37, 40],     // A minor triad
        ];
        
        for chord in chord_patterns {
            assert!(piano.is_chord_pattern(&chord), "Should detect {:?} as chord", chord);
        }
        
        // These should NOT be detected as chords (output should show ♪)
        let individual_patterns = vec![
            vec![48],             // Single note
            vec![48, 49],         // Two notes
            vec![48, 60],         // Two notes, wide interval
        ];
        
        for pattern in individual_patterns {
            assert!(!piano.is_chord_pattern(&pattern), "Should NOT detect {:?} as chord", pattern);
        }
    }

    #[test]
    fn test_audio_engine_switching() {
        let mut piano = PlayerPiano::new_silent();
        
        // Test that we can switch between audio modes
        piano.play_keys(&[48]);
        
        piano.enable_audio();
        piano.play_keys(&[48]); // Should work with real audio
        
        piano.disable_audio(); 
        piano.play_keys(&[48]); // Should work in silent mode
    }
}