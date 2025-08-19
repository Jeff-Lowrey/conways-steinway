// Integration tests for the main application flow
// Verifies the full chain of Game of Life, Audio, and Configuration

use config::{Config, BoardType, GenerationLimit};
use life::GameBoard;
use audio::PlayerPiano;

#[test]
fn test_end_to_end_flow() {
    // Create a minimal config for testing
    let mut config = Config::default();
    config.board_type = BoardType::Random;
    config.silent = true; // Silent mode for testing
    config.generations = GenerationLimit::Limited(5); // Just a few generations
    config.step_delay_ms = 0; // No delay for tests
    
    // Print the config in debug mode
    println!("Test configuration: {:?}", config);
    
    // Initialize a game board based on the config
    let mut game = match config.board_type {
        BoardType::Static => GameBoard::create_complex_board(),
        BoardType::FurElise => GameBoard::create_fur_elise_board(),
        BoardType::Random | _ => GameBoard::create_random_board(),
    };
    
    // Create a silent piano
    let piano = PlayerPiano::new_silent();
    
    // Run the simulation for the configured number of generations
    let mut step = 0;
    let should_continue = |current_step: u32| -> bool {
        match config.generations {
            GenerationLimit::Limited(max_generations) => current_step < max_generations,
            GenerationLimit::Unlimited => true,
        }
    };
    
    // Verify game state before starting
    assert_eq!(game.generation(), 0, "Game should start at generation 0");
    
    // Capture the initial board state for comparison
    let initial_alive_count = count_alive_cells(&game);
    
    // Run the main loop
    while should_continue(step) {
        step += 1;
        
        // Extract bottom row and get piano keys
        let piano_keys = GameBoard::get_bottom_row_and_advance(&mut game);
        
        // Simulate playing the keys
        piano.play_keys(&piano_keys);
        
        // Verify the generation has advanced
        assert_eq!(game.generation(), step, "Game generation should match step count");
    }
    
    // Final assertions
    assert_eq!(step, 5, "Simulation should have run for 5 generations");
    assert_eq!(game.generation(), 5, "Final game generation should be 5");
    
    // The board should have changed from its initial state
    let final_alive_count = count_alive_cells(&game);
    println!("Initial alive cells: {}, Final alive cells: {}", 
             initial_alive_count, final_alive_count);
}

#[test]
fn test_config_loading_and_application() {
    // Test that configuration values are correctly applied to the game
    
    // Create a custom config
    let mut config = Config::default();
    config.board_type = BoardType::Static;
    config.silent = true;
    config.generations = GenerationLimit::Limited(1);
    
    // Initialize based on this config
    let board = match config.board_type {
        BoardType::Static => GameBoard::create_complex_board(),
        BoardType::FurElise => GameBoard::create_fur_elise_board(),
        BoardType::Random | _ => GameBoard::create_random_board(),
    };
    
    // Board should have been created using the complex pattern method
    let alive_count = count_alive_cells(&board);
    assert!(alive_count > 50, "Complex board should have multiple patterns");
    
    // Create piano based on config
    let piano = if config.silent {
        PlayerPiano::new_silent()
    } else {
        // Never executed in this test
        PlayerPiano::new()
    };
    
    // Verify silent piano was created (indirectly, by ensuring no crash)
    piano.play_keys(&[0, 1, 2, 3]);
}

// Helper function to count alive cells in a game board
fn count_alive_cells(game: &life::GameOfLife) -> usize {
    use life::{Cell, BOARD_WIDTH, BOARD_HEIGHT};
    
    let mut count = 0;
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            if game.get_cell(row, col) == Cell::Alive {
                count += 1;
            }
        }
    }
    count
}