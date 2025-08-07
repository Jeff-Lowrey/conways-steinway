use crate::audio::{AudioPlayer, AudioEngine, NullAudioEngine};

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
            println!("♪ Silence");
            return;
        }

        // Check if this looks like a chord pattern
        let is_chord = self.is_chord_pattern(keys);
        
        if is_chord {
            print!("♫ Playing chord: ");
        } else {
            print!("♪ Playing piano keys: ");
        }
        
        for (i, &key) in keys.iter().enumerate() {
            if i > 0 { print!(", "); }
            print!("{}", key + 1);
        }
        println!();

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