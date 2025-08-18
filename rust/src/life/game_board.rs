use crate::{GameOfLife, Cell, BOARD_WIDTH, BOARD_HEIGHT};
use log::{info, debug, trace};

pub struct GameBoard;

impl GameBoard {
    pub fn create_random_board() -> GameOfLife {
        debug!("Creating random game board");
        let mut game = GameOfLife::new();
        
        // Simple random seeding based on time-like value
        let mut seed = 12345u64;
        
        // Fill board with random cells (about 25% alive)
        let mut alive_cells = 0;
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                if seed % 4 == 0 {
                    game.set_cell(row, col, Cell::Alive);
                    alive_cells += 1;
                }
            }
        }
        
        debug!("Random board created with {} alive cells", alive_cells);
        game
    }
    
    pub fn create_complex_board() -> GameOfLife {
        debug!("Creating complex game board with predefined patterns");
        let mut game = GameOfLife::new();
        
        // Add multiple gliders at different positions
        debug!("Adding gliders to complex board");
        Self::create_glider(&mut game, 0, 0);
        Self::create_glider(&mut game, 5, 20);
        Self::create_glider(&mut game, 10, 40);
        Self::create_glider(&mut game, 2, 60);
        Self::create_glider(&mut game, 15, 10);
        Self::create_glider(&mut game, 8, 70);
        Self::create_glider(&mut game, 20, 25);
        Self::create_glider(&mut game, 25, 50);
        Self::create_glider(&mut game, 30, 15);
        Self::create_glider(&mut game, 1, 75);
        Self::create_glider(&mut game, 22, 8);
        Self::create_glider(&mut game, 28, 65);
        
        // Add oscillators
        debug!("Adding oscillators to complex board");
        Self::create_blinker(&mut game, 5, 5);
        Self::create_toad(&mut game, 12, 30);
        Self::create_beacon(&mut game, 25, 5);
        Self::create_pentadecathlon(&mut game, 1, 34);
        
        // Add still lifes
        debug!("Adding still lifes to complex board");
        Self::create_block(&mut game, 15, 75);
        Self::create_beehive(&mut game, 10, 50);
        Self::create_loaf(&mut game, 18, 45);
        Self::create_boat(&mut game, 34, 25);
        
        // Add methuselah patterns
        debug!("Adding methuselah patterns to complex board");
        Self::create_r_pentomino(&mut game, 6, 55);
        Self::create_diehard(&mut game, 18, 15);
        Self::create_acorn(&mut game, 32, 35);
        
        // Add spaceships
        debug!("Adding spaceships to complex board");
        Self::create_lwss(&mut game, 35, 60);
        
        debug!("Complex board creation complete");
        game
    }
    
    pub fn add_random_row(game: &mut GameOfLife) {
        trace!("Adding random top row, generation: {}", game.generation());
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        game.generation().hash(&mut hasher);
        let seed = hasher.finish();
        
        let mut rng_state = seed;
        let mut alive_count = 0;
        for col in 0..BOARD_WIDTH {
            rng_state = rng_state.wrapping_mul(1664525).wrapping_add(1013904223);
            let cell = if (rng_state % 5) == 0 {
                alive_count += 1;
                Cell::Alive
            } else {
                Cell::Dead
            };
            game.set_cell(0, col, cell);
        }
        trace!("Added random top row with {} alive cells", alive_count);
    }
    
    pub fn from_pattern(pattern: &[&str]) -> GameOfLife {
        let mut game = GameOfLife::new();
        
        for (row_idx, &row) in pattern.iter().enumerate() {
            if row_idx >= BOARD_HEIGHT { break; }
            
            for (col_idx, ch) in row.chars().enumerate() {
                if col_idx >= BOARD_WIDTH { break; }
                
                if ch == 'O' || ch == 'X' || ch == '*' {
                    game.set_cell(row_idx, col_idx, Cell::Alive);
                }
            }
        }
        
        game
    }
    
    pub fn get_bottom_row_and_advance(game: &mut GameOfLife) -> Vec<usize> {
        debug!("Getting bottom row and advancing board, generation: {}", game.generation());
        
        let bottom_row_keys: Vec<usize> = (0..BOARD_WIDTH)
            .filter(|&col| game.get_cell(BOARD_HEIGHT - 1, col) == Cell::Alive)
            .collect();

        trace!("Bottom row has {} active cells: {:?}", bottom_row_keys.len(), bottom_row_keys);

        // Shift board down (remove bottom row, add empty row at top)
        trace!("Shifting board down one row");
        for row in (1..BOARD_HEIGHT).rev() {
            for col in 0..BOARD_WIDTH {
                let cell = game.get_cell(row - 1, col);
                game.set_cell(row, col, cell);
            }
        }
        
        // Clear top row
        for col in 0..BOARD_WIDTH {
            game.set_cell(0, col, Cell::Dead);
        }
        
        Self::add_random_row(game);
        trace!("Calculating next generation");
        game.next_generation();
        
        debug!("Board advanced to generation: {}", game.generation());
        bottom_row_keys
    }
    
    // Still Life patterns
    pub fn create_block(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col, Cell::Alive);
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 1, Cell::Alive);
    }
    
    pub fn create_beehive(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row, col + 2, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 3, Cell::Alive);
        game.set_cell(row + 2, col + 1, Cell::Alive);
        game.set_cell(row + 2, col + 2, Cell::Alive);
    }
    
    pub fn create_loaf(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row, col + 2, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 3, Cell::Alive);
        game.set_cell(row + 2, col + 1, Cell::Alive);
        game.set_cell(row + 2, col + 3, Cell::Alive);
        game.set_cell(row + 3, col + 2, Cell::Alive);
    }
    
    pub fn create_boat(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col, Cell::Alive);
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 2, Cell::Alive);
        game.set_cell(row + 2, col + 1, Cell::Alive);
    }
    
    // Oscillator patterns
    pub fn create_blinker(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col, Cell::Alive);
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row, col + 2, Cell::Alive);
    }
    
    pub fn create_toad(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row, col + 2, Cell::Alive);
        game.set_cell(row, col + 3, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 1, Cell::Alive);
        game.set_cell(row + 1, col + 2, Cell::Alive);
    }
    
    pub fn create_beacon(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col, Cell::Alive);
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 1, Cell::Alive);
        game.set_cell(row + 2, col + 2, Cell::Alive);
        game.set_cell(row + 2, col + 3, Cell::Alive);
        game.set_cell(row + 3, col + 2, Cell::Alive);
        game.set_cell(row + 3, col + 3, Cell::Alive);
    }
    
    pub fn create_pulsar(game: &mut GameOfLife, row: usize, col: usize) {
        // Top cross
        for i in 0..3 {
            game.set_cell(row + 2, col + 4 + i, Cell::Alive);
            game.set_cell(row + 2, col + 10 + i, Cell::Alive);
        }
        
        // Vertical lines
        for i in 0..3 {
            game.set_cell(row + 4 + i, col + 2, Cell::Alive);
            game.set_cell(row + 4 + i, col + 7, Cell::Alive);
            game.set_cell(row + 4 + i, col + 9, Cell::Alive);
            game.set_cell(row + 4 + i, col + 14, Cell::Alive);
            
            game.set_cell(row + 10 + i, col + 2, Cell::Alive);
            game.set_cell(row + 10 + i, col + 7, Cell::Alive);
            game.set_cell(row + 10 + i, col + 9, Cell::Alive);
            game.set_cell(row + 10 + i, col + 14, Cell::Alive);
        }
        
        // Horizontal lines
        for i in 0..3 {
            game.set_cell(row + 7, col + 4 + i, Cell::Alive);
            game.set_cell(row + 7, col + 10 + i, Cell::Alive);
            game.set_cell(row + 9, col + 4 + i, Cell::Alive);
            game.set_cell(row + 9, col + 10 + i, Cell::Alive);
        }
        
        // Bottom cross
        for i in 0..3 {
            game.set_cell(row + 14, col + 4 + i, Cell::Alive);
            game.set_cell(row + 14, col + 10 + i, Cell::Alive);
        }
    }
    
    pub fn create_pentadecathlon(game: &mut GameOfLife, row: usize, col: usize) {
        // Central line
        for i in 0..8 {
            game.set_cell(row + i, col + 1, Cell::Alive);
        }
        
        // End pieces
        game.set_cell(row + 3, col, Cell::Alive);
        game.set_cell(row + 3, col + 2, Cell::Alive);
        game.set_cell(row + 4, col, Cell::Alive);
        game.set_cell(row + 4, col + 2, Cell::Alive);
    }
    
    // Spaceship patterns
    pub fn create_glider(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col + 2, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 2, Cell::Alive);
        game.set_cell(row + 2, col + 1, Cell::Alive);
        game.set_cell(row + 2, col + 2, Cell::Alive);
    }
    
    pub fn create_lwss(game: &mut GameOfLife, row: usize, col: usize) {
        // Light-weight spaceship
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row, col + 4, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 2, col, Cell::Alive);
        game.set_cell(row + 2, col + 4, Cell::Alive);
        game.set_cell(row + 3, col, Cell::Alive);
        game.set_cell(row + 3, col + 1, Cell::Alive);
        game.set_cell(row + 3, col + 2, Cell::Alive);
        game.set_cell(row + 3, col + 3, Cell::Alive);
    }
    
    pub fn create_mwss(game: &mut GameOfLife, row: usize, col: usize) {
        // Middle-weight spaceship
        game.set_cell(row, col + 2, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 4, Cell::Alive);
        game.set_cell(row + 2, col + 5, Cell::Alive);
        game.set_cell(row + 3, col, Cell::Alive);
        game.set_cell(row + 3, col + 5, Cell::Alive);
        game.set_cell(row + 4, col + 1, Cell::Alive);
        game.set_cell(row + 4, col + 2, Cell::Alive);
        game.set_cell(row + 4, col + 3, Cell::Alive);
        game.set_cell(row + 4, col + 4, Cell::Alive);
        game.set_cell(row + 4, col + 5, Cell::Alive);
    }
    
    pub fn create_hwss(game: &mut GameOfLife, row: usize, col: usize) {
        // Heavy-weight spaceship
        game.set_cell(row, col + 2, Cell::Alive);
        game.set_cell(row, col + 3, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 5, Cell::Alive);
        game.set_cell(row + 2, col + 6, Cell::Alive);
        game.set_cell(row + 3, col, Cell::Alive);
        game.set_cell(row + 3, col + 6, Cell::Alive);
        game.set_cell(row + 4, col + 1, Cell::Alive);
        game.set_cell(row + 4, col + 2, Cell::Alive);
        game.set_cell(row + 4, col + 3, Cell::Alive);
        game.set_cell(row + 4, col + 4, Cell::Alive);
        game.set_cell(row + 4, col + 5, Cell::Alive);
        game.set_cell(row + 4, col + 6, Cell::Alive);
    }
    
    // Methuselah patterns
    pub fn create_r_pentomino(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row, col + 2, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 1, Cell::Alive);
        game.set_cell(row + 2, col + 1, Cell::Alive);
    }
    
    pub fn create_diehard(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col + 6, Cell::Alive);
        game.set_cell(row + 1, col, Cell::Alive);
        game.set_cell(row + 1, col + 1, Cell::Alive);
        game.set_cell(row + 2, col + 1, Cell::Alive);
        game.set_cell(row + 2, col + 5, Cell::Alive);
        game.set_cell(row + 2, col + 6, Cell::Alive);
        game.set_cell(row + 2, col + 7, Cell::Alive);
    }
    
    pub fn create_acorn(game: &mut GameOfLife, row: usize, col: usize) {
        game.set_cell(row, col + 1, Cell::Alive);
        game.set_cell(row + 1, col + 3, Cell::Alive);
        game.set_cell(row + 2, col, Cell::Alive);
        game.set_cell(row + 2, col + 1, Cell::Alive);
        game.set_cell(row + 2, col + 4, Cell::Alive);
        game.set_cell(row + 2, col + 5, Cell::Alive);
        game.set_cell(row + 2, col + 6, Cell::Alive);
    }
    
    // Gun patterns
    pub fn create_gosper_glider_gun(game: &mut GameOfLife, row: usize, col: usize) {
        // Left block
        Self::create_block(game, row + 5, col);
        
        // Left part
        game.set_cell(row + 3, col + 10, Cell::Alive);
        game.set_cell(row + 4, col + 10, Cell::Alive);
        game.set_cell(row + 5, col + 10, Cell::Alive);
        game.set_cell(row + 2, col + 11, Cell::Alive);
        game.set_cell(row + 6, col + 11, Cell::Alive);
        game.set_cell(row + 1, col + 12, Cell::Alive);
        game.set_cell(row + 7, col + 12, Cell::Alive);
        game.set_cell(row + 1, col + 13, Cell::Alive);
        game.set_cell(row + 7, col + 13, Cell::Alive);
        game.set_cell(row + 4, col + 14, Cell::Alive);
        game.set_cell(row + 2, col + 15, Cell::Alive);
        game.set_cell(row + 6, col + 15, Cell::Alive);
        game.set_cell(row + 3, col + 16, Cell::Alive);
        game.set_cell(row + 4, col + 16, Cell::Alive);
        game.set_cell(row + 5, col + 16, Cell::Alive);
        game.set_cell(row + 4, col + 17, Cell::Alive);
        
        // Right part
        game.set_cell(row + 1, col + 20, Cell::Alive);
        game.set_cell(row + 2, col + 20, Cell::Alive);
        game.set_cell(row + 3, col + 20, Cell::Alive);
        game.set_cell(row + 1, col + 21, Cell::Alive);
        game.set_cell(row + 2, col + 21, Cell::Alive);
        game.set_cell(row + 3, col + 21, Cell::Alive);
        game.set_cell(row, col + 22, Cell::Alive);
        game.set_cell(row + 4, col + 22, Cell::Alive);
        game.set_cell(row - 1, col + 24, Cell::Alive);
        game.set_cell(row, col + 24, Cell::Alive);
        game.set_cell(row + 4, col + 24, Cell::Alive);
        game.set_cell(row + 5, col + 24, Cell::Alive);
        
        // Right block
        Self::create_block(game, row + 3, col + 34);
    }
    
    // Board configuration to play "Für Elise" melody
    pub fn create_fur_elise_board() -> GameOfLife {
        let mut game = GameOfLife::new();
        
        // Für Elise melody notes (piano key numbers, 1-88):
        // E5-D#5-E5-D#5-E5-B4-D5-C5-A4 (main phrase)
        // Piano keys: 52-51-52-51-52-47-50-49-45
        
        // Create patterns that will hit the bottom row to play these notes
        // Using careful timing with different pattern types and positions
        
        // E5 (key 52) - First note, immediate impact
        Self::create_glider(&mut game, 36, 51); // Will reach bottom quickly
        
        // D#5 (key 51) - Second note
        Self::create_blinker(&mut game, 35, 50); // Oscillates, hits on step 2
        
        // E5 (key 52) - Third note  
        Self::create_glider(&mut game, 34, 51); // Delayed glider
        
        // D#5 (key 51) - Fourth note
        Self::create_toad(&mut game, 32, 49); // Toad pattern, hits step 4
        
        // E5 (key 52) - Fifth note
        Self::create_glider(&mut game, 30, 51); // Another glider
        
        // B4 (key 47) - Sixth note
        Self::create_r_pentomino(&mut game, 25, 45); // Long-term pattern
        
        // D5 (key 50) - Seventh note
        Self::create_lwss(&mut game, 28, 46); // Spaceship moving toward key 50
        
        // C5 (key 49) - Eighth note  
        Self::create_beacon(&mut game, 26, 47); // Beacon oscillator
        
        // A4 (key 45) - Ninth note
        Self::create_acorn(&mut game, 20, 42); // Acorn methuselah
        
        // Add some supporting patterns for rhythm and harmony
        Self::create_block(&mut game, 15, 40); // Bass note stability
        Self::create_block(&mut game, 15, 55); // High note stability
        
        // Add gliders that will create sustained notes
        Self::create_glider(&mut game, 10, 30); // Lower register accompaniment
        Self::create_glider(&mut game, 8, 60);  // Higher register accompaniment
        
        // Create a "conductor" pattern - pentadecathlon for timing
        Self::create_pentadecathlon(&mut game, 5, 44);
        
        // Add some harmonic patterns
        Self::create_beehive(&mut game, 12, 35); // Harmonic support
        Self::create_loaf(&mut game, 18, 65);    // Treble harmony
        
        // Second phrase preparation - more complex patterns
        Self::create_diehard(&mut game, 15, 20);  // Dies and creates space
        Self::create_gosper_glider_gun(&mut game, 2, 10); // Continuous glider generation
        
        // Add patterns for the second phrase melody
        // C4-E4-A4-B4 sequence (keys 41-44-45-47)
        Self::create_hwss(&mut game, 22, 38);    // Heavy spaceship for C4
        Self::create_mwss(&mut game, 24, 41);    // Medium spaceship for E4
        Self::create_glider(&mut game, 26, 44);  // Glider for A4
        Self::create_pulsar(&mut game, 1, 30);   // Pulsar for complex timing
        
        game
    }
    
    // Helper method to create a board with various patterns for demonstration
    pub fn create_showcase_board() -> GameOfLife {
        let mut game = GameOfLife::new();
        
        // Add various patterns across the board
        Self::create_glider(&mut game, 1, 1);
        Self::create_block(&mut game, 5, 10);
        Self::create_blinker(&mut game, 8, 15);
        Self::create_toad(&mut game, 12, 20);
        Self::create_beacon(&mut game, 16, 25);
        Self::create_lwss(&mut game, 20, 30);
        Self::create_r_pentomino(&mut game, 25, 35);
        Self::create_acorn(&mut game, 30, 40);
        
        // Add some still lifes
        Self::create_beehive(&mut game, 10, 50);
        Self::create_loaf(&mut game, 15, 55);
        Self::create_boat(&mut game, 20, 60);
        
        game
    }
}