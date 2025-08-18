// Conway's Steinway Configuration Types
//
// This file contains the core configuration types and their implementations.

use clap::{Arg, ArgAction, Command, ValueHint};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::collections::HashMap;
use java_properties;
use log::{info, warn, error, debug};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub board_type: BoardType,
    #[serde(alias = "silent")]
    pub audio_enabled: bool,
    pub generations: GenerationLimit,
    pub step_delay_ms: u64,
    pub tempo_bpm: Option<f64>,
    pub config_file: Option<PathBuf>,
    
    // Audio settings
    #[serde(default = "default_note_duration")]
    pub note_duration_ms: u64,
    #[serde(default = "default_gap_ms")]
    pub gap_ms: u64,
    #[serde(default = "default_chord_duration")]
    pub chord_duration_ms: u64,
    #[serde(default = "default_initial_delay")]
    pub initial_delay_ms: u64,
    #[serde(default = "default_detect_chords")]
    pub detect_chords: bool,
    #[serde(default = "default_volume")]
    pub volume: f32,
    #[serde(default = "default_pitch_shift")]
    pub pitch_shift: bool,
    
    // Random board settings
    #[serde(default = "default_alive_probability")]
    pub alive_probability: f32,
    
    // Board dimensions (fixed)
    pub board_height: Option<usize>,
    
    // Logging configuration
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    // Multi-destination logging settings
    #[serde(default = "default_log_to_file")]
    pub log_to_file: bool,
    #[serde(default = "default_log_file_path")]
    pub log_file_path: Option<PathBuf>,
    #[serde(default = "default_log_file_level")]
    pub log_file_level: String,
    #[serde(default = "default_log_console_level")]
    pub log_console_level: String,
    #[serde(default = "default_log_file_rotation")]
    pub log_file_rotation: bool,
    #[serde(default = "default_log_file_size_limit")]
    pub log_file_size_limit: u64,
    #[serde(default = "default_log_file_count")]
    pub log_file_count: u32,
}

// Default functions for optional fields
fn default_note_duration() -> u64 { 200 }
fn default_gap_ms() -> u64 { 50 }
fn default_chord_duration() -> u64 { 300 }
fn default_initial_delay() -> u64 { 50 }
fn default_detect_chords() -> bool { true }
fn default_volume() -> f32 { 0.6 }
fn default_pitch_shift() -> bool { true }
fn default_alive_probability() -> f32 { 0.2 }
fn default_log_level() -> String { "info".to_string() }
fn default_log_to_file() -> bool { false }
fn default_log_file_path() -> Option<PathBuf> { None }
fn default_log_file_level() -> String { "debug".to_string() }
fn default_log_console_level() -> String { "info".to_string() }
fn default_log_file_rotation() -> bool { true }
fn default_log_file_size_limit() -> u64 { 10 * 1024 * 1024 } // 10 MB
fn default_log_file_count() -> u32 { 5 }

// Valid log levels that can be used
pub const VALID_LOG_LEVELS: [&str; 5] = ["trace", "debug", "info", "warn", "error"];

// Default log file path
pub const DEFAULT_LOG_DIR: &str = "logs";
pub const DEFAULT_LOG_FILE: &str = "conways_steinway.log";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardType {
    Random,
    Static,
    FurElise,
    Complex,
    Showcase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationLimit {
    Limited(u32),
    Unlimited,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            board_type: BoardType::Random,
            audio_enabled: true,
            generations: GenerationLimit::Unlimited,
            step_delay_ms: 200,
            tempo_bpm: None, // Will be set based on board type
            config_file: None,
            
            // Audio settings
            note_duration_ms: default_note_duration(),
            gap_ms: default_gap_ms(),
            chord_duration_ms: default_chord_duration(),
            initial_delay_ms: default_initial_delay(),
            detect_chords: default_detect_chords(),
            volume: default_volume(),
            pitch_shift: default_pitch_shift(),
            
            // Random board settings
            alive_probability: default_alive_probability(),
            
            // Board dimensions (fixed)
            board_height: Some(40),
            
            // Logging configuration
            log_level: default_log_level(),
            log_to_file: default_log_to_file(),
            log_file_path: default_log_file_path(),
            log_file_level: default_log_file_level(),
            log_console_level: default_log_console_level(),
            log_file_rotation: default_log_file_rotation(),
            log_file_size_limit: default_log_file_size_limit(),
            log_file_count: default_log_file_count(),
        }
    }
}

impl Config {
    pub fn from_args_and_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Config::default();
        
        // Load from environment variables first
        config.load_from_env()?;
        
        // Parse command line arguments
        let app = Command::new("Conway's Steinway")
            .version("0.1.0")
            .about("A musical interpretation of Conway's Game of Life using piano sounds")
            .arg(Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
                .value_hint(ValueHint::FilePath))
            .arg(Arg::new("board-type")
                .short('b')
                .long("board-type")
                .value_name("TYPE")
                .help("Board initialization type")
                .value_parser(["random", "static", "fur_elise", "complex", "showcase"])
                .env("CONWAYS_STEINWAY_BOARD_TYPE"))
            .arg(Arg::new("silent")
                .short('s')
                .long("silent")
                .help("Disable audio output")
                .action(ArgAction::SetTrue)
                .env("CONWAYS_STEINWAY_SILENT"))
            // Remove the --audio flag since audio is now the default and we only check for --silent
            .arg(Arg::new("generations")
                .short('g')
                .long("generations")
                .value_name("COUNT")
                .help("Number of generations to run (0 for unlimited)")
                .value_parser(clap::value_parser!(u32))
                .env("CONWAYS_STEINWAY_GENERATIONS"))
            .arg(Arg::new("step-delay")
                .short('d')
                .long("step-delay")
                .value_name("MILLISECONDS")
                .help("Delay between steps in milliseconds")
                .value_parser(clap::value_parser!(u64))
                .env("CONWAYS_STEINWAY_STEP_DELAY"))
            .arg(Arg::new("tempo")
                .short('t')
                .long("tempo")
                .value_name("BPM")
                .help("Musical tempo in beats per minute (overrides delay)")
                .value_parser(clap::value_parser!(f64))
                .env("CONWAYS_STEINWAY_TEMPO"))
            // Audio settings
            .arg(Arg::new("note-duration")
                .long("note-duration")
                .value_name("MILLISECONDS")
                .help("Duration of individual notes in milliseconds")
                .value_parser(clap::value_parser!(u64))
                .env("CONWAYS_STEINWAY_NOTE_DURATION"))
            .arg(Arg::new("gap")
                .long("gap")
                .value_name("MILLISECONDS")
                .help("Gap between notes in milliseconds")
                .value_parser(clap::value_parser!(u64))
                .env("CONWAYS_STEINWAY_GAP"))
            .arg(Arg::new("chord-duration")
                .long("chord-duration")
                .value_name("MILLISECONDS")
                .help("Duration of chords in milliseconds")
                .value_parser(clap::value_parser!(u64))
                .env("CONWAYS_STEINWAY_CHORD_DURATION"))
            .arg(Arg::new("initial-delay")
                .long("initial-delay")
                .value_name("MILLISECONDS")
                .help("Initial delay between notes in milliseconds")
                .value_parser(clap::value_parser!(u64))
                .env("CONWAYS_STEINWAY_INITIAL_DELAY"))
            // Remove --detect-chords flag since it's now the default and we only check for --no-detect-chords
            .arg(Arg::new("no-detect-chords")
                .long("no-detect-chords")
                .help("Disable automatic chord detection")
                .action(ArgAction::SetTrue))
            .arg(Arg::new("volume")
                .long("volume")
                .value_name("LEVEL")
                .help("Audio volume (0.0-1.0)")
                .value_parser(clap::value_parser!(f32))
                .env("CONWAYS_STEINWAY_VOLUME"))
            // Remove --pitch-shift flag since it's now the default and we only check for --no-pitch-shift
            .arg(Arg::new("no-pitch-shift")
                .long("no-pitch-shift")
                .help("Disable pitch shifting")
                .action(ArgAction::SetTrue))
            // Random board settings
            .arg(Arg::new("alive-probability")
                .long("alive-probability")
                .value_name("PROBABILITY")
                .help("Probability of cells being alive in random boards (0.0-1.0)")
                .value_parser(clap::value_parser!(f32))
                .env("CONWAYS_STEINWAY_ALIVE_PROBABILITY"))
            // Board dimensions
            .arg(Arg::new("height")
                .long("height")
                .value_name("CELLS")
                .help("Board height in cells")
                .value_parser(clap::value_parser!(usize))
                .env("CONWAYS_STEINWAY_BOARD_HEIGHT"))
            .arg(Arg::new("log-level")
                .long("log-level")
                .value_name("LEVEL")
                .help("Log level (trace, debug, info, warn, error)")
                .value_parser(["trace", "debug", "info", "warn", "error"])
                .env("RUST_LOG"))
            .arg(Arg::new("log-to-file")
                .long("log-to-file")
                .help("Enable logging to file")
                .action(ArgAction::SetTrue)
                .env("CONWAYS_STEINWAY_LOG_TO_FILE"))
            .arg(Arg::new("log-file-path")
                .long("log-file-path")
                .value_name("PATH")
                .help("Path to log file")
                .value_hint(ValueHint::FilePath)
                .env("CONWAYS_STEINWAY_LOG_FILE_PATH"))
            .arg(Arg::new("log-file-level")
                .long("log-file-level")
                .value_name("LEVEL")
                .help("Log level for file output (trace, debug, info, warn, error)")
                .value_parser(["trace", "debug", "info", "warn", "error"])
                .env("CONWAYS_STEINWAY_LOG_FILE_LEVEL"))
            .arg(Arg::new("log-console-level")
                .long("log-console-level")
                .value_name("LEVEL")
                .help("Log level for console output (trace, debug, info, warn, error)")
                .value_parser(["trace", "debug", "info", "warn", "error"])
                .env("CONWAYS_STEINWAY_LOG_CONSOLE_LEVEL"))
            .arg(Arg::new("no-log-file-rotation")
                .long("no-log-file-rotation")
                .help("Disable log file rotation")
                .action(ArgAction::SetTrue)
                .env("CONWAYS_STEINWAY_NO_LOG_FILE_ROTATION"))
            .arg(Arg::new("log-file-size-limit")
                .long("log-file-size-limit")
                .value_name("SIZE_MB")
                .help("Size limit for log files in megabytes")
                .value_parser(clap::value_parser!(u64))
                .env("CONWAYS_STEINWAY_LOG_FILE_SIZE_LIMIT"))
            .arg(Arg::new("log-file-count")
                .long("log-file-count")
                .value_name("COUNT")
                .help("Number of rotated log files to keep")
                .value_parser(clap::value_parser!(u32))
                .env("CONWAYS_STEINWAY_LOG_FILE_COUNT"));

        let matches = app.get_matches();

        // Load from config file if specified
        if let Some(config_path) = matches.get_one::<String>("config") {
            let path = PathBuf::from(config_path);
            config.config_file = Some(path.clone());
            config.load_from_file(&path)?;
        }

        // Override with command line arguments
        if let Some(board_type) = matches.get_one::<String>("board-type") {
            config.board_type = match board_type.as_str() {
                "static" => BoardType::Static,
                "fur_elise" => BoardType::FurElise,
                "complex" => BoardType::Complex,
                "showcase" => BoardType::Showcase,
                _ => BoardType::Random,
            };
        }

        // Audio is enabled by default (audio_enabled=true)
        // Set audio_enabled=false if the --silent flag is present
        if matches.get_flag("silent") {
            config.audio_enabled = false;
        }

        if let Some(&generations) = matches.get_one::<u32>("generations") {
            config.generations = if generations == 0 {
                GenerationLimit::Unlimited
            } else {
                GenerationLimit::Limited(generations)
            };
        }

        if let Some(&delay) = matches.get_one::<u64>("step-delay") {
            config.step_delay_ms = delay;
        }

        if let Some(&tempo) = matches.get_one::<f64>("tempo") {
            config.tempo_bpm = Some(tempo);
        }

        // Audio settings from command line
        if let Some(&note_duration) = matches.get_one::<u64>("note-duration") {
            config.note_duration_ms = note_duration;
        }
        
        if let Some(&gap) = matches.get_one::<u64>("gap") {
            config.gap_ms = gap;
        }
        
        if let Some(&chord_duration) = matches.get_one::<u64>("chord-duration") {
            config.chord_duration_ms = chord_duration;
        }
        
        if let Some(&initial_delay) = matches.get_one::<u64>("initial-delay") {
            config.initial_delay_ms = initial_delay;
        }
        
        // Chord detection is enabled by default (detect_chords=true)
        // Only set detect_chords=false if the --no-detect-chords flag is present
        if matches.get_flag("no-detect-chords") {
            config.detect_chords = false;
        }
        
        if let Some(&volume) = matches.get_one::<f32>("volume") {
            config.volume = volume;
        }
        
        // Pitch shifting is enabled by default (pitch_shift=true)
        // Only set pitch_shift=false if the --no-pitch-shift flag is present
        if matches.get_flag("no-pitch-shift") {
            config.pitch_shift = false;
        }
        
        // Random board settings from command line
        if let Some(&alive_probability) = matches.get_one::<f32>("alive-probability") {
            config.alive_probability = alive_probability;
        }
        
        // Board dimensions from command line
        if let Some(&height) = matches.get_one::<usize>("height") {
            config.board_height = Some(height);
        }
        
        // Logging configuration
        if let Some(log_level) = matches.get_one::<String>("log-level") {
            // No need to validate here since we've already restricted the input with value_parser
            config.log_level = log_level.to_string();
        }
        
        if matches.get_flag("log-to-file") {
            config.log_to_file = true;
        }
        
        if let Some(log_file_path) = matches.get_one::<String>("log-file-path") {
            config.log_file_path = Some(PathBuf::from(log_file_path));
        }
        
        if let Some(log_file_level) = matches.get_one::<String>("log-file-level") {
            config.log_file_level = log_file_level.to_string();
        }
        
        if let Some(log_console_level) = matches.get_one::<String>("log-console-level") {
            config.log_console_level = log_console_level.to_string();
        }
        
        if matches.get_flag("no-log-file-rotation") {
            config.log_file_rotation = false;
        }
        
        if let Some(&size_mb) = matches.get_one::<u64>("log-file-size-limit") {
            config.log_file_size_limit = size_mb * 1024 * 1024; // Convert MB to bytes
        }
        
        if let Some(&count) = matches.get_one::<u32>("log-file-count") {
            config.log_file_count = count;
        }

        Ok(config)
    }

    pub fn load_from_env(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Environment variables are handled by clap with .env() calls
        // This method is kept for potential future custom env var handling
        Ok(())
    }

    pub fn load_from_file(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if path.exists() {
            // First check if 'silent' key exists in raw file
            let contents = fs::read_to_string(path)?;
            let is_silent = contents.contains("silent") || contents.contains("audio.enabled=false");
            self.audio_enabled = !is_silent;
            
            // Parse the properties file
            let properties = Self::parse_properties_file(path)?;
            
            // Apply core configuration values
            if let Some(board_type) = properties.get("board.type") {
                self.board_type = match board_type.to_lowercase().as_str() {
                    "static" => BoardType::Static,
                    "fur_elise" => BoardType::FurElise,
                    "complex" => BoardType::Complex,
                    "showcase" => BoardType::Showcase,
                    _ => BoardType::Random,
                };
            }
            
            // Check for audio.enabled setting
            if let Some(audio_enabled) = properties.get("audio.enabled") {
                self.audio_enabled = audio_enabled.to_lowercase() == "true";
            }
            
            // Parse generations
            if let Some(generations_str) = properties.get("generations") {
                if generations_str.to_lowercase() == "unlimited" {
                    self.generations = GenerationLimit::Unlimited;
                } else if let Ok(num) = generations_str.parse::<u32>() {
                    self.generations = if num == 0 {
                        GenerationLimit::Unlimited
                    } else {
                        GenerationLimit::Limited(num)
                    };
                }
            }
            
            // Parse step delay
            if let Some(delay_str) = properties.get("step.delay.ms") {
                if let Ok(delay) = delay_str.parse::<u64>() {
                    self.step_delay_ms = delay;
                }
            }
            
            // Parse tempo
            if let Some(tempo_str) = properties.get("tempo.bpm") {
                if let Ok(tempo) = tempo_str.parse::<f64>() {
                    self.tempo_bpm = Some(tempo);
                }
            }
            
            // Parse audio settings
            if let Some(note_duration_str) = properties.get("audio.note.duration.ms") {
                if let Ok(duration) = note_duration_str.parse::<u64>() {
                    self.note_duration_ms = duration;
                }
            }
            
            if let Some(gap_str) = properties.get("audio.gap.ms") {
                if let Ok(gap) = gap_str.parse::<u64>() {
                    self.gap_ms = gap;
                }
            }
            
            if let Some(chord_duration_str) = properties.get("audio.chord.duration.ms") {
                if let Ok(duration) = chord_duration_str.parse::<u64>() {
                    self.chord_duration_ms = duration;
                }
            }
            
            if let Some(initial_delay_str) = properties.get("audio.initial.delay.ms") {
                if let Ok(delay) = initial_delay_str.parse::<u64>() {
                    self.initial_delay_ms = delay;
                }
            }
            
            if let Some(detect_chords_str) = properties.get("audio.detect.chords") {
                let value = detect_chords_str.to_lowercase();
                self.detect_chords = value == "true" || value == "yes" || value == "on" || value == "1";
            }
            
            if let Some(volume_str) = properties.get("audio.volume") {
                if let Ok(volume) = volume_str.parse::<f32>() {
                    self.volume = volume;
                }
            } else if let Some(volume_str) = properties.get("volume") {
                if let Ok(volume) = volume_str.parse::<f32>() {
                    self.volume = volume;
                }
            }
            
            if let Some(pitch_shift_str) = properties.get("audio.pitch.shift") {
                let value = pitch_shift_str.to_lowercase();
                self.pitch_shift = value == "true" || value == "yes" || value == "on" || value == "1";
            } else if let Some(pitch_shift_str) = properties.get("pitch.shift") {
                let value = pitch_shift_str.to_lowercase();
                self.pitch_shift = value == "true" || value == "yes" || value == "on" || value == "1";
            }
            
            // Parse random board settings
            if let Some(alive_prob_str) = properties.get("random.alive.probability") {
                if let Ok(prob) = alive_prob_str.parse::<f32>() {
                    self.alive_probability = prob;
                }
            }
            
            // Parse board dimensions
            if let Some(height_str) = properties.get("board.height") {
                if let Ok(height) = height_str.parse::<usize>() {
                    self.board_height = Some(height);
                }
            }
            
            // Parse logging configuration
            if let Some(log_level) = properties.get("log.level") {
                // Validate log level
                let log_level = log_level.to_lowercase();
                if VALID_LOG_LEVELS.contains(&log_level.as_str()) {
                    self.log_level = log_level;
                } else {
                    warn!("Invalid log level '{}' in config file. Using default: {}", 
                          log_level, self.log_level);
                }
            }
            
            // Parse multi-destination logging configuration
            if let Some(log_to_file) = properties.get("log.to.file") {
                let value = log_to_file.to_lowercase();
                self.log_to_file = value == "true" || value == "yes" || value == "on" || value == "1";
            }
            
            if let Some(log_file_path) = properties.get("log.file.path") {
                self.log_file_path = Some(PathBuf::from(log_file_path));
            }
            
            if let Some(log_file_level) = properties.get("log.file.level") {
                let level = log_file_level.to_lowercase();
                if VALID_LOG_LEVELS.contains(&level.as_str()) {
                    self.log_file_level = level;
                } else {
                    warn!("Invalid file log level '{}' in config file. Using default: {}", 
                          level, self.log_file_level);
                }
            }
            
            if let Some(log_console_level) = properties.get("log.console.level") {
                let level = log_console_level.to_lowercase();
                if VALID_LOG_LEVELS.contains(&level.as_str()) {
                    self.log_console_level = level;
                } else {
                    warn!("Invalid console log level '{}' in config file. Using default: {}", 
                          level, self.log_console_level);
                }
            }
            
            if let Some(rotation) = properties.get("log.file.rotation") {
                let value = rotation.to_lowercase();
                self.log_file_rotation = value == "true" || value == "yes" || value == "on" || value == "1";
            }
            
            if let Some(size_limit) = properties.get("log.file.size.limit") {
                if let Ok(size_mb) = size_limit.parse::<u64>() {
                    self.log_file_size_limit = size_mb * 1024 * 1024; // Convert MB to bytes
                }
            }
            
            if let Some(file_count) = properties.get("log.file.count") {
                if let Ok(count) = file_count.parse::<u32>() {
                    self.log_file_count = count;
                }
            }
        }
        Ok(())
    }
    
    fn parse_properties_file(path: &PathBuf) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // Use the java-properties crate to parse the properties file
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let props = java_properties::read(reader)?;
        
        // Convert from java_properties::PropertiesIter to HashMap<String, String>
        let mut properties = HashMap::new();
        for (key, value) in props {
            properties.insert(key, value);
        }
        
        Ok(properties)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Create a new properties map
        let mut props = java_properties::PropertiesWriter::new();
        
        // Add comments for sections
        props.set_comment("Conway's Steinway Configuration File\nGenerated automatically\n\n----- Core Configuration -----");
        
        // Core configuration
        let board_type_str = match self.board_type {
            BoardType::Random => "random",
            BoardType::Static => "static",
            BoardType::FurElise => "fur_elise",
            BoardType::Complex => "complex",
            BoardType::Showcase => "showcase",
        };
        props.set("board.type", board_type_str);
        
        if self.silent {
            props.set("silent", "");
        }
        
        let generations_str = match self.generations {
            GenerationLimit::Unlimited => "unlimited".to_string(),
            GenerationLimit::Limited(n) => n.to_string(),
        };
        props.set("generations", generations_str);
        
        props.set("step.delay.ms", self.step_delay_ms.to_string());
        
        if let Some(tempo) = self.tempo_bpm {
            props.set("tempo.bpm", tempo.to_string());
        }
        
        // Audio settings
        props.set_comment("----- Audio Settings -----");
        props.set("audio.note.duration.ms", self.note_duration_ms.to_string());
        props.set("audio.gap.ms", self.gap_ms.to_string());
        props.set("audio.chord.duration.ms", self.chord_duration_ms.to_string());
        props.set("audio.initial.delay.ms", self.initial_delay_ms.to_string());
        props.set("audio.detect.chords", self.detect_chords.to_string());
        props.set("audio.volume", self.volume.to_string());
        props.set("audio.pitch.shift", self.pitch_shift.to_string());
        
        // Random board settings
        props.set_comment("----- Random Board Settings -----");
        props.set("random.alive.probability", self.alive_probability.to_string());
        
        // Board dimensions
        props.set_comment("----- Board Dimensions -----\nNOTE: Board width is ALWAYS 88 cells to match piano keys and CANNOT be changed.");
        if let Some(height) = self.board_height {
            props.set("board.height", height.to_string());
        } else {
            props.set("board.height", "40");
        }
        
        // Write the properties to the file
        let file = fs::File::create(path)?;
        java_properties::write(props, file)?;
        
        Ok(())
    }

    pub fn tempo_to_delay_ms(bpm: f64) -> u64 {
        // Convert BPM to milliseconds per beat
        // BPM = beats per minute, so ms per beat = (60 * 1000) / BPM
        // For a reasonable musical feel, we'll treat each generation as a beat subdivision
        // Using quarter note subdivision: delay = (60000 / BPM) / 4
        let delay = (60000.0 / bpm) / 2.0; // Using eighth note subdivision
        delay.round() as u64
    }

    pub fn get_effective_delay(&self) -> u64 {
        if let Some(bpm) = self.tempo_bpm {
            Self::tempo_to_delay_ms(bpm)
        } else {
            self.step_delay_ms
        }
    }

    pub fn print_config(&self) {
        println!("Configuration:");
        println!("  Board Type: {:?}", self.board_type);
        println!("  Silent Mode: {}", !self.audio_enabled);
        println!("  Generations: {:?}", self.generations);
        
        if let Some(bpm) = self.tempo_bpm {
            let effective_delay = self.get_effective_delay();
            println!("  Tempo: {:.1} BPM ({}ms per step)", bpm, effective_delay);
        } else {
            println!("  Step Delay: {}ms", self.step_delay_ms);
        }
        
        // Board dimensions
        let height = self.board_height.unwrap_or(40);
        println!("  Board: 88×{}", height);
        
        // Audio settings
        println!("  Audio Settings:");
        println!("    Note Duration: {}ms", self.note_duration_ms);
        println!("    Chord Duration: {}ms", self.chord_duration_ms);
        println!("    Gap Between Notes: {}ms", self.gap_ms);
        println!("    Detect Chords: {}", self.detect_chords);
        println!("    Volume: {:.1}", self.volume);
        println!("    Pitch Shift: {}", self.pitch_shift);
        
        // Random board settings
        if matches!(self.board_type, BoardType::Random) {
            println!("  Random Board: {:.1}% alive cells", self.alive_probability * 100.0);
        }
        
        // Logging settings
        println!("  Logging Settings:");
        println!("    Log Level: {}", self.log_level);
        println!("    Log to File: {}", self.log_to_file);
        if self.log_to_file {
            if let Some(ref path) = self.log_file_path {
                println!("    Log File: {}", path.display());
            } else {
                println!("    Log File: {}/{}", DEFAULT_LOG_DIR, DEFAULT_LOG_FILE);
            }
            println!("    File Log Level: {}", self.log_file_level);
            println!("    Console Log Level: {}", self.log_console_level);
            println!("    File Rotation: {}", self.log_file_rotation);
            if self.log_file_rotation {
                println!("    File Size Limit: {} MB", self.log_file_size_limit / (1024 * 1024));
                println!("    File Count: {}", self.log_file_count);
            }
        }
        
        if let Some(ref path) = self.config_file {
            println!("  Config File: {}", path.display());
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(matches!(config.board_type, BoardType::Random));
        assert!(!config.silent);
        assert!(matches!(config.generations, GenerationLimit::Unlimited));
        assert_eq!(config.step_delay_ms, 200);
        assert!(config.tempo_bpm.is_none());
    }

    #[test]
    fn test_config_file_creation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.properties");
        
        let config = Config {
            board_type: BoardType::Static,
            silent: true,
            generations: GenerationLimit::Unlimited,
            step_delay_ms: 500,
            tempo_bpm: Some(140.0),
            config_file: Some(file_path.clone()),
            ..Default::default()
        };

        config.save_to_file(&file_path).unwrap();
        assert!(file_path.exists());

        let contents = fs::read_to_string(&file_path).unwrap();
        assert!(contents.contains("board.type=static"));
        assert!(contents.contains("silent="));
    }

    #[test]
    fn test_generation_limit_serialization() {
        let unlimited = GenerationLimit::Unlimited;
        let limited = GenerationLimit::Limited(50);

        let config_unlimited = Config {
            generations: unlimited,
            ..Default::default()
        };

        let config_limited = Config {
            generations: limited,
            ..Default::default()
        };

        // Create temporary files for testing
        let dir = tempdir().unwrap();
        let file_unlimited = dir.path().join("unlimited.properties");
        let file_limited = dir.path().join("limited.properties");
        
        // Save to properties files
        config_unlimited.save_to_file(&file_unlimited).unwrap();
        config_limited.save_to_file(&file_limited).unwrap();
        
        // Read contents
        let content_unlimited = fs::read_to_string(&file_unlimited).unwrap();
        let content_limited = fs::read_to_string(&file_limited).unwrap();

        assert!(content_unlimited.contains("generations=unlimited"));
        assert!(content_limited.contains("generations=50"));
    }

    #[test]
    fn test_tempo_conversion() {
        // Test tempo to delay conversion
        let delay_120_bpm = Config::tempo_to_delay_ms(120.0);
        let delay_126_bpm = Config::tempo_to_delay_ms(126.0);
        
        // At 120 BPM, eighth notes should be about 250ms
        assert!((delay_120_bpm as f64 - 250.0).abs() < 10.0, 
               "120 BPM should give ~250ms, got {}ms", delay_120_bpm);
        
        // At 126 BPM should be slightly faster
        assert!(delay_126_bpm < delay_120_bpm, 
               "126 BPM should be faster than 120 BPM");
        
        // Test effective delay
        let mut config = Config::default();
        assert_eq!(config.get_effective_delay(), 200); // Uses step_delay_ms
        
        config.tempo_bpm = Some(120.0);
        assert_eq!(config.get_effective_delay(), delay_120_bpm); // Uses tempo
    }
}
