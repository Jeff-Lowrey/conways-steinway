#!/bin/bash

# Change to the rust/life directory where Cargo.toml is located
cd "$(dirname "$0")/life"

# Run the compiled binary with logging enabled
RUST_LOG=info cargo run -- "$@"