# Conway's Steinway Configuration Guide

Conway's Steinway now supports comprehensive configuration through multiple sources:

## Configuration Sources (in order of precedence)

1. **Command line arguments** (highest priority)
2. **Configuration file** (TOML format)
3. **Environment variables**
4. **Default values** (lowest priority)

## Command Line Options

```bash
# Show help
cargo run -- --help

# Basic options
cargo run                                        # Run with default settings (unlimited generations)
cargo run -- --silent                           # Disable audio
cargo run -- --generations 10                   # Run for 10 generations
cargo run -- --generations 0                    # Run unlimited generations (explicit)
cargo run -- --board-type static                # Use static board pattern
cargo run -- --board-type fur-elise             # Use Für Elise pattern (always 80 generations)
cargo run -- --delay 500                        # 500ms delay between steps
cargo run -- --config my_config.toml            # Use specific config file

# Combined example
cargo run -- --board-type static --generations 5 --silent --delay 100
```

## Environment Variables

All command line options can be set via environment variables:

```bash
# Set environment variables
export CONWAYS_STEINWAY_BOARD_TYPE=fur-elise
export CONWAYS_STEINWAY_GENERATIONS=15
export CONWAYS_STEINWAY_SILENT=true
export CONWAYS_STEINWAY_DELAY=300

# Run with environment configuration
cargo run
```

## Configuration File (TOML)

Create a `.toml` file with your preferred settings:

```toml
# Example: my_config.toml

# Board initialization type: "Random", "Static", or "FurElise"
board_type = "Static"

# Enable or disable audio output
audio_enabled = false

# Generation limit: use "Limited" with a number or "Unlimited"
generations = { Limited = 25 }
# generations = "Unlimited"    # For unlimited generations

# Delay between steps in milliseconds
step_delay_ms = 150
```

Then use it:
```bash
cargo run -- --config my_config.toml
```

## Configuration Options

### Board Types
- `random` - Randomly generated patterns (default)
- `static` - Complex predefined patterns
- `fur-elise` - Für Elise melody-based patterns

### Generation Limits
- `Limited(n)` - Run for exactly n generations
- `Unlimited` - Run indefinitely (default, use Ctrl+C to stop)
- Use `0` on command line for unlimited generations
- **Special case**: Für Elise **always** uses exactly 80 generations for complete musical experience (ignores --generations flag)

### Audio Options
- `true` - Enable realistic piano sample playback with full 88-key coverage (default)
- `false` - Silent mode for testing or background runs

**Audio System**: Uses high-quality piano samples with intelligent pitch-shifting to provide authentic sound for all 88 piano keys. The system automatically selects the best sample for each key and applies chromatic pitch adjustments with volume compensation.

### Step Delay
- Milliseconds between each generation step
- Lower values = faster animation
- Higher values = easier to follow visually
- Default: 200ms

## Examples

### Quick Test Run
```bash
cargo run -- --silent --generations 3 --delay 50
```

### Musical Performance
```bash
cargo run -- --board-type fur-elise --delay 400    # Always 80 generations for full song
cargo run -- --board-type fur-elise                # Even simpler - uses all defaults (80 generations)
cargo run -- --board-type fur-elise --generations 10   # Still uses 80 generations (flag ignored)
```

### Long Analysis Run
```bash
cargo run -- --board-type static --generations 0 --silent --delay 10
```

### Configuration File Example
```bash
# Create config file
cat > analysis.toml << EOF
board_type = "Static"
audio_enabled = false
generations = { Limited = 100 }
step_delay_ms = 50
EOF

# Use it
cargo run -- --config analysis.toml
```

### Environment Variables Example
```bash
# Set up environment for batch processing
export CONWAYS_STEINWAY_SILENT=true
export CONWAYS_STEINWAY_DELAY=10
export CONWAYS_STEINWAY_GENERATIONS=50

# Run multiple experiments
for board_type in random static fur-elise; do
    echo "Testing $board_type board..."
    CONWAYS_STEINWAY_BOARD_TYPE=$board_type cargo run
done
```

## Priority Example

If you have:
- Config file: `generations = { Limited = 10 }`
- Environment: `CONWAYS_STEINWAY_GENERATIONS=20`
- Command line: `--generations 30`

The program will use 30 generations (command line wins).

## Sample Workflow

1. Create a default config file for your preferred settings
2. Use environment variables for batch processing
3. Override with command line arguments for quick tests
4. Use `--generations 0` for unlimited exploration runs

This flexible configuration system allows Conway's Steinway to adapt to different use cases, from quick testing to long-running musical performances.
