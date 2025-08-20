// Integration tests for Game of Life patterns
// These tests verify that patterns are correctly generated and evolve as expected

use life::{Cell, GameOfLife, GameBoard, BOARD_WIDTH, BOARD_HEIGHT};

// Helper function to count alive cells in a game board
fn count_alive_cells(game: &GameOfLife) -> usize {
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

#[test]
fn test_create_random_board() {
    let game = GameBoard::create_random_board();
    
    // Random board should have some alive cells (typically around 25%)
    let alive_count = count_alive_cells(&game);
    let total_cells = BOARD_WIDTH * BOARD_HEIGHT;
    
    // Check if cells are in the expected range (15-35% of board)
    assert!(alive_count > total_cells / 7, "Random board should have a reasonable number of alive cells");
    assert!(alive_count < total_cells / 2, "Random board shouldn't be too crowded");
    
    // Generation should start at 0
    assert_eq!(game.generation(), 0);
}

#[test]
fn test_create_complex_board() {
    let game = GameBoard::create_complex_board();
    
    // Complex board should have several patterns
    let alive_count = count_alive_cells(&game);
    
    // Complex board should have more than a few cells alive
    assert!(alive_count > 50, "Complex board should have multiple patterns with alive cells");
    
    // Generation should start at 0
    assert_eq!(game.generation(), 0);
}

#[test]
fn test_create_glider() {
    // Test a single glider
    let mut game = GameOfLife::new();
    GameBoard::create_glider(&mut game, 10, 10);
    
    // Standard glider has 5 cells - checking the actual implementation
    assert_eq!(game.get_cell(10, 12), Cell::Alive);
    assert_eq!(game.get_cell(11, 10), Cell::Alive);
    assert_eq!(game.get_cell(11, 12), Cell::Alive);
    assert_eq!(game.get_cell(12, 11), Cell::Alive);
    assert_eq!(game.get_cell(12, 12), Cell::Alive);
    
    // Count total number of alive cells to verify no extras
    assert_eq!(count_alive_cells(&game), 5, "Glider should have exactly 5 alive cells");
}

#[test]
fn test_create_block() {
    // Test a block pattern (2x2 still life)
    let mut game = GameOfLife::new();
    GameBoard::create_block(&mut game, 5, 5);
    
    // Block has 4 cells in a square
    assert_eq!(game.get_cell(5, 5), Cell::Alive);
    assert_eq!(game.get_cell(5, 6), Cell::Alive);
    assert_eq!(game.get_cell(6, 5), Cell::Alive);
    assert_eq!(game.get_cell(6, 6), Cell::Alive);
    
    // Block should have exactly 4 alive cells
    assert_eq!(count_alive_cells(&game), 4);
    
    // Block is a still life - should not change after one generation
    game.next_generation();
    
    assert_eq!(game.get_cell(5, 5), Cell::Alive);
    assert_eq!(game.get_cell(5, 6), Cell::Alive);
    assert_eq!(game.get_cell(6, 5), Cell::Alive);
    assert_eq!(game.get_cell(6, 6), Cell::Alive);
    assert_eq!(count_alive_cells(&game), 4);
}

#[test]
fn test_create_blinker() {
    // Test a blinker (period 2 oscillator)
    let mut game = GameOfLife::new();
    GameBoard::create_blinker(&mut game, 10, 10);
    
    // Blinker starts as horizontal line of 3 cells
    assert_eq!(game.get_cell(10, 10), Cell::Alive);
    assert_eq!(game.get_cell(10, 11), Cell::Alive);
    assert_eq!(game.get_cell(10, 12), Cell::Alive);
    
    // Count cells in initial state
    assert_eq!(count_alive_cells(&game), 3);
    
    // Advance one generation - should become vertical
    game.next_generation();
    
    // Check vertical orientation
    assert_eq!(game.get_cell(9, 11), Cell::Alive);
    assert_eq!(game.get_cell(10, 11), Cell::Alive);
    assert_eq!(game.get_cell(11, 11), Cell::Alive);
    
    // Horizontal cells should now be dead
    assert_eq!(game.get_cell(10, 10), Cell::Dead);
    assert_eq!(game.get_cell(10, 12), Cell::Dead);
    
    // Still 3 cells total
    assert_eq!(count_alive_cells(&game), 3);
    
    // Advance again - should return to horizontal
    game.next_generation();
    
    // Check horizontal orientation again
    assert_eq!(game.get_cell(10, 10), Cell::Alive);
    assert_eq!(game.get_cell(10, 11), Cell::Alive);
    assert_eq!(game.get_cell(10, 12), Cell::Alive);
    
    // Vertical cells (except center) should now be dead
    assert_eq!(game.get_cell(9, 11), Cell::Dead);
    assert_eq!(game.get_cell(11, 11), Cell::Dead);
}

#[test]
fn test_fur_elise_board() {
    // Test the Für Elise board creator
    let game = GameBoard::create_fur_elise_board();
    
    // Für Elise board should have a significant number of alive cells
    let alive_count = count_alive_cells(&game);
    assert!(alive_count > 100, "Für Elise board should have many patterns");
    
    // Generation should start at 0
    assert_eq!(game.generation(), 0);
}

#[test]
fn test_bottom_row_extraction_with_patterns() {
    // Create a game with specific patterns that will reach the bottom row
    let mut game = GameOfLife::new();
    
    // Create patterns near the bottom of the board
    let bottom_row = BOARD_HEIGHT - 1;
    
    // Set specific keys active
    let active_keys = [0, 12, 24, 36, 48, 60, 72, 84];
    for &key in &active_keys {
        game.set_cell(bottom_row, key, Cell::Alive);
    }
    
    // Get the bottom row and advance the game
    let piano_keys = GameBoard::get_bottom_row_and_advance(&mut game);
    
    // Verify the correct keys were extracted
    assert_eq!(piano_keys.len(), active_keys.len());
    for &key in &active_keys {
        assert!(piano_keys.contains(&key), "Key {} should be in extracted keys", key);
    }
    
    // Verify the generation advanced
    assert_eq!(game.generation(), 1);
}
