# conways-steinway
An implementation of Conway's Game of Life creating player piano instructions, using several different languages and frameworks

Conway's Game of Life is a cellular automaton that evolves a set of black and white sqaures according to simple rules.

A full explanation of Conway's Game of Life is not relevant here.

This project adapts the Game of Life board into a scrolling field, and uses this as the paper tape controls for a player piano.

I've implemented this in several different languages, both as a study for myself about thoes languages and an example of my skills.

This is a frivolous project, but is intended and designed to work as part of a highly scalable and observable cloud application running in Kubernetes containers.

Or at least some kind of scalable and observable cloud application running on something - possibly with backends running serverless or who knows.

## Project Structure

```
/
├── config/              # Centralized configuration files
│   ├── python/         # Python configuration
│   └── rust/           # Rust configuration
├── python/             # Python implementation
├── rust/               # Rust implementation
│   ├── audio/          # Audio modules
│   └── life/           # Game of Life modules
├── static/             # Static assets
└── run.py              # Main launcher script
```

## Quick Start

### Running the Python Version

```bash
# From the project root directory
./run.py
```

See [python/README.md](python/README.md) for more details on the Python implementation.

### Running the Rust Version

```bash
# From the project root directory
cd rust/life
cargo run
```

## Configuration

The project uses a centralized configuration structure in the `/config` directory. Each language implementation has its own subdirectory with configuration files specific to that implementation.

