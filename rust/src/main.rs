use std::thread;
use std::time::Duration;
use log::{info, debug};

// Import crate items directly
use audio::PlayerPiano;
use config::{Config, BoardType, GenerationLimit};
use life::GameBoard;

fn main() {
    // Load configuration first to get log level
    let pre_config = match Config::from_args_and_env() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize the multi-destination logging system
    if let Err(e) = logging::init_logging(&pre_config) {
        eprintln!("Error initializing logging system: {}", e);
        std::process::exit(1);
    }
    
    info!("Conway's Steinway - Rust Implementation");
    info!("======================================");
    debug!("Initialized with log level: {}", pre_config.log_level);

    // Use the already loaded configuration
    let mut config = pre_config;

    // Apply board-specific configuration - Für Elise gets special treatment
    match config.board_type {
        BoardType::FurElise => {
            // Always use 80 generations for complete musical experience
            if !matches!(config.generations, GenerationLimit::Limited(80)) {
                info!("Für Elise always uses 80 generations for complete musical experience (ignoring --generations flag)");
            }
            config.generations = GenerationLimit::Limited(80);
            
            // Set appropriate musical tempo if not explicitly set
            if config.tempo_bpm.is_none() {
                config.tempo_bpm = Some(126.0); // Für Elise typical tempo
                info!("Setting Für Elise tempo to 126 BPM for authentic musical timing");
            }
        },
        _ => {
            // Other board types use configured settings
        }
    }

    // Print current configuration
    config.print_config();

    // Initialize the game board based on configuration
    let mut game = match config.board_type {
        BoardType::Static => {
            info!("Using complex predefined patterns");
            GameBoard::create_complex_board()
        },
        BoardType::FurElise => {
            info!("Using Für Elise melody configuration");
            GameBoard::create_fur_elise_board()
        },
        BoardType::Random => {
            info!("Using random board configuration");
            GameBoard::create_random_board()
        },
        BoardType::Complex | BoardType::Showcase => {
            // Default to random board for these types
            info!("Using random board for {:?} type", config.board_type);
            GameBoard::create_random_board()
        }
    };

    // Initialize audio based on configuration
    let piano = if config.silent {
        PlayerPiano::new_silent()
    } else {
        PlayerPiano::new()
    };

    // Run the simulation based on generation limit
    let mut step = 0;
    let should_continue = |current_step: u32| -> bool {
        match config.generations {
            GenerationLimit::Limited(max_generations) => current_step < max_generations,
            GenerationLimit::Unlimited => true,
        }
    };

    while should_continue(step) {
        step += 1;
        
        match config.generations {
            GenerationLimit::Limited(max) => info!("\nStep {} of {}", step, max),
            GenerationLimit::Unlimited => info!("\nStep {} (unlimited)", step),
        }
        
        let piano_keys = GameBoard::get_bottom_row_and_advance(&mut game);
        piano.play_keys(&piano_keys);
        
        // Use configured delay between steps (respects tempo if set)
        thread::sleep(Duration::from_millis(config.get_effective_delay()));
        
        info!("\n{}", game);

        // For unlimited generations, allow graceful interruption
        if matches!(config.generations, GenerationLimit::Unlimited) && step % 100 == 0 {
            info!("(Press Ctrl+C to stop after {} steps)", step);
        }
    }
    
    info!("\nSimulation completed after {} generations", step);
    info!("Final generation: {}", game.generation());
}