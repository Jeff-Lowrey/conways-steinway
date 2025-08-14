@echo off
REM Script to activate the virtual environment on Windows

REM Determine the directory where this script is located
set SCRIPT_DIR=%~dp0
set VENV_PATH=%SCRIPT_DIR%..\\.venv

REM Check if virtual environment exists
if not exist "%VENV_PATH%" (
    echo Virtual environment not found at %VENV_PATH%
    echo Creating new virtual environment...
    python -m venv "%VENV_PATH%"
)

REM Activate the virtual environment
call "%VENV_PATH%\Scripts\activate.bat"

REM Always update pip to latest version
echo Updating pip to latest version...
pip install --upgrade pip

REM Install development dependencies if needed
if not exist "%VENV_PATH%\.initialized" (
    echo Installing development dependencies...
    pip install -e "%SCRIPT_DIR%"
    type nul > "%VENV_PATH%\.initialized"
)

REM Check if the package needs to be reinstalled
if "%1"=="--reinstall" (
    echo Reinstalling package...
    pip install -e "%SCRIPT_DIR%"
)

echo Virtual environment activated at %VENV_PATH%
echo You can now run "python main.py" or other commands
echo To reinstall the package, run: activate_venv.bat --reinstall