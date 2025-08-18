use std::fmt;
use std::thread;
use std::time::Duration;
use log::{info, warn, error, debug, trace};

// Local modules
mod audio;
mod config;
mod life;
mod logging;

use audio::PlayerPiano;
use config::{Config, BoardType, GenerationLimit};
use life::GameBoard;

const BOARD_WIDTH: usize = 88;
const BOARD_HEIGHT: usize = 40;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match *self {
            Cell::Dead => '.',
            Cell::Alive => 'O',
        };
        write!(f, "{}", symbol)
    }
}

pub struct GameOfLife {
    board: Vec<Vec<Cell>>,
    generation: u32,
}

impl GameOfLife {
    pub fn new() -> Self {
        let board = vec![vec![Cell::Dead; BOARD_WIDTH]; BOARD_HEIGHT];
        GameOfLife {
            board,
            generation: 0,
        }
    }

    pub fn from_pattern(pattern: &[&str]) -> Self {
        let mut game = Self::new();
        
        for (row_idx, &row) in pattern.iter().enumerate() {
            if row_idx >= BOARD_HEIGHT { break; }
            
            for (col_idx, ch) in row.chars().enumerate() {
                if col_idx >= BOARD_WIDTH { break; }    
                
                if ch == 'O' || ch == 'X' || ch == '*' {
                    game.board[row_idx][col_idx] = Cell::Alive;
                }
            }
        }
        
        game
    }

    pub fn set_cell(&mut self, row: usize, col: usize, state: Cell) {
        if row < BOARD_HEIGHT && col < BOARD_WIDTH {
            self.board[row][col] = state;
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        if row < BOARD_HEIGHT && col < BOARD_WIDTH {
            self.board[row][col]
        } else {
            Cell::Dead
        }
    }

    fn count_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        
        for dr in -1i32..=1 {
            for dc in -1i32..=1 {
                if dr == 0 && dc == 0 { continue; }
                
                let new_row = row as i32 + dr;
                let new_col = col as i32 + dc;
                
                if new_row >= 0 && new_row < BOARD_HEIGHT as i32 &&
                   new_col >= 0 && new_col < BOARD_WIDTH as i32 {
                    if self.board[new_row as usize][new_col as usize] == Cell::Alive {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    pub fn next_generation(&mut self) {
        let mut new_board = self.board.clone();
        
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                let neighbors = self.count_neighbors(row, col);
                let current_cell = self.board[row][col];
                
                new_board[row][col] = match (current_cell, neighbors) {
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (Cell::Dead, _) => Cell::Dead,
                };
            }
        }
        
        self.board = new_board;
        self.generation += 1;
    }

    pub fn get_bottom_row_and_advance(&mut self) -> Vec<usize> {
        let bottom_row_keys: Vec<usize> = self.board[BOARD_HEIGHT - 1]
            .iter()
            .enumerate()
            .filter_map(|(idx, &cell)| {
                if cell == Cell::Alive { Some(idx) } else { None }
            })
            .collect();

        self.board.remove(BOARD_HEIGHT - 1);
        self.board.insert(0, vec![Cell::Dead; BOARD_WIDTH]);
        
        self.add_random_top_row();
        self.next_generation();
        
        bottom_row_keys
    }

    pub fn add_random_top_row(&mut self) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.generation.hash(&mut hasher);
        let seed = hasher.finish();
        
        let mut rng_state = seed;
        for col in 0..BOARD_WIDTH {
            rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            self.board[0][col] = if (rng_state % 5) == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            };
        }
    }

    pub fn generation(&self) -> u32 {
        self.generation
    }
}

impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Generation: {}", self.generation)?;
        writeln!(f, "Piano Keys: 1-88 (left to right)")?;
        writeln!(f, "{}", "=".repeat(BOARD_WIDTH + 4))?;
        
        for row in &self.board {
            write!(f, "| ")?;
            for &cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f, " |")?;
        }
        
        writeln!(f, "{}", "=".repeat(BOARD_WIDTH + 4))
    }
}

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
        }
    };

    // Initialize audio based on configuration
    let piano = if config.audio_enabled {
        PlayerPiano::new()
    } else {
        PlayerPiano::new_silent()
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