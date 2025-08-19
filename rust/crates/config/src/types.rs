// Conway's Steinway Configuration Types
//
// This file contains the core configuration types and their implementations.

use clap::{Arg, ArgAction, Command, ValueHint};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use log::warn;
// Import life crate to access BOARD_WIDTH constant
use life;
// Import configparser for INI parsing
use configparser::ini::Ini;
// Path is used in implementation

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub board_type: BoardType,
    #[serde(default = "default_silent")]
    pub silent: bool, // Changed from audio_enabled to match Python implementation
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
    pub volume: f64, // Changed from f32 to f64 to match Python
    #[serde(default = "default_pitch_shift")]
    pub pitch_shift: bool,
    
    // Random board settings
    #[serde(default = "default_alive_probability")]
    pub alive_probability: f64, // Changed from f32 to f64 to match Python
    
    // Board dimensions (fixed height, width is fixed at 88 cells by a constant)
    #[serde(default = "default_board_height")]
    pub board_height: usize, // Changed from Option<usize> to usize to match Python
    
    // Logging configuration
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    // Multi-destination logging settings
    #[serde(default = "default_log_destinations")]
    pub log_destinations: Vec<LogDestination>,
    
    // Legacy logging settings (for backward compatibility)
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
fn default_silent() -> bool { false } // Audio is enabled by default (silent=false)
fn default_note_duration() -> u64 { 200 }
fn default_gap_ms() -> u64 { 50 }
fn default_chord_duration() -> u64 { 300 }
fn default_initial_delay() -> u64 { 50 }
fn default_detect_chords() -> bool { true }
fn default_volume() -> f64 { 0.6 } // Changed from f32 to f64
fn default_pitch_shift() -> bool { true }
fn default_alive_probability() -> f64 { 0.2 } // Changed from f32 to f64
// Board width is now a fixed constant (life::BOARD_WIDTH = 88)
fn default_board_height() -> usize { 40 }
fn default_log_level() -> String { "info".to_string() }
fn default_log_to_file() -> bool { false }
fn default_log_file_path() -> Option<PathBuf> { None }
fn default_log_file_level() -> String { "debug".to_string() }
fn default_log_console_level() -> String { "info".to_string() }
fn default_log_file_rotation() -> bool { true }
fn default_log_file_size_limit() -> u64 { 10 * 1024 * 1024 } // 10 MB
fn default_log_file_count() -> u32 { 5 }
fn default_log_destinations() -> Vec<LogDestination> { 
    vec![
        LogDestination {
            name: "console".to_string(),
            destination_type: LogDestinationType::Console,
            level: "info".to_string(),
            pattern: None,
            file_path: None,
            rotation: None,
            http: None,
            syslog: None,
            socket: None,
            fluentd: None,
            gelf: None,
            mongodb: None,
            postgres: None,
            kafka: None,
            rabbitmq: None,
            redis: None,
        }
    ]
}

// Valid log levels that can be used
pub const VALID_LOG_LEVELS: [&str; 5] = ["trace", "debug", "info", "warn", "error"];

// Default log file name and subdirectory
pub const DEFAULT_LOG_FILE: &str = "conways_steinway.log";
pub const DEFAULT_LOG_SUBDIR: &str = "backend";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LogDestinationType {
    // Basic appenders (already supported)
    Console,
    File,
    Json,
    
    // Network appenders
    Http,      // HTTP/HTTPS endpoints
    Syslog,    // Syslog servers (local or remote)
    Socket,    // Raw TCP/UDP socket
    Fluentd,   // Fluentd data collector
    Gelf,      // Graylog Extended Log Format
    
    // Database appenders
    MongoDB,   // MongoDB database
    Postgres,  // PostgreSQL database
    
    // Message queue appenders
    Kafka,     // Apache Kafka
    RabbitMQ,  // RabbitMQ
    Redis      // Redis pub/sub or lists
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotationConfig {
    pub enabled: bool,
    #[serde(default = "default_log_file_size_limit")]
    pub size_limit: u64,
    #[serde(default = "default_log_file_count")]
    pub file_count: u32,
}

impl Default for LogRotationConfig {
    fn default() -> Self {
        LogRotationConfig {
            enabled: true,
            size_limit: default_log_file_size_limit(),
            file_count: default_log_file_count(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub key_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub url: String,
    pub method: String,
    pub content_type: String,
    pub batch_size: Option<usize>,
    pub timeout_ms: Option<u64>,
    pub retry_count: Option<usize>,
    pub headers: Option<HashMap<String, String>>,
    pub auth: Option<AuthConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyslogConfig {
    pub hostname: String,
    pub port: Option<u16>,
    pub facility: String,
    pub protocol: Option<String>, // "tcp", "udp", or unspecified for local
    pub app_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketConfig {
    pub hostname: String,
    pub port: u16,
    pub protocol: String, // "tcp" or "udp"
    pub retry_count: Option<usize>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FluentdConfig {
    pub hostname: String,
    pub port: u16,
    pub tag: String,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GelfConfig {
    pub hostname: String,
    pub port: u16,
    pub protocol: String, // "tcp", "udp", "http"
    pub compression: Option<bool>,
    pub additional_fields: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDBConfig {
    pub connection_string: String,
    pub database: String,
    pub collection: String,
    pub batch_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub connection_string: String,
    pub table: String,
    pub batch_size: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub brokers: Vec<String>,
    pub topic: String,
    pub client_id: Option<String>,
    pub compression: Option<String>, // "none", "gzip", "snappy", "lz4"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RabbitMQConfig {
    pub uri: String,
    pub exchange: String,
    pub routing_key: String,
    pub declare_exchange: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub uri: String,
    pub key: String,
    pub mode: String, // "list", "pubsub", "channel"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogDestination {
    pub name: String,
    pub destination_type: LogDestinationType,
    #[serde(default = "default_log_level")]
    pub level: String,
    pub pattern: Option<String>,
    pub file_path: Option<PathBuf>,
    pub rotation: Option<LogRotationConfig>,
    
    // New appender configurations
    pub http: Option<HttpConfig>,
    pub syslog: Option<SyslogConfig>,
    pub socket: Option<SocketConfig>,
    pub fluentd: Option<FluentdConfig>,
    pub gelf: Option<GelfConfig>,
    pub mongodb: Option<MongoDBConfig>,
    pub postgres: Option<PostgresConfig>,
    pub kafka: Option<KafkaConfig>,
    pub rabbitmq: Option<RabbitMQConfig>,
    pub redis: Option<RedisConfig>,
}

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

impl GenerationLimit {
    // This method helps to match Python's GenerationLimit behavior
    pub fn is_limited(&self) -> bool {
        match self {
            GenerationLimit::Limited(_) => true,
            GenerationLimit::Unlimited => false,
        }
    }
    
    // Get the limit value if limited, otherwise None
    pub fn limit(&self) -> Option<u32> {
        match self {
            GenerationLimit::Limited(value) => Some(*value),
            GenerationLimit::Unlimited => None,
        }
    }
    
    // Parse from a string, similar to Python's constructor
    pub fn from_string(value: &str) -> Self {
        if value.to_lowercase() == "unlimited" {
            GenerationLimit::Unlimited
        } else if let Ok(num) = value.parse::<u32>() {
            if num == 0 {
                GenerationLimit::Unlimited
            } else {
                GenerationLimit::Limited(num)
            }
        } else {
            GenerationLimit::Unlimited
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            board_type: BoardType::Random,
            silent: default_silent(), // Changed from audio_enabled to silent
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
            
            // Board dimensions (height only, width is fixed at 88 cells)
            board_height: default_board_height(), // Changed from Option<usize> to usize
            
            // Logging configuration
            log_level: default_log_level(),
            log_destinations: default_log_destinations(),
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
                .value_parser(clap::value_parser!(f64))
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
                .value_parser(clap::value_parser!(f64))
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

        // Audio is enabled by default (silent=false)
        // Set silent=true if the --silent flag is present
        if matches.get_flag("silent") {
            config.silent = true;
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
        
        if let Some(&volume) = matches.get_one::<f64>("volume") {
            config.volume = volume;
        }
        
        // Pitch shifting is enabled by default (pitch_shift=true)
        // Only set pitch_shift=false if the --no-pitch-shift flag is present
        if matches.get_flag("no-pitch-shift") {
            config.pitch_shift = false;
        }
        
        // Random board settings from command line
        if let Some(&alive_probability) = matches.get_one::<f64>("alive-probability") {
            config.alive_probability = alive_probability;
        }
        
        // Board dimensions from command line
        if let Some(&height) = matches.get_one::<usize>("height") {
            config.board_height = height;
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
            // Parse the configuration file
            let properties = Self::parse_properties_file(path)?;
            
            // Apply core configuration values
            if let Some(board_type) = properties.get("core_board_type") {
                self.board_type = match board_type.to_lowercase().as_str() {
                    "static" => BoardType::Static,
                    "fur_elise" => BoardType::FurElise,
                    "complex" => BoardType::Complex,
                    "showcase" => BoardType::Showcase,
                    _ => BoardType::Random,
                };
            }
            
            // Check for silent mode setting
            if let Some(silent) = properties.get("core_silent") {
                self.silent = silent.to_lowercase() == "true";
            }
            
            // Parse generations
            if let Some(generations_str) = properties.get("core_generations") {
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
            if let Some(delay_str) = properties.get("core_step_delay_ms") {
                if let Ok(delay) = delay_str.parse::<u64>() {
                    self.step_delay_ms = delay;
                }
            }
            
            // Parse tempo
            if let Some(tempo_str) = properties.get("core_tempo_bpm") {
                if let Ok(tempo) = tempo_str.parse::<f64>() {
                    self.tempo_bpm = Some(tempo);
                }
            }
            
            // Parse audio settings
            if let Some(note_duration_str) = properties.get("audio_note_duration_ms") {
                if let Ok(duration) = note_duration_str.parse::<u64>() {
                    self.note_duration_ms = duration;
                }
            }
            
            if let Some(gap_str) = properties.get("audio_gap_ms") {
                if let Ok(gap) = gap_str.parse::<u64>() {
                    self.gap_ms = gap;
                }
            }
            
            if let Some(chord_duration_str) = properties.get("audio_chord_duration_ms") {
                if let Ok(duration) = chord_duration_str.parse::<u64>() {
                    self.chord_duration_ms = duration;
                }
            }
            
            if let Some(initial_delay_str) = properties.get("audio_initial_delay_ms") {
                if let Ok(delay) = initial_delay_str.parse::<u64>() {
                    self.initial_delay_ms = delay;
                }
            }
            
            if let Some(detect_chords_str) = properties.get("audio_detect_chords") {
                let value = detect_chords_str.to_lowercase();
                self.detect_chords = value == "true" || value == "yes" || value == "on" || value == "1";
            }
            
            if let Some(volume_str) = properties.get("audio_volume") {
                if let Ok(volume) = volume_str.parse::<f64>() {
                    self.volume = volume;
                }
            }
            
            if let Some(pitch_shift_str) = properties.get("audio_pitch_shift") {
                let value = pitch_shift_str.to_lowercase();
                self.pitch_shift = value == "true" || value == "yes" || value == "on" || value == "1";
            }
            
            // Parse random board settings
            if let Some(alive_prob_str) = properties.get("random_alive_probability") {
                if let Ok(prob) = alive_prob_str.parse::<f64>() {
                    self.alive_probability = prob;
                }
            }
            
            // Parse board dimensions
            if let Some(height_str) = properties.get("board_height") {
                if let Ok(height) = height_str.parse::<usize>() {
                    self.board_height = height;
                }
            }
            
            // Parse logging configuration
            if let Some(log_level) = properties.get("logging_level") {
                // Validate log level
                let log_level = log_level.to_lowercase();
                if VALID_LOG_LEVELS.contains(&log_level.as_str()) {
                    self.log_level = log_level;
                } else {
                    warn!("Invalid log level '{}' in config file. Using default: {}", 
                          log_level, self.log_level);
                }
            }
            
            // Handle destinations directly - we'll keep these fields for compatibility
            // with the logging module, but they're no longer configured through legacy settings
            self.log_to_file = false;  // Disable legacy file logging by default
            
            // Log file destination will be handled through the destinations config
            let log_path_prefix = properties.get("rust_log_path_prefix")
                .map(|s| s.to_string())
                .unwrap_or_else(|| "logs/rust".to_string());
            
            // Set default log file path
            self.log_file_path = Some(PathBuf::from(format!("{}/conways_steinway.log", log_path_prefix)));
            
            // Parse logging destinations from INI sections
            if let Some(console_level) = properties.get("logging_destinations_console_level") {
                let level = console_level.to_lowercase();
                if VALID_LOG_LEVELS.contains(&level.as_str()) {
                    // Update the console level in the destinations
                    for dest in &mut self.log_destinations {
                        if dest.destination_type == LogDestinationType::Console {
                            dest.level = level.clone();
                        }
                    }
                }
            }
            
            // Parse logging destinations
            self.parse_logging_destinations(&properties);
        }
        Ok(())
    }
    
    fn parse_properties_file(path: &PathBuf) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // Use the configparser crate to parse the INI file
        let mut ini = Ini::new();
        let ini_map = ini.load(path)?;
        
        // Convert from INI format to flat HashMap<String, String>
        let mut properties = HashMap::new();
        
        for (section, props) in ini_map.iter() {
            for (key, value) in props.iter() {
                let config_key = if section == "DEFAULT" {
                    // Keys in the DEFAULT section are used as-is
                    key.clone()
                } else {
                    // Keys in other sections are prefixed with the section name
                    format!("{}_{}", section.to_lowercase(), key)
                };
                
                if let Some(val) = value {
                    properties.insert(config_key, val.clone());
                }
            }
        }
        
        // Special case for handling the old "silent" property
        // which might be present as a boolean value or a flag
        if ini_map.get("core").and_then(|props| props.get("silent")).is_some() {
            properties.insert("silent".to_string(), "true".to_string());
        }
        
        Ok(properties)
    }

    // Helper method to parse logging destinations from properties
    fn parse_logging_destinations(&mut self, properties: &HashMap<String, String>) {
        // Start with default console destination
        self.log_destinations = default_log_destinations();
        
        // Look for any console destination configuration and update the default one
        if let Some(pattern) = properties.get("logging_destinations_console_pattern") {
            if let Some(dest) = self.log_destinations.iter_mut().find(|d| d.destination_type == LogDestinationType::Console) {
                dest.pattern = Some(pattern.clone());
            }
        }
        
        // We could add support for file and other destinations here in a full implementation
        // For now, we'll just keep it simple with the console destination
    }
    
    // Helper function to save configuration to a file
    #[cfg(test)]
    fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // Create an INI config
        let mut ini = Ini::new();
        
        // Core configuration
        let board_type_str = match self.board_type {
            BoardType::Random => "random",
            BoardType::Static => "static",
            BoardType::FurElise => "fur_elise",
            BoardType::Complex => "complex",
            BoardType::Showcase => "showcase",
        };
        
        ini.set("core", "board_type", Some(board_type_str.to_string()));
        
        if self.silent {
            ini.set("core", "silent", Some("true".to_string()));
        }
        
        let generations_str = match self.generations {
            GenerationLimit::Unlimited => "unlimited".to_string(),
            GenerationLimit::Limited(n) => n.to_string(),
        };
        ini.set("core", "generations", Some(generations_str));
        
        ini.set("core", "step_delay_ms", Some(self.step_delay_ms.to_string()));
        
        if let Some(tempo) = self.tempo_bpm {
            ini.set("core", "tempo_bpm", Some(tempo.to_string()));
        }
        
        // Audio settings
        ini.set("audio", "note_duration_ms", Some(self.note_duration_ms.to_string()));
        ini.set("audio", "gap_ms", Some(self.gap_ms.to_string()));
        ini.set("audio", "chord_duration_ms", Some(self.chord_duration_ms.to_string()));
        ini.set("audio", "initial_delay_ms", Some(self.initial_delay_ms.to_string()));
        ini.set("audio", "detect_chords", Some(self.detect_chords.to_string()));
        ini.set("audio", "volume", Some(self.volume.to_string()));
        ini.set("audio", "pitch_shift", Some(self.pitch_shift.to_string()));
        
        // Random board settings
        ini.set("random", "alive_probability", Some(self.alive_probability.to_string()));
        
        // Board dimensions
        ini.set("board", "height", Some(self.board_height.to_string()));
        
        // Logging settings
        ini.set("logging", "level", Some(self.log_level.clone()));
        ini.set("logging", "to_file", Some(self.log_to_file.to_string()));
        ini.set("logging", "file_level", Some(self.log_file_level.clone()));
        ini.set("logging", "console_level", Some(self.log_console_level.clone()));
        ini.set("logging", "file_rotation", Some(self.log_file_rotation.to_string()));
        ini.set("logging", "file_size_limit", Some((self.log_file_size_limit / (1024 * 1024)).to_string()));
        ini.set("logging", "file_count", Some(self.log_file_count.to_string()));
        
        if let Some(ref file_path) = self.log_file_path {
            ini.set("logging", "file_path", Some(file_path.to_string_lossy().to_string()));
        }
        
        // Write the INI file
        ini.write(path.to_str().unwrap())?;
        
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
        println!("  Silent Mode: {}", self.silent);
        println!("  Generations: {:?}", self.generations);
        
        if let Some(bpm) = self.tempo_bpm {
            let effective_delay = self.get_effective_delay();
            println!("  Tempo: {:.1} BPM ({}ms per step)", bpm, effective_delay);
        } else {
            println!("  Step Delay: {}ms", self.step_delay_ms);
        }
        
        // Board dimensions
        println!("  Board: {}×{}", life::BOARD_WIDTH, self.board_height);
        
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
        println!("    Logging Destinations: {}", self.log_destinations.len());
        for (i, dest) in self.log_destinations.iter().enumerate() {
            println!("    Destination #{}: {}", i+1, dest.name);
            println!("      Type: {:?}", dest.destination_type);
            println!("      Level: {}", dest.level);
            if let Some(ref pattern) = dest.pattern {
                println!("      Pattern: {}", pattern);
            }
            if let Some(ref path) = dest.file_path {
                println!("      File Path: {}", path.display());
            }
            if let Some(ref rotation) = dest.rotation {
                println!("      Rotation: enabled={}", rotation.enabled);
                if rotation.enabled {
                    println!("      Size Limit: {} MB", rotation.size_limit / (1024 * 1024));
                    println!("      File Count: {}", rotation.file_count);
                }
            }
        }
        
        // Legacy logging settings
        println!("    Legacy Log Settings:");
        println!("      Log to File: {}", self.log_to_file);
        if self.log_to_file {
            if let Some(ref path) = self.log_file_path {
                println!("      Log File: {}", path.display());
            } else {
                println!("      Log File: logs/{}/{}", DEFAULT_LOG_SUBDIR, DEFAULT_LOG_FILE);
            }
            println!("      File Log Level: {}", self.log_file_level);
            println!("      Console Log Level: {}", self.log_console_level);
            println!("      File Rotation: {}", self.log_file_rotation);
            if self.log_file_rotation {
                println!("      File Size Limit: {} MB", self.log_file_size_limit / (1024 * 1024));
                println!("      File Count: {}", self.log_file_count);
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
        assert!(!config.silent); // Audio is enabled by default (silent=false)
        assert!(matches!(config.generations, GenerationLimit::Unlimited));
        assert_eq!(config.step_delay_ms, 200);
        assert!(config.tempo_bpm.is_none());
    }

    #[test]
    fn test_config_file_creation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.cfg");
        
        let config = Config {
            board_type: BoardType::Static,
            silent: true, // Changed from audio_enabled: false
            generations: GenerationLimit::Unlimited,
            step_delay_ms: 500,
            tempo_bpm: Some(140.0),
            config_file: Some(file_path.clone()),
            ..Default::default()
        };

        config.save_to_file(&file_path).unwrap();
        assert!(file_path.exists());

        let contents = fs::read_to_string(&file_path).unwrap();
        println!("Config file contents: {}", contents);
        assert!(contents.contains("[core]"));
        assert!(contents.contains("board_type=static"));
        assert!(contents.contains("silent=true"));
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
        let file_unlimited = dir.path().join("unlimited.cfg");
        let file_limited = dir.path().join("limited.cfg");
        
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
