#!/bin/bash

# Change to the rust directory where Cargo.toml is now located
cd "$(dirname "$0")"

# Run the compiled binary with standard logging
cargo run -- "$@"