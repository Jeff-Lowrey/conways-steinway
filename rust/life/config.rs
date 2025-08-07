use clap::{Arg, ArgAction, Command, ValueHint};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub board_type: BoardType,
    pub audio_enabled: bool,
    pub generations: GenerationLimit,
    pub step_delay_ms: u64,
    pub tempo_bpm: Option<f64>,
    pub config_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardType {
    Random,
    Static,
    FurElise,
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
                .value_parser(["random", "static", "fur-elise"])
                .env("CONWAYS_STEINWAY_BOARD_TYPE"))
            .arg(Arg::new("silent")
                .short('s')
                .long("silent")
                .help("Disable audio output")
                .action(ArgAction::SetTrue)
                .env("CONWAYS_STEINWAY_SILENT"))
            .arg(Arg::new("generations")
                .short('g')
                .long("generations")
                .value_name("COUNT")
                .help("Number of generations to run (0 for unlimited)")
                .value_parser(clap::value_parser!(u32))
                .env("CONWAYS_STEINWAY_GENERATIONS"))
            .arg(Arg::new("delay")
                .short('d')
                .long("delay")
                .value_name("MILLISECONDS")
                .help("Delay between steps in milliseconds")
                .value_parser(clap::value_parser!(u64))
                .env("CONWAYS_STEINWAY_DELAY"))
            .arg(Arg::new("tempo")
                .short('t')
                .long("tempo")
                .value_name("BPM")
                .help("Musical tempo in beats per minute (overrides delay)")
                .value_parser(clap::value_parser!(f64))
                .env("CONWAYS_STEINWAY_TEMPO"));

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
                "fur-elise" => BoardType::FurElise,
                _ => BoardType::Random,
            };
        }

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

        if let Some(&delay) = matches.get_one::<u64>("delay") {
            config.step_delay_ms = delay;
        }

        if let Some(&tempo) = matches.get_one::<f64>("tempo") {
            config.tempo_bpm = Some(tempo);
        }

        Ok(config)
    }

    fn load_from_env(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Environment variables are handled by clap with .env() calls
        // This method is kept for potential future custom env var handling
        Ok(())
    }

    fn load_from_file(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            let file_config: Config = toml::from_str(&contents)?;
            
            // Merge file config with current config (file config takes precedence)
            self.board_type = file_config.board_type;
            self.audio_enabled = file_config.audio_enabled;
            self.generations = file_config.generations;
            self.step_delay_ms = file_config.step_delay_ms;
            self.tempo_bpm = file_config.tempo_bpm;
        }
        Ok(())
    }

    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let contents = toml::to_string_pretty(self)?;
        fs::write(path, contents)?;
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
        println!("  Audio Enabled: {}", self.audio_enabled);
        println!("  Generations: {:?}", self.generations);
        
        if let Some(bpm) = self.tempo_bpm {
            let effective_delay = self.get_effective_delay();
            println!("  Tempo: {:.1} BPM ({}ms per step)", bpm, effective_delay);
        } else {
            println!("  Step Delay: {}ms", self.step_delay_ms);
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
        assert!(config.audio_enabled);
        assert!(matches!(config.generations, GenerationLimit::Unlimited));
        assert_eq!(config.step_delay_ms, 200);
        assert!(config.tempo_bpm.is_none());
    }

    #[test]
    fn test_config_file_creation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_config.toml");
        
        let config = Config {
            board_type: BoardType::Static,
            audio_enabled: false,
            generations: GenerationLimit::Unlimited,
            step_delay_ms: 500,
            tempo_bpm: Some(140.0),
            config_file: Some(file_path.clone()),
        };

        config.save_to_file(&file_path).unwrap();
        assert!(file_path.exists());

        let contents = fs::read_to_string(&file_path).unwrap();
        assert!(contents.contains("board_type = \"Static\""));
        assert!(contents.contains("audio_enabled = false"));
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

        let toml_unlimited = toml::to_string(&config_unlimited).unwrap();
        let toml_limited = toml::to_string(&config_limited).unwrap();

        assert!(toml_unlimited.contains("Unlimited"));
        assert!(toml_limited.contains("Limited"));
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