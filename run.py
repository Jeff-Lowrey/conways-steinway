#!/usr/bin/env python3
"""
Launcher script for Conway's Steinway

This script adds the config and python directories to the Python path
and then launches the main application.
"""

import os
import sys
from pathlib import Path

# Get the project root directory (where this script is located)
PROJECT_ROOT = Path(__file__).resolve().parent

# Add python and config directories to Python path
PYTHON_PATH = PROJECT_ROOT / "python"
CONFIG_PATH = PROJECT_ROOT / "config"

sys.path.insert(0, str(PYTHON_PATH))
sys.path.insert(0, str(CONFIG_PATH))

# Change to the python directory
os.chdir(str(PYTHON_PATH))

# Import and run the main function
try:
    from main import main
    main()
except ImportError as e:
    print(f"Error importing main module: {e}")
    print(f"Python path: {sys.path}")
    sys.exit(1)
except Exception as e:
    print(f"Error running Conway's Steinway: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)
