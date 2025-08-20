// Integration tests for Game of Life logic
// These tests verify core game logic without relying on mocks or external resources

use life::{Cell, GameOfLife, GameBoard, BOARD_WIDTH, BOARD_HEIGHT};

#[test]
fn test_new_game_creation() {
    let game = GameOfLife::new();
    
    // Verify dimensions
    assert_eq!(BOARD_WIDTH, 88, "Board width should be 88 (piano keys)");
    assert_eq!(BOARD_HEIGHT, 40, "Board height should be 40");
    
    // Verify initial state - all cells should be dead
    for row in 0..BOARD_HEIGHT {
        for col in 0..BOARD_WIDTH {
            assert_eq!(game.get_cell(row, col), Cell::Dead, 
                       "Cell at ({}, {}) should be dead initially", row, col);
        }
    }
    
    // Verify generation counter starts at 0
    assert_eq!(game.generation(), 0, "Initial generation should be 0");
}

#[test]
fn test_cell_manipulation() {
    let mut game = GameOfLife::new();
    
    // Set some cells and verify they were set correctly
    let test_cells = [
        (0, 0),
        (5, 10),
        (BOARD_HEIGHT - 1, BOARD_WIDTH - 1),
        (BOARD_HEIGHT / 2, BOARD_WIDTH / 2),
    ];
    
    for &(row, col) in &test_cells {
        game.set_cell(row, col, Cell::Alive);
        assert_eq!(game.get_cell(row, col), Cell::Alive,
                   "Cell at ({}, {}) should be alive after setting", row, col);
    }
    
    // Check bounds handling for set_cell
    game.set_cell(BOARD_HEIGHT, BOARD_WIDTH, Cell::Alive);  // Out of bounds
    game.set_cell(BOARD_HEIGHT + 10, BOARD_WIDTH + 10, Cell::Alive);  // Far out of bounds
    
    // Check bounds handling for get_cell
    assert_eq!(game.get_cell(BOARD_HEIGHT, BOARD_WIDTH), Cell::Dead,
               "Out of bounds cells should return Dead");
    assert_eq!(game.get_cell(BOARD_HEIGHT + 10, BOARD_WIDTH + 10), Cell::Dead,
               "Far out of bounds cells should return Dead");
}

#[test]
fn test_from_pattern() {
    // Create a simple pattern and verify it loads correctly
    let pattern = [
        "...O.",
        "..O.O",
        "...O.",
    ];
    
    let game = GameOfLife::from_pattern(&pattern);
    
    // Check that the pattern was loaded correctly
    assert_eq!(game.get_cell(0, 3), Cell::Alive);
    assert_eq!(game.get_cell(1, 2), Cell::Alive);
    assert_eq!(game.get_cell(1, 4), Cell::Alive);
    assert_eq!(game.get_cell(2, 3), Cell::Alive);
    
    // And that cells outside the pattern are dead
    assert_eq!(game.get_cell(0, 0), Cell::Dead);
    assert_eq!(game.get_cell(0, 1), Cell::Dead);
    assert_eq!(game.get_cell(0, 2), Cell::Dead);
    assert_eq!(game.get_cell(0, 4), Cell::Dead);
}

#[test]
fn test_game_rules() {
    // Test the rules of the Game of Life by setting up known patterns
    // and verifying they evolve correctly
    
    // 1. A block pattern (2x2) is stable and should not change
    let mut block = GameOfLife::new();
    // Set up a block at (5,5)
    block.set_cell(5, 5, Cell::Alive);
    block.set_cell(5, 6, Cell::Alive);
    block.set_cell(6, 5, Cell::Alive);
    block.set_cell(6, 6, Cell::Alive);
    
    // Get the state before evolving
    let block_before = [
        block.get_cell(5, 5),
        block.get_cell(5, 6),
        block.get_cell(6, 5),
        block.get_cell(6, 6),
    ];
    
    // Evolve one generation
    block.next_generation();
    
    // Get the state after evolving
    let block_after = [
        block.get_cell(5, 5),
        block.get_cell(5, 6),
        block.get_cell(6, 5),
        block.get_cell(6, 6),
    ];
    
    // Verify the block is stable
    assert_eq!(block_before, block_after, "Block pattern should be stable");
    assert_eq!(block.generation(), 1, "Generation should be incremented");
    
    // 2. A blinker pattern oscillates with period 2
    let mut blinker = GameOfLife::new();
    // Set up a horizontal blinker at (10,10)
    blinker.set_cell(10, 10, Cell::Alive);
    blinker.set_cell(10, 11, Cell::Alive);
    blinker.set_cell(10, 12, Cell::Alive);
    
    // Verify horizontal state
    assert_eq!(blinker.get_cell(9, 11), Cell::Dead);
    assert_eq!(blinker.get_cell(10, 11), Cell::Alive);
    assert_eq!(blinker.get_cell(11, 11), Cell::Dead);
    
    // Evolve one generation - should become vertical
    blinker.next_generation();
    
    // Verify vertical state
    assert_eq!(blinker.get_cell(9, 11), Cell::Alive);
    assert_eq!(blinker.get_cell(10, 11), Cell::Alive);
    assert_eq!(blinker.get_cell(11, 11), Cell::Alive);
    assert_eq!(blinker.get_cell(10, 10), Cell::Dead);
    assert_eq!(blinker.get_cell(10, 12), Cell::Dead);
    
    // Evolve one more generation - should return to horizontal
    blinker.next_generation();
    
    // Verify horizontal state again
    assert_eq!(blinker.get_cell(10, 10), Cell::Alive);
    assert_eq!(blinker.get_cell(10, 11), Cell::Alive);
    assert_eq!(blinker.get_cell(10, 12), Cell::Alive);
    assert_eq!(blinker.get_cell(9, 11), Cell::Dead);
    assert_eq!(blinker.get_cell(11, 11), Cell::Dead);
    
    assert_eq!(blinker.generation(), 2, "Generation should be incremented twice");
}

#[test]
fn test_bottom_row_extraction() {
    let mut game = GameOfLife::new();
    
    // Set up some cells in the bottom row
    let bottom_row = BOARD_HEIGHT - 1;
    let active_keys = [0, 5, 10, 20, 40, 87];
    
    for &col in &active_keys {
        game.set_cell(bottom_row, col, Cell::Alive);
    }
    
    // Extract bottom row and advance
    let extracted_keys = GameBoard::get_bottom_row_and_advance(&mut game);
    
    // Verify extracted keys match our active keys
    assert_eq!(extracted_keys.len(), active_keys.len());
    for &key in &active_keys {
        assert!(extracted_keys.contains(&key), "Extracted keys should include {}", key);
    }
    
    // Verify generation has advanced
    assert_eq!(game.generation(), 1, "Generation should be incremented after extraction");
    
    // Verify bottom row is now empty (shifted up)
    for col in 0..BOARD_WIDTH {
        assert_eq!(game.get_cell(bottom_row, col), Cell::Dead, 
                   "Bottom row should be empty after extraction");
    }
    
    // In this implementation, we're not specifically testing if random cells were added
    // since that's an implementation detail that might change.
    // Instead, we just verify the generation incremented correctly, which is the important part.
    assert_eq!(game.generation(), 1, "Generation should be incremented after extraction");
}
