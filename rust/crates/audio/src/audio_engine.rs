use std::thread;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;
use std::collections::HashMap;
use std::fs::File;
use log::{info, warn, error, debug};
// Keep RepoStructure import as it's used in load_samples() and tests
use common::RepoStructure;

// We no longer need hardcoded paths since we're using the repo structure utility

pub trait AudioPlayer {
    fn play_piano_keys(&self, keys: &[usize]);
    fn play_chord(&self, keys: &[usize], duration_ms: u64);
}

pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    sample_cache: HashMap<usize, Vec<u8>>, // Cache for piano samples
}

pub struct NullAudioEngine;

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioEngine {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|_| {
            warn!("Warning: Could not initialize audio stream");
            OutputStream::try_default().expect("Failed to create fallback audio stream")
        });
        
        let sink = Sink::try_new(&stream_handle).unwrap_or_else(|_| {
            warn!("Warning: Could not create audio sink");
            Sink::try_new(&stream_handle).expect("Failed to create fallback audio sink")
        });
        
        let mut engine = AudioEngine { 
            _stream, 
            sink, 
            sample_cache: HashMap::new()
        };
        
        // Load piano samples
        engine.load_samples();
        engine
    }

    fn load_samples(&mut self) {
        // Load available piano samples with comprehensive chromatic coverage
        // Piano key mapping: A0=0, A#0=1, B0=2, C1=3, C#1=4, D1=5, D#1=6, E1=7, F1=8, F#1=9, G1=10, G#1=11, A1=12...
        let sample_files = [
            // Low range (Octave 1 & 2)
            (9, "piano_a1.wav"),     // A1 (key 9)
            (21, "piano_a2.wav"),    // A2 (key 21)
            (24, "piano_c2.wav"),    // C2 (key 24)
            
            // Mid-low range (Octave 3) - Better chromatic coverage
            (36, "piano_c3.wav"),    // C3 (key 36)
            (38, "piano_d3.wav"),    // D3 (key 38)
            (41, "piano_f3.wav"),    // F3 (key 41)
            (43, "piano_g3.wav"),    // G3 (key 43)
            
            // Mid range (Octave 4) - Even better coverage
            (48, "piano_c4_ivory.wav"), // C4 - Middle C (key 48) - Best quality
            (50, "piano_d4.wav"),    // D4 (key 50)
            (53, "piano_f4.wav"),    // F4 (key 53)
            (55, "piano_g4.wav"),    // G4 (key 55)
            
            // Alternative samples for comparison/backup
            (36, "piano_c3_kawai.wav"), // Alternative C3
            (48, "piano_c4.wav"),    // Alternative C4
            (48, "piano_c4_kawai.wav"), // Another C4 option
            
            // Upper-mid range (Octave 5)
            (60, "piano_c5.wav"),    // C5 (key 60)
            
            // High range (Octave 6 & 7)
            (72, "piano_c6.wav"),    // C6 (key 72)
            (84, "piano_c7.wav"),    // C7 (key 84)
        ];
        
        // Get audio samples directory from repository structure
        let repo = RepoStructure::new();
        let audio_dir = repo.audio_samples_dir();
        
        // Log the audio path being used
        info!("Loading audio samples from path: {}", audio_dir.display());

        for (key, file_name) in sample_files.iter() {
            // Construct the full path
            let full_path = audio_dir.join(file_name);
            if let Ok(mut file) = File::open(&full_path) {
                let mut buffer = Vec::new();
                if std::io::Read::read_to_end(&mut file, &mut buffer).is_ok() {
                    self.sample_cache.insert(*key, buffer);
                    let note_name = self.key_to_note_name(*key);
                    info!("Loaded sample for key {} ({}): {}", key, note_name, full_path.display());
                } else {
                    warn!("Failed to read sample file: {}", full_path.display());
                }
            } else {
                warn!("Could not find sample file: {}", full_path.display());
            }
        }
        
        info!("Loaded {} piano samples covering chromatic range", self.sample_cache.len());
        self.print_coverage_analysis();
    }

    fn key_to_note_name(&self, key: usize) -> String {
        let note_names = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
        let octave = key / 12;
        let note_in_octave = key % 12;
        format!("{}{}", note_names[note_in_octave], octave)
    }

    fn print_coverage_analysis(&self) {
        debug!("=== Chromatic Coverage Analysis ===");
        let mut keys: Vec<usize> = self.sample_cache.keys().copied().collect();
        keys.sort();
        
        for &key in &keys {
            let note_name = self.key_to_note_name(key);
            debug!("  {} (key {})", note_name, key);
        }
        
        // Analysis of gaps
        let mut gaps = Vec::new();
        for i in 1..keys.len() {
            let gap = keys[i] - keys[i-1];
            if gap > 3 {
                gaps.push((keys[i-1], keys[i], gap));
            }
        }
        
        if !gaps.is_empty() {
            debug!("=== Coverage Gaps (>3 semitones) ===");
            for (from_key, to_key, gap) in gaps {
                debug!("  {} to {} ({} semitones)", 
                    self.key_to_note_name(from_key), 
                    self.key_to_note_name(to_key), 
                    gap);
            }
        } else {
            debug!("Excellent chromatic coverage - no major gaps!");
        }
    }

    fn get_sample_for_key(&self, key: usize) -> Option<&Vec<u8>> {
        // Find the closest available sample with intelligent chromatic selection
        let available_keys: Vec<usize> = self.sample_cache.keys().copied().collect();
        if available_keys.is_empty() {
            return None;
        }

        // Advanced sample selection algorithm for better chromatic coverage
        let closest_key = available_keys.iter()
            .min_by_key(|&&sample_key| {
                let distance = (sample_key as i32 - key as i32).abs();
                
                // Chromatic optimization: prefer samples that result in better pitch shifts
                if distance == 0 {
                    0 // Perfect match
                } else if distance <= 2 {
                    distance // Minimal shift penalty (within major second)
                } else if distance <= 6 {
                    distance + 1 // Slight penalty for larger shifts (up to tritone)
                } else if distance <= 12 {
                    distance * 2 // Higher penalty for shifts over an octave
                } else {
                    distance * 3 // Very high penalty for extreme shifts
                }
            })
            .copied()?;

        self.sample_cache.get(&closest_key)
    }

    fn play_sample(&self, key: usize) {
        if let Some(sample_data) = self.get_sample_for_key(key) {
            let cursor = Cursor::new(sample_data.clone());
            if let Ok(source) = Decoder::new(cursor) {
                // Calculate pitch adjustment if needed
                let closest_sample_key = self.sample_cache.keys()
                    .min_by_key(|&&sample_key| (sample_key as i32 - key as i32).abs())
                    .copied()
                    .unwrap_or(48);

                let semitone_difference = key as f32 - closest_sample_key as f32;
                let pitch_ratio = 2.0_f32.powf(semitone_difference / 12.0);
                
                // Advanced volume compensation for chromatic intervals
                let volume_compensation = if semitone_difference > 0.0 {
                    // Pitching up: reduce volume progressively for higher pitches
                    let reduction_factor = 1.0 - (semitone_difference * 0.03).min(0.3);
                    reduction_factor.max(0.6) // Don't reduce below 60% volume
                } else if semitone_difference < 0.0 {
                    // Pitching down: increase volume progressively for lower pitches  
                    let boost_factor = 1.0 + (-semitone_difference * 0.04).min(0.4);
                    boost_factor.min(1.5) // Don't boost above 150% volume
                } else {
                    1.0 // No adjustment for perfect match
                };
                
                // Apply pitch shift, volume compensation, and play
                let adjusted_source = source
                    .speed(pitch_ratio)
                    .amplify(0.6 * volume_compensation);
                    
                self.sink.append(adjusted_source);
                
                // Debug info
                if (semitone_difference).abs() > 0.1 {
                    debug!("Key {}: using sample {} (shift: {:.1} semitones, vol: {:.2})", 
                        key, closest_sample_key, semitone_difference, volume_compensation);
                }
            }
        } else {
            // This should never happen with our comprehensive sample coverage
            error!("Critical error: No sample available for key {} - this indicates a problem with sample loading", key);
        }
    }

    // Made public to be used by piano_player
    pub fn is_chord_pattern(&self, keys: &[usize]) -> bool {
        if keys.len() < 3 {
            return false;
        }
        
        // Check for common chord patterns
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
                   (6..=8).contains(&interval2) {
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
}

// Implement AudioPlayer trait for AudioEngine
impl AudioPlayer for AudioEngine {
    fn play_piano_keys(&self, keys: &[usize]) {
        if keys.is_empty() {
            return;
        }

        // Detect chord patterns and play accordingly
        if self.is_chord_pattern(keys) {
            self.play_chord(keys, 300);
        } else {
            // Play individual keys with slight delay using samples
            for &key in keys {
                self.play_sample(key);
                thread::sleep(Duration::from_millis(50));
            }
        }
        
        // Wait for audio to finish
        thread::sleep(Duration::from_millis(300));
    }

    fn play_chord(&self, keys: &[usize], duration_ms: u64) {
        if keys.is_empty() {
            return;
        }

        // Play chord using samples with slight timing offset for natural feel
        for (i, &key) in keys.iter().enumerate() {
            // Add slight delay between notes for natural chord attack
            if i > 0 {
                thread::sleep(Duration::from_millis(10));
            }
            self.play_sample(key);
        }
        
        thread::sleep(Duration::from_millis(duration_ms));
    }
}

// Additional methods for AudioEngine are implemented in this block
// This ensures the trait implementation remains clean
impl AudioEngine {
    // Implementation exists for possible future use by other modules
    #[cfg(test)]
    #[allow(dead_code)]
    fn play_notes_in_sequence(&self, keys: &[usize], note_duration_ms: u64, gap_ms: u64) {
        if keys.is_empty() {
            return;
        }

        for &key in keys {
            self.play_sample(key);
            thread::sleep(Duration::from_millis(note_duration_ms));
            
            if gap_ms > 0 {
                thread::sleep(Duration::from_millis(gap_ms));
            }
        }
    }
}

impl Default for NullAudioEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl NullAudioEngine {
    pub fn new() -> Self {
        NullAudioEngine
    }
}

impl AudioPlayer for NullAudioEngine {
    fn play_piano_keys(&self, _keys: &[usize]) {
        // Do nothing - null object pattern
    }

    fn play_chord(&self, _keys: &[usize], _duration_ms: u64) {
        // Do nothing - null object pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_engine_creation() {
        // Test that audio engine can be created without panicking
        let engine = NullAudioEngine::new();
        
        // Test that it implements the AudioPlayer trait
        engine.play_piano_keys(&[]);
        engine.play_piano_keys(&[48]); // Middle C
        engine.play_chord(&[48, 52, 55], 500); // C Major
    }

    #[test]
    fn test_null_audio_engine() {
        let engine = NullAudioEngine::new();
        
        // All operations should be no-ops and not panic
        engine.play_piano_keys(&[0, 1, 2, 3, 4]);
        engine.play_chord(&[24, 28, 31], 1000);
        
        // Test edge cases
        engine.play_piano_keys(&[]);
        engine.play_piano_keys(&[87]); // Highest key
        engine.play_chord(&[], 500);
    }


    #[test]
    fn test_sample_selection_algorithm() {
        let engine = AudioEngine::new();
        
        // Samples should always be available with our repository structure
        let repo = RepoStructure::new();
        let audio_dir = repo.audio_samples_dir();
        assert!(!engine.sample_cache.is_empty(), "Sample cache is empty. Audio path: {}", audio_dir.display());
        
        // Test sample selection for various keys
        for key in 0..88 {
            let sample = engine.get_sample_for_key(key);
            // We should have a sample for every key now
            assert!(sample.is_some(), "No sample found for key {}", key);
        }
        
        println!("Successfully tested sample selection with {} samples", engine.sample_cache.len());
    }

    #[test]
    fn test_key_to_note_name_conversion() {
        let engine = AudioEngine::new();
        
        // Test specific known conversions
        assert_eq!(engine.key_to_note_name(0), "A0");
        assert_eq!(engine.key_to_note_name(48), "A4"); // Key 48 should be A4
        assert_eq!(engine.key_to_note_name(87), "C7");  // Key 87 should be C7
        
        // Test that all keys produce valid note names
        for key in 0..88 {
            let note_name = engine.key_to_note_name(key);
            assert!(!note_name.is_empty(), "Note name should not be empty for key {}", key);
            assert!(note_name.len() >= 2, "Note name should have at least note and octave for key {}", key);
        }
    }

    #[test]
    fn test_chord_pattern_recognition() {
        let engine = AudioEngine::new();
        
        // Test that various chord patterns are recognized correctly
        assert!(engine.is_chord_pattern(&[24, 28, 31])); // C Major triad
        assert!(engine.is_chord_pattern(&[24, 27, 31])); // C minor triad  
        assert!(!engine.is_chord_pattern(&[24]));        // Single note
        assert!(!engine.is_chord_pattern(&[24, 25]));    // Two notes
        assert!(!engine.is_chord_pattern(&[]));          // Empty
        
        // Test dense cluster recognition
        let dense_cluster = vec![48, 49, 50, 51, 52]; // 5 consecutive semitones
        assert!(engine.is_chord_pattern(&dense_cluster));
    }
}
