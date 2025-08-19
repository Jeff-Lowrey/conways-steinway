#!/usr/bin/env python3
"""
Simple script to update pip to the latest version.
Run with: python update_pip.py
"""

import sys
import subprocess
import os

def update_pip():
    print("Updating pip to the latest version...")
    try:
        subprocess.check_call([sys.executable, "-m", "pip", "install", "--upgrade", "pip"])
        print("Pip has been updated successfully!")
    except subprocess.CalledProcessError as e:
        print(f"Error updating pip: {e}", file=sys.stderr)
        return 1
    return 0

if __name__ == "__main__":
    sys.exit(update_pip())
