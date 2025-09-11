# Python Backend for Conway's Steinway

## Overview
This is the Python implementation of Conway's Steinway, which combines Conway's Game of Life with a piano interface. Each live cell in the bottom row of the Game of Life board triggers a corresponding piano key. The board is 88 cells wide (matching the number of keys on a piano) with a configurable height (default: 40 cells).

## Features
- Configurable Game of Life board (fixed width: 88 cells, configurable height: default 40 cells)
- Real-time generation of piano notes based on the Game of Life patterns
- Multiple board types (random, static, fur_elise, complex, showcase)
- Chord detection for better audio playback
- Pitch shifting for more musical output
- Modular design with separate Life and Piano classes
- Support for configuration via command-line, environment variables, and config files

## Setup

### Prerequisites
- Python 3.13 or higher
- PDM (Python Dependency Manager) - install with `pip install pdm`

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Jeff-Lowrey/conways-steinway.git
   cd conways-steinway/python
   ```

2. Install dependencies using PDM:
   ```bash
   # Install dependencies (creates __pypackages__ directory for PEP 582)
   pdm install
   
   # Or install in a virtual environment if PEP 582 is not supported
   pdm venv create
   pdm use .venv
   pdm install
   ```
   
   PDM supports PEP 582, which means packages are installed in a `__pypackages__` directory rather than requiring a virtual environment. This provides automatic package isolation without activation scripts.

## Usage

### Running the Application
With PDM, you can run the application using PDM scripts:
```bash
# Run using PDM script
pdm run start

# Or run directly with PDM
pdm run python main.py

# Or run directly if using PEP 582
python main.py
```

This will:
1. Initialize a Game of Life board with random live cells
2. Play the piano based on the bottom row (audio enabled by default)
3. Display a visual representation of the piano keys being played
4. Run indefinitely until interrupted (Ctrl+C)

### Development

#### Development Commands

PDM provides several convenient scripts for development:

```bash
# Run the application
pdm run start

# Run tests
pdm run test

# Update pip (legacy support)
pdm run update-pip

# Install development dependencies
pdm install --dev
```

#### Configuration

The project uses a centralized configuration structure:

```
/config
  conways_steinway.properties  # Shared configuration file (Java properties format)
  .env                        # Environment variables for development
  scripts/                    # Helper scripts for environment setup

/python
  config.py                   # Python configuration management
  config_loader.py            # Loads configuration from the shared file
  setup.py                    # Installation script
  pyproject.toml              # Python project configuration (includes dependencies)
```

##### Command-line Arguments

Run with `--help` to see all available options:

```bash
python main.py --help
```

Key options include:

```
--board-type {random,static,fur_elise,complex,showcase}
                    Board initialization type
--silent            Disable audio output (audio is enabled by default)
--generations GENERATIONS
                    Generation limit (number or "Unlimited")
--step-delay STEP_DELAY
                    Delay between steps in milliseconds
--tempo TEMPO       Musical tempo in beats per minute
--no-detect-chords  Disable automatic chord detection (enabled by default)
--no-pitch-shift    Disable pitch shifting (enabled by default)
```

#### Core Components

1. **life.py**: Implements Conway's Game of Life
   - `Life` class: Manages the game board and rules
   - `Row` class: Represents a single row of the game board

2. **piano.py**: Implements the piano interface
   - `Piano` class: Converts game rows to piano key triggers
   
3. **config.py**: Configuration management
   - Located in the `python` directory
   - Manages command-line arguments, environment variables, and configuration files

#### Design Decisions

1. **Row Representation**: Each row is represented as a list of binary values (0=dead, 1=alive)
2. **Board Configuration**: The board width matches piano keys (88) with a configurable height (default 40)
3. **Piano Interface**: Each column position directly maps to a piano key position

## Testing
The project uses pytest for testing. Run the tests with PDM:

```bash
# Run tests using PDM script
pdm run test

# Or run tests directly
pdm run pytest tests/
```

See the [tests/README.md](./tests/README.md) for more details on testing.

## Future Enhancements
1. Create a visual interface for the Game of Life board
2. Add more initial patterns and save/load functionality
3. Implement additional musical features (scales, arpeggios)
4. Add support for MIDI output
5. Create a web-based interface

## Design Decisions

1. **Row Representation**: Each row is represented as a list of 88 values with state represented as binary (0=Dead, 1=Alive)
2. **Controller Implementation**: The Piano class acts as the controller for running the game
3. **Board Display**: Currently uses simple text output, but could be enhanced with a graphical interface

## License
[LICENSE](../LICENSE) 