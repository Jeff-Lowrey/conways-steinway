# Conway's Steinway Project

## Overview
This is a multi-language implementation of Conway's Game of Life that generates player piano instructions. The project includes implementations in Go, Python, Rust, and Node.js.

## Project Structure
- `/go/` - Go implementation with main.go
- `/python/` - Python backend with life.py and piano.py modules
- `/rust/` - Rust implementation 
- `/node/` - Node.js implementation
- `/tests/` - Test files
- `/static/` - Static assets
- `/scratch/` - Development scratch space

## Current Branch
Working on: `python` branch (main branch is `main`)

## Python Implementation Details
- Game of Life board: 40+ cells high, exactly 88 cells wide (matching piano keys)
- Bottom row removal returns piano instructions
- Live cells trigger corresponding piano key positions

## Common Commands
(To be updated as development progresses)

## Notes
- Project designed for cloud deployment with Kubernetes
- Focus on scalable and observable architecture
- Currently in development phase with skeleton modules
