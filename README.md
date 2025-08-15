# Conway's Steinway

An implementation of Conway's Game of Life creating player piano instructions, using several different languages and frameworks.

Conway's Game of Life is a cellular automaton that evolves a set of black and white squares according to simple rules. This project adapts the Game of Life board into a scrolling field, and uses this as the paper tape controls for a player piano.

The project has been implemented in multiple languages (Python, Rust, with more planned) to demonstrate different approaches to the same problem. Each implementation shares a common configuration system and behavior.

## Project Structure

```
/
├── config/                 # Centralized configuration files
│   ├── conways_steinway.properties  # Main configuration file (Java properties format)
│   ├── scripts/             # Helper scripts for environment setup
│   └── .env                 # Environment variables for development
├── python/                # Python implementation
│   ├── config.py           # Configuration handling
│   ├── config_loader.py    # Loads configuration from central location
│   ├── life.py             # Game of Life implementation
│   ├── piano.py            # Piano sound generation
│   └── main.py             # Main entry point
├── rust/                  # Rust implementation
│   ├── audio/              # Audio modules
│   ├── config/             # Configuration handling
│   └── life/               # Game of Life modules
├── static/                # Static assets
├── run.py                 # Python launcher script
└── run-rust.sh            # Rust launcher script
```

## Quick Start

### Running the Python Version

```bash
# From the project root directory
./run.py

# With custom options
./run.py --board-type fur_elise --tempo 120
```

See [python/README.md](python/README.md) for more details on the Python implementation.

### Running the Rust Version

```bash
# From the project root directory
./run-rust.sh

# With custom options
./run-rust.sh --board-type showcase --tempo 100
```

Or run it directly with cargo:

```bash
cd rust/life
cargo run
```

## Configuration

The project uses a centralized configuration structure in the `/config` directory. All implementations share a common configuration file in Java properties format (`conways_steinway.properties`).

### Configuration Options

Configuration can be set through three methods (in order of precedence):

1. Command-line arguments
2. Environment variables
3. Configuration file (`config/conways_steinway.properties`)

### Command-line Arguments

```
Common options:
  --board-type {random,static,fur_elise,complex,showcase}
                        Board initialization type
  --silent              Disable audio output (audio is enabled by default)
  --generations GENERATIONS
                        Generation limit (number or "Unlimited")
  --step-delay STEP_DELAY
                        Delay between steps in milliseconds
  --tempo TEMPO         Musical tempo in beats per minute

Audio options:
  --note-duration NOTE_DURATION_MS
                        Duration of individual notes in milliseconds
  --gap GAP_MS          Gap between notes in milliseconds
  --chord-duration CHORD_DURATION_MS
                        Duration of chords in milliseconds
  --no-detect-chords    Disable automatic chord detection (enabled by default)
  --volume VOLUME       Audio volume (0.0-1.0)
  --no-pitch-shift      Disable pitch shifting (enabled by default)

Board options:
  --alive-probability ALIVE_PROBABILITY
                        Probability of cells being alive in random boards (0.0-1.0)
  --height BOARD_HEIGHT
                        Board height in cells (width is fixed at 88 to match piano keys)
```

### Environment Variables

Environment variables use the `CONWAYS_STEINWAY_` prefix followed by the setting name in uppercase:

```
CONWAYS_STEINWAY_BOARD_TYPE="random"
CONWAYS_STEINWAY_SILENT="true"  # Set to "true" to disable audio
CONWAYS_STEINWAY_GENERATIONS="unlimited"
CONWAYS_STEINWAY_STEP_DELAY="200"
CONWAYS_STEINWAY_TEMPO="120.0"
```

### Configuration File

The configuration file (`config/conways_steinway.properties`) uses Java properties format:

```properties
# Board configuration
board.type=random
board.height=40

# Audio is enabled by default. Just including this key enables silent mode.
# silent

# Generations ("unlimited" or a number)
generations=unlimited

# Timing
step.delay.ms=200
# tempo.bpm=120.0

# Audio settings
audio.note.duration.ms=200
audio.gap.ms=50
audio.chord.duration.ms=300

# Chord detection is enabled by default. Set to "false" to disable.
# audio.detect.chords=false

# Pitch shifting is enabled by default. Set to "false" to disable.
# audio.pitch.shift=false
```

### Boolean Flag Standardization

All boolean options follow a standardized approach:

1. Features are **enabled by default**
2. Only the negative flags are provided to disable features
   - `--silent` to disable audio
   - `--no-detect-chords` to disable chord detection
   - `--no-pitch-shift` to disable pitch shifting

## Running Tests

### Python Tests

```bash
# Run all tests
python -m pytest python/tests/

# Run specific test files
python -m pytest python/tests/test_life.py
python -m pytest python/tests/test_piano.py
python -m pytest python/tests/test_audio.py
```

### Rust Tests

```bash
cd rust/life
cargo test
```

## Project Goals

1. **Unified Configuration**: All language implementations share a common configuration structure
2. **Audio Consistency**: Each implementation should produce similar audio output given the same input
3. **Performance Analysis**: Compare performance characteristics across language implementations
4. **Modular Design**: Common components across implementations with language-specific optimizations
5. **CI/CD Integration**: Automated testing and review workflows for all submitted code
6. **Documentation**: Comprehensive documentation for all components

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request
