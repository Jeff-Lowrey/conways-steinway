#!/bin/bash

# Script to run the Rust implementation of Conway's Steinway
# with proper configuration paths

# Get the absolute path of the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
RUST_DIR="$SCRIPT_DIR/rust/life"
CONFIG_DIR="$SCRIPT_DIR/config"

# CD to the Rust directory and run the program
cd "$RUST_DIR" || exit 1
echo "Running Conway's Steinway Rust implementation..."
cargo run -- --config "$CONFIG_DIR/conways_steinway.toml" "$@"