use std::thread;
use std::time::Duration;
use std::io::BufWriter;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::io::Cursor;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub trait AudioPlayer {
    fn play_piano_keys(&self, keys: &[usize]);
    fn play_chord(&self, keys: &[usize], duration_ms: u64);
    fn play_note_sequence(&self, keys: &[usize], note_duration_ms: u64, gap_ms: u64);
}

pub struct AudioEngine {
    _stream: OutputStream,
    sink: Sink,
    sample_cache: HashMap<usize, Vec<u8>>, // Cache for piano samples
}

pub struct NullAudioEngine;

impl AudioEngine {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|_| {
            println!("Warning: Could not initialize audio stream");
            OutputStream::try_default().expect("Failed to create fallback audio stream")
        });
        
        let sink = Sink::try_new(&stream_handle).unwrap_or_else(|_| {
            println!("Warning: Could not create audio sink");
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
        // Load available piano samples - comprehensive coverage across piano range
        // Piano key mapping: A0=0, A#0=1, B0=2, C1=3, C#1=4... A4=45, A#4=46, B4=47, C5=48...
        let sample_files = [
            // Low range
            (9, "samples/piano_a1.wav"),    // A1 
            (21, "samples/piano_a2.wav"),   // A2 (actually using this as A2)
            (24, "samples/piano_c2.wav"),   // C2
            
            // Mid-low range  
            (36, "samples/piano_c3.wav"),   // C3
            (36, "samples/piano_c3_kawai.wav"), // Alternative C3 (will overwrite, keeping best)
            
            // Mid range
            (48, "samples/piano_c4.wav"),   // C4 - Middle C
            (48, "samples/piano_c4_ivory.wav"), // Alternative C4 (better quality, will overwrite)
            (48, "samples/piano_c4_kawai.wav"), // Another C4 option
            
            // Upper-mid range
            (60, "samples/piano_c5.wav"),   // C5
            
            // High range
            (72, "samples/piano_c6.wav"),   // C6
            (84, "samples/piano_c7.wav"),   // C7
        ];

        for (key, file_path) in sample_files.iter() {
            if let Ok(mut file) = File::open(file_path) {
                let mut buffer = Vec::new();
                if std::io::Read::read_to_end(&mut file, &mut buffer).is_ok() {
                    self.sample_cache.insert(*key, buffer);
                    println!("Loaded sample for key {}: {}", key, file_path);
                } else {
                    println!("Failed to read sample file: {}", file_path);
                }
            } else {
                println!("Could not find sample file: {}", file_path);
            }
        }
    }

    fn get_sample_for_key(&self, key: usize) -> Option<&Vec<u8>> {
        // Find the closest available sample with intelligent selection
        let available_keys: Vec<usize> = self.sample_cache.keys().copied().collect();
        if available_keys.is_empty() {
            return None;
        }

        // Find the closest sample key, with preference for samples that don't require too much pitch shifting
        let closest_key = available_keys.iter()
            .min_by_key(|&&sample_key| {
                let distance = (sample_key as i32 - key as i32).abs();
                // Penalize large pitch shifts (more than 6 semitones) as they sound less natural
                if distance > 6 {
                    distance * 2 // Double the penalty for large shifts
                } else {
                    distance
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
                
                // Volume compensation: higher pitches need less volume, lower pitches need more
                let volume_compensation = if semitone_difference > 0.0 {
                    // Pitching up: reduce volume slightly
                    0.85_f32.powf(semitone_difference / 6.0)
                } else {
                    // Pitching down: increase volume slightly  
                    1.15_f32.powf(-semitone_difference / 6.0)
                };
                
                // Apply pitch shift, volume compensation, and play
                let adjusted_source = source
                    .speed(pitch_ratio)
                    .amplify(0.6 * volume_compensation);
                    
                self.sink.append(adjusted_source);
                
                // Debug info
                if (semitone_difference).abs() > 0.1 {
                    println!("Key {}: using sample {} (shift: {:.1} semitones, vol: {:.2})", 
                        key, closest_sample_key, semitone_difference, volume_compensation);
                }
            }
        } else {
            // Fallback to synthesis if no samples available
            println!("No samples available, falling back to synthesis for key {}", key);
            let frequency = self.piano_key_to_frequency(key);
            self.play_frequency(frequency, 500);
        }
    }

    fn piano_key_to_frequency(&self, key: usize) -> f64 {
        // Piano key 49 (A4) = 440 Hz
        // Formula: f = 440 * 2^((n-49)/12) where n is the piano key number (1-88)
        let piano_key = key + 1; // Convert from 0-based to 1-based
        let frequency = 440.0 * 2.0_f64.powf((piano_key as f64 - 49.0) / 12.0);
        
        // Debug output to verify frequencies
        #[cfg(debug_assertions)]
        {
            let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
            let octave = ((piano_key as i32 - 4) / 12) + 1;
            let note_in_octave = (piano_key as i32 - 4) % 12;
            let note_name = if note_in_octave >= 0 {
                note_names[note_in_octave as usize]
            } else {
                note_names[(12 + note_in_octave) as usize]
            };
            println!("Piano key {} = {}{} = {:.2} Hz", piano_key, note_name, octave, frequency);
        }
        
        frequency
    }

    fn is_chord_pattern(&self, keys: &[usize]) -> bool {
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

    fn generate_piano_wave(&self, frequency: f64, duration: f64) -> Vec<i16> {
        let sample_rate = 44100;
        let frames = (duration * sample_rate as f64) as usize;
        let mut samples = Vec::with_capacity(frames);
        
        // Calculate piano key number for register-specific characteristics
        let piano_key = (12.0 * (frequency / 440.0).log2() + 49.0) as i32;
        
        // Much more realistic register characteristics
        let (bass_factor, mid_factor, treble_factor, warmth, softness, noise_factor) = if piano_key <= 28 {
            // Deep bass: very warm, wooden, rich fundamentals
            (1.0, 0.3, 0.05, 0.95, 0.9, 0.02)
        } else if piano_key <= 40 {
            // Bass: warm and mellow with wooden resonance
            (0.9, 0.5, 0.1, 0.9, 0.8, 0.015)
        } else if piano_key <= 52 {
            // Lower mid: balanced but very warm
            (0.7, 0.8, 0.15, 0.85, 0.7, 0.01)
        } else if piano_key <= 64 {
            // Mid register: the "singing" range, very musical
            (0.5, 1.0, 0.2, 0.8, 0.6, 0.008)
        } else if piano_key <= 76 {
            // Upper mid: controlled brightness, still warm
            (0.3, 0.7, 0.4, 0.7, 0.5, 0.005)
        } else {
            // Treble: much softer, bell-like
            (0.1, 0.3, 0.6, 0.6, 0.4, 0.003)
        };
        
        for i in 0..frames {
            let t = i as f64 / sample_rate as f64;
            
            // Much softer, more realistic hammer strike with felt texture
            let hammer_strike = if t < 0.008 {
                let felt_attack = (t / 0.008).powf(0.3); // Very gentle attack curve
                let hammer_noise = noise_factor * ((t * 12000.0).sin() * 0.3 + (t * 8000.0).sin() * 0.2);
                softness * 0.15 * felt_attack * (1.0 - felt_attack) + hammer_noise
            } else {
                0.0
            };
            
            // Realistic inharmonicity based on actual piano physics
            let inharmonicity_coeff = if piano_key <= 28 {
                0.00002 * (frequency / 440.0) // Bass strings are longer, less inharmonic
            } else {
                0.000008 * (frequency / 440.0) // Treble strings more inharmonic but still subtle
            };
            
            // Much more realistic harmonic generation
            let fundamental = bass_factor * (2.0 * std::f64::consts::PI * frequency * t).sin();
            
            // Very subtle frequency modulation for organic warmth
            let vibrato = 1.0 + warmth * 0.0008 * (frequency * 0.006 * t).sin();
            let wobbled_freq = frequency * vibrato;
            
            // Softer, more musical harmonics
            let h2_freq = wobbled_freq * (2.0 + inharmonicity_coeff * 0.3);
            let second_harmonic = mid_factor * 0.25 * (2.0 * std::f64::consts::PI * h2_freq * t).sin();
            
            let h3_freq = wobbled_freq * (3.0 + inharmonicity_coeff * 0.5);
            let third_harmonic = warmth * 0.12 * (2.0 * std::f64::consts::PI * h3_freq * t).sin();
            
            let h4_freq = wobbled_freq * (4.0 + inharmonicity_coeff * 0.7);
            let fourth_harmonic = treble_factor * 0.06 * (2.0 * std::f64::consts::PI * h4_freq * t).sin();
            
            let h5_freq = wobbled_freq * (5.0 + inharmonicity_coeff * 0.9);
            let fifth_harmonic = treble_factor * 0.03 * (2.0 * std::f64::consts::PI * h5_freq * t).sin();
            
            // Add very subtle higher harmonics for warmth
            let h6_harmonic = treble_factor * 0.015 * (2.0 * std::f64::consts::PI * wobbled_freq * 6.0 * t).sin();
            let h7_harmonic = treble_factor * 0.008 * (2.0 * std::f64::consts::PI * wobbled_freq * 7.0 * t).sin();
            
            // Much more subtle string coupling and sympathetic resonance
            let coupling_strength = if piano_key <= 40 { 0.08 * warmth } else { 0.04 * warmth };
            let string_coupling = coupling_strength * 
                (2.0 * std::f64::consts::PI * frequency * 0.25 * t).sin() * 
                (2.0 * std::f64::consts::PI * frequency * 0.17 * t).cos();
            
            // More realistic soundboard resonance - much subtler
            let soundboard_resonance = warmth * 0.03 * 
                ((2.0 * std::f64::consts::PI * 85.0 * t).sin() * 0.5 + 
                 (2.0 * std::f64::consts::PI * 125.0 * t).sin() * 0.3 +
                 (2.0 * std::f64::consts::PI * 180.0 * t).sin() * 0.2 +
                 (2.0 * std::f64::consts::PI * 240.0 * t).sin() * 0.1);
            
            // Combine harmonics with much more natural blend
            let harmonic_sum = fundamental + 
                              second_harmonic * (1.0 - softness * 0.1) + 
                              third_harmonic * (1.0 - softness * 0.05) + 
                              fourth_harmonic * (1.0 - softness * 0.15) + 
                              fifth_harmonic * (1.0 - softness * 0.2) + 
                              h6_harmonic * (1.0 - softness * 0.25) +
                              h7_harmonic * (1.0 - softness * 0.3) +
                              string_coupling + soundboard_resonance + hammer_strike;
            
            // Much more realistic piano envelope with smooth transitions
            let envelope = if piano_key <= 28 {
                // Deep bass: wound strings, very slow, warm attack
                if t < 0.035 {
                    // Very gentle attack curve
                    let progress = t / 0.035;
                    progress.powf(0.4)  // Extremely gentle curve
                } else if t < 0.15 {
                    // Gentle transition to sustain
                    1.0 - 0.08 * ((t - 0.035) / 0.115).powf(0.7)
                } else if t < duration * 0.9 {
                    // Very long, warm sustain
                    0.92 * (-0.15 * (t - 0.15)).exp()
                } else {
                    // Gentle release
                    let release_progress = (t - duration * 0.9) / (duration * 0.1);
                    let sustain_level = 0.92 * (-0.15 * (duration * 0.9 - 0.15)).exp();
                    sustain_level * (1.0 - release_progress.powf(0.3))
                }
            } else if piano_key <= 52 {
                // Bass to mid: smooth, musical attack
                if t < 0.020 {
                    // Smooth attack
                    let progress = t / 0.020;
                    progress.powf(0.6)
                } else if t < 0.08 {
                    // Gentle initial decay
                    1.0 - 0.12 * ((t - 0.020) / 0.060).powf(0.8)
                } else if t < duration * 0.8 {
                    // Natural sustain
                    0.88 * (-0.25 * (t - 0.08)).exp()
                } else {
                    // Smooth release
                    let release_progress = (t - duration * 0.8) / (duration * 0.2);
                    let sustain_level = 0.88 * (-0.25 * (duration * 0.8 - 0.08)).exp();
                    sustain_level * (1.0 - release_progress.powf(0.5))
                }
            } else if piano_key <= 76 {
                // Mid to upper: balanced, smooth attack
                if t < 0.012 {
                    // Quick but smooth attack
                    let progress = t / 0.012;
                    progress.powf(0.8)
                } else if t < 0.05 {
                    // Quick but gentle decay
                    1.0 - 0.18 * ((t - 0.012) / 0.038).powf(0.9)
                } else if t < duration * 0.7 {
                    // Moderate sustain
                    0.82 * (-0.4 * (t - 0.05)).exp()
                } else {
                    // Natural release
                    let release_progress = (t - duration * 0.7) / (duration * 0.3);
                    let sustain_level = 0.82 * (-0.4 * (duration * 0.7 - 0.05)).exp();
                    sustain_level * (1.0 - release_progress.powf(0.7))
                }
            } else {
                // Treble: quick attack, but still musical
                if t < 0.006 {
                    // Fast but smooth attack
                    let progress = t / 0.006;
                    progress.powf(1.0)
                } else if t < 0.025 {
                    // Quick decay but not harsh
                    1.0 - 0.25 * ((t - 0.006) / 0.019).powf(1.1)
                } else if t < duration * 0.5 {
                    // Short but musical sustain
                    0.75 * (-0.8 * (t - 0.025)).exp()
                } else {
                    // Quick but smooth release
                    let release_progress = (t - duration * 0.5) / (duration * 0.5);
                    let sustain_level = 0.75 * (-0.8 * (duration * 0.5 - 0.025)).exp();
                    sustain_level * (1.0 - release_progress.powf(1.5))
                }
            };
            
            // Add slight envelope modulation for more organic sound
            let envelope_modulation = 1.0 + 0.02 * (frequency * 0.003 * t).sin();
            let final_envelope = envelope * envelope_modulation;
            
            // Apply envelope to harmonic content
            let final_sample = harmonic_sum * final_envelope;
            
            // Piano-specific volume scaling based on actual piano acoustics
            let volume_scale = if piano_key <= 21 {
                // Sub-bass (A0-A1): very quiet, need significant boost
                15000.0
            } else if piano_key <= 28 {
                // Deep bass (A#1-E2): still quiet, boost needed  
                13000.0
            } else if piano_key <= 40 {
                // Bass (F2-E3): moderate volume
                12000.0
            } else if piano_key <= 52 {
                // Lower mid (F3-E4): good projection
                11500.0
            } else if piano_key <= 64 {
                // Mid register (F4-E5): strongest projection, "singing" range
                12500.0
            } else if piano_key <= 76 {
                // Upper mid (F5-E6): bright but controlled
                11000.0
            } else if piano_key <= 84 {
                // Treble (F#6-C7): naturally quieter due to short strings
                9500.0
            } else {
                // Extreme treble (C#7-C8): very quiet, delicate
                8000.0
            };
            
            // Add very subtle room tone and tape-like saturation
            let room_ambiance = 0.002 * ((t * 60.0).sin() + (t * 97.0).sin() + (t * 133.0).sin());
            
            // Gentle saturation for warmth
            let saturated_sample = final_sample + room_ambiance;
            let warm_sample = if saturated_sample.abs() > 0.7 {
                saturated_sample.signum() * (0.7 + 0.3 * (saturated_sample.abs() - 0.7).tanh())
            } else {
                saturated_sample
            };
            
            samples.push((warm_sample * volume_scale).clamp(-32767.0, 32767.0) as i16);
        }
        
        samples
    }

    fn play_frequency(&self, frequency: f64, duration_ms: u64) {
        let samples = self.generate_piano_wave(frequency, duration_ms as f64 / 1000.0);
        
        // Create a WAV buffer in memory
        let mut wav_buffer = Vec::new();
        {
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: 44100,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            
            let mut writer = hound::WavWriter::new(Cursor::new(&mut wav_buffer), spec).unwrap();
            for sample in samples {
                writer.write_sample(sample).unwrap();
            }
            writer.finalize().unwrap();
        }
        
        // Play the WAV buffer
        let cursor = Cursor::new(wav_buffer);
        if let Ok(source) = Decoder::new(cursor) {
            self.sink.append(source);
        }
    }
}

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

    fn play_note_sequence(&self, keys: &[usize], note_duration_ms: u64, gap_ms: u64) {
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

    fn play_note_sequence(&self, _keys: &[usize], _note_duration_ms: u64, _gap_ms: u64) {
        // Do nothing - null object pattern
    }
}