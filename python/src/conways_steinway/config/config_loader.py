"""
Conway's Steinway Configuration Loader

This module handles loading configuration from the /config directory.
"""

import os
import sys
from pathlib import Path


def get_config_path() -> Path:
    """Get the path to the config directory"""
    # Try to find the config directory relative to this file
    base_path = Path(__file__).resolve().parent.parent
    config_path = base_path / "config"

    if not config_path.exists():
        # Fallback to current working directory
        current_dir = Path(os.getcwd())
        config_path = current_dir / "config"

        if not config_path.exists():
            # One more fallback - try relative to script execution
            config_path = Path(".").resolve() / "config"

    return config_path


def get_default_config_file() -> Path:
    """Get the default config file path"""
    return get_config_path() / "conways_steinway.properties"


def ensure_config_in_path() -> Path:
    """Ensure the config directory is in the Python path"""
    config_path = get_config_path()
    config_path_str = str(config_path)

    if config_path_str not in sys.path:
        sys.path.insert(0, config_path_str)

    return config_path
