# Conway's Steinway - Core Rust Implementation

This directory contains the core implementation of Conway's Game of Life with piano output in Rust.

## Overview

The `rust/life` directory contains the main Game of Life implementation, configuration handling, and integration with the audio module.

## Project Structure

```
rust/life/
├── src/                # Source code for the Rust implementation
│   ├── config_loader.rs    # Configuration file loading
│   └── unified_config.rs   # Shared configuration system
├── tests/             # Integration tests
├── Cargo.toml         # Cargo configuration and dependencies
├── config.rs          # Configuration handling
├── game_board.rs      # Board manipulation and pattern generators
└── lib.rs             # Core library definitions
```

## Building and Running

### Building
```bash
cargo build --release
```

### Running
```bash
cargo run
```

### With Options
```bash
cargo run -- --board-type fur_elise --tempo 120
```

## Configuration

Configuration options:

```
--board-type <type>      Board initialization type (random, static, fur_elise)
--silent                 Disable audio output (audio is enabled by default)
--generations <num>      Generation limit (number or "unlimited")
--step-delay <ms>        Delay between steps in milliseconds
--tempo <bpm>            Musical tempo in beats per minute
--no-detect-chords       Disable automatic chord detection (enabled by default)
--no-pitch-shift         Disable pitch shifting (enabled by default)
```

## Running Tests

```bash
cargo test
```

## Adding New Features

### New Board Patterns

To add a new board pattern:

1. Open `game_board.rs`
2. Add your pattern generation function
3. Update the `create_board` function to handle your pattern type

Example:

```rust
pub fn create_my_pattern(game: &mut GameOfLife, row: usize, col: usize) {
    game.set_cell(row, col, Cell::Alive);
    game.set_cell(row, col + 1, Cell::Alive);
    // ... define your pattern
}
```

### Extending Configuration

To add new configuration options:

1. Update the `Config` struct in `config.rs`
2. Add command-line argument handling
3. Add environment variable support
4. Add configuration file support in `unified_config.rs`

## Dependencies

This module depends on:
- The audio module (`../audio/`)
- The configuration files (`../../config/`)