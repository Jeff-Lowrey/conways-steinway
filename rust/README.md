# Conway's Steinway - Rust Implementation

A Rust implementation of Conway's Game of Life that generates piano music based on cellular automaton patterns. Living cells on the bottom row of the game board are interpreted as piano keys, creating emergent musical compositions.

## Overview

This Rust implementation combines Conway's Game of Life with piano audio synthesis. As the cellular automaton evolves, patterns that reach the bottom row trigger piano notes, creating unique musical experiences with each run.

## Features

- Fixed 88×40 game board (width matches piano keys)
- Multiple board initialization types (random, static, fur_elise)
- Piano audio output with sample-based playback
- Chord detection for more musical output
- Standardized configuration system compatible with Python implementation
- Structured logging to multiple destinations (console and file)
- Option to run in silent mode (no audio)

## Project Structure

The Rust implementation is organized as follows:

```
rust/
├── Cargo.toml             # Rust package manifest
├── src/                   # Source code directory
│   ├── main.rs            # Main application entry point
│   ├── audio/             # Audio module for piano sound generation
│   │   ├── mod.rs         # Module definition
│   │   ├── audio.rs       # Audio engine implementation
│   │   └── piano_player.rs # Piano player implementation
│   ├── config/            # Configuration handling
│   │   ├── mod.rs         # Module definition
│   │   ├── types.rs       # Configuration types
│   │   └── loader.rs      # Configuration loading
│   ├── life/              # Game of Life implementation
│   │   ├── mod.rs         # Module definition
│   │   └── game_board.rs  # Game board implementation
│   └── logging.rs         # Multi-destination logging system
├── logs/                  # Directory for log files (created at runtime)
└── run-rust.sh            # Helper script to run the application
```

## Installation

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Basic audio drivers (typically included with OS)

### Building
```bash
cd rust
cargo build --release
```

## Running

You can run the Rust implementation using the included launcher script:

```bash
# From the project root directory
./rust/run-rust.sh

# With custom options
./rust/run-rust.sh --board-type fur_elise --tempo 120
```

Or run it directly with cargo:

```bash
cd rust
cargo run

# With options
cargo run -- --board-type fur_elise --silent
```

## Configuration Options

### Game Options

```
--board-type <type>      Board initialization type (random, static, fur_elise)
--silent                 Disable audio output (audio is enabled by default)
--generations <num>      Generation limit (number or "unlimited")
--step-delay <ms>        Delay between steps in milliseconds
--tempo <bpm>            Musical tempo in beats per minute
--no-detect-chords       Disable automatic chord detection (enabled by default)
--no-pitch-shift         Disable pitch shifting (enabled by default)
```

### Logging Options

The application supports logging to multiple destinations (console and file) with different log levels:

```
--log-level <level>           Global log level (trace, debug, info, warn, error)
--log-to-file                 Enable logging to file
--log-file-path <path>        Custom path for log file
--log-file-level <level>      Log level for file output (default: debug)
--log-console-level <level>   Log level for console output (default: info)
--no-log-file-rotation        Disable log file rotation
--log-file-size-limit <MB>    Size limit for log files in megabytes (default: 10)
--log-file-count <count>      Number of rotated log files to keep (default: 5)
```

By default, when file logging is enabled:
- Log files are stored in the project root `logs/backend/` directory
- Log files are rotated when they reach 10 MB
- Up to 5 rotated log files are kept
- Console log level is set to `info`
- File log level is set to `debug` (more verbose)

Configuration is loaded from the following sources (in order of precedence):
1. Command-line arguments
2. Environment variables
3. Configuration file (`config/conways_steinway.properties`)

## Architecture

### Core Modules

- `main.rs`: Application entry point and game loop
- `logging.rs`: Multi-destination logging implementation
- `config/types.rs`: Configuration types and validation
- `config/loader.rs`: Configuration loading from various sources
- `life/game_board.rs`: Board manipulation and pattern generators
- `audio/audio.rs`: Audio engine for sample playback
- `audio/piano_player.rs`: Piano interface for key mapping

### Game of Life Implementation

The implementation follows Conway's classic rules:
1. Survival: Live cells with 2-3 neighbors survive
2. Birth: Dead cells with exactly 3 neighbors become alive
3. Death: All other cells die or remain dead

### Musical Interpretation

1. Piano Mapping: The 88-column board represents piano keys (A0-C8)
2. Note Triggering: Living cells in the bottom row trigger corresponding piano keys
3. Pattern Evolution: The board continuously evolves, creating new musical patterns
4. Chord Detection: Multiple adjacent keys trigger chord playback

## Boolean Flag Standardization

All boolean options follow a standardized approach:

1. Features are **enabled by default**
2. Only the negative flags are provided to disable features
   - `--silent` to disable audio
   - `--no-detect-chords` to disable chord detection
   - `--no-pitch-shift` to disable pitch shifting
   - `--no-log-file-rotation` to disable log file rotation

## Running Tests

```bash
cd rust
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
