#!/bin/bash
# Script to activate the virtual environment

# Determine the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
VENV_PATH="$SCRIPT_DIR/../.venv"

# Check if virtual environment exists
if [ ! -d "$VENV_PATH" ]; then
    echo "Virtual environment not found at $VENV_PATH"
    echo "Creating new virtual environment..."
    python3 -m venv "$VENV_PATH"
fi

# Activate the virtual environment
source "$VENV_PATH/bin/activate"

# Always update pip to latest version
echo "Updating pip to latest version..."
pip install --upgrade pip

# Install development dependencies if needed
if [ ! -f "$VENV_PATH/.initialized" ]; then
    echo "Installing development dependencies..."
    pip install -e "$SCRIPT_DIR"
    touch "$VENV_PATH/.initialized"
fi

# Check if the package needs to be reinstalled
if [ "$1" == "--reinstall" ]; then
    echo "Reinstalling package..."
    pip install -e "$SCRIPT_DIR"
fi

echo "Virtual environment activated at $VENV_PATH"
echo "You can now run 'python main.py' or other commands"
echo "To reinstall the package, run: source ./activate_venv.sh --reinstall"