# Python pytests for Conway's Steinway

This directory contains pytest-based tests for the Conway's Steinway Python implementation.

## Test Structure

1. **test_life.py**: Tests for the Game of Life implementation
   - `test_life_initialization`: Verifies that a Life instance can be created with default parameters
   - `test_custom_board_size`: Verifies that a Life instance can be created with custom dimensions
   - `test_get_row`: Tests that get_row() returns the bottom row and adds a new row at the top
   - `test_next_generation`: Verifies that the next_generation method correctly applies Conway's rules

2. **test_piano.py**: Tests for the Piano implementation
   - `test_piano_initialization`: Verifies that a Piano instance can be created with default parameters
   - `test_piano_custom_parameters`: Verifies that a Piano instance can be created with custom parameters
   - `test_piano_mute`: Tests the mute functionality of the Piano class
   - `test_piano_play`: Tests that the Piano play method correctly processes rows from Life

## Running Tests

Run all tests:
```bash
python -m pytest tests/
```

Run tests with detailed output:
```bash
python -m pytest tests/ -v
```

Run a specific test file:
```bash
python -m pytest tests/test_life.py
```

## Decisions Made

1. **Test Coverage**: The tests focus on core functionality without being exhaustive
   - Board initialization and configuration
   - Game of Life rules implementation
   - Row removal and addition
   - Piano note playing logic
   - Mute functionality

2. **Sound Output Testing**: 
   - We test the logic of which keys would be played rather than actual sound output
   - The tests verify that the correct key numbers are identified for playing
   - For CI/CD environments, this approach avoids sound-related dependencies

## Future Improvements

1. **Expanded Coverage**: Add more tests for edge cases like board boundaries
2. **Performance Testing**: Add tests for performance metrics with larger boards
3. **Integration Testing**: Add tests that verify end-to-end functionality from game to piano output
4. **Visual Output**: Add tests for visual representation of the game board
