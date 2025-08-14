# Python Backend for Conway's Steinway

## Overview
This is the Python implementation of the backend for Conway's Steinway, which combines Conway's Game of Life with a piano interface. Each live cell in the bottom row of the Game of Life board triggers a corresponding piano key.

## Features
- Configurable Game of Life board (default: 88×40 cells)
- Real-time generation of piano notes based on the Game of Life patterns
- Modular design with separate Life and Piano classes
- Mutable sound output for thread safety

## Setup

### Prerequisites
- Python 3.7 or higher

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/conways-steinway.git
   cd conways-steinway/python
   ```

2. Create and activate the virtual environment:
   
   **On macOS/Linux:**
   ```bash
   # Option 1: Use the provided script (recommended)
   source ./activate_venv.sh
   
   # To reinstall the package:
   source ./activate_venv.sh --reinstall
   ```
   
   **On Windows:**
   ```cmd
   # Option 1: Use the provided script (recommended)
   .\activate_venv.bat
   
   # To reinstall the package:
   .\activate_venv.bat --reinstall
   ```
   
   **Manual activation (all platforms):**
   ```bash
   # On macOS/Linux:
   python -m venv ../.venv
   source ../.venv/bin/activate
   
   # On Windows:
   python -m venv ../.venv
   ../.venv\Scripts\activate
   ```
   
   The project is configured to use a virtual environment at `../.venv` (one level up from the python directory).

3. Install dependencies:
   ```bash
   pip install -e .
   ```
   
   This will install the package in development mode and automatically update pip to the latest version.

## Usage

### Running the Application
To run the main demonstration, use the project-level launcher script:
```bash
# From the project root directory
./run.py
```

Alternatively, you can run the main.py directly:
```bash
python main.py
```

This will:
1. Initialize a Game of Life board with random live cells
2. Play the piano based on the bottom row for 20 generations
3. Display a visual representation of the piano keys being played

### Development

#### Updating Pip

The package includes several ways to ensure pip is always updated:

1. **During Installation**: Pip is automatically updated when installing the package
2. **Using the Command**: Run the update-pip command that's installed with the package
   ```bash
   update-pip
   ```
3. **Directly Using the Script**: Execute the update_pip.py script
   ```bash
   python update_pip.py
   ```

#### Configuration

The project uses a centralized configuration structure:

```
/config
  conways_steinway.toml    # Shared configuration file for all implementations
  pyproject.toml          # Python project configuration (includes dependencies)

/python
  config.py               # Python configuration management
  config_loader.py        # Loads configuration from the shared file
  .env                    # Environment variables
  setup.py                # Installation script
  activate_venv.sh        # Virtual environment activation (Unix)
  activate_venv.bat       # Virtual environment activation (Windows)
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
The project uses pytest for testing. Run the tests with:

```bash
python -m pytest tests/
```

See the [tests/README.md](./tests/README.md) for more details on testing.

## Future Enhancements
1. Add actual sound output (using a library like pygame)
2. Create a visual interface for the Game of Life board
3. Add different initial patterns and save/load functionality
4. Implement tempo control for the piano playback

## Design Decisions

1. **Row Representation**: Each row is represented as a list of 88 values with state represented as binary (0=Dead, 1=Alive)
2. **Controller Implementation**: The Piano class acts as the controller for running the game
3. **Board Display**: Currently uses simple text output, but could be enhanced with a graphical interface

## License
[License information goes here]