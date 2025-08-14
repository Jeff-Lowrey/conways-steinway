"""
Conway's Steinway Configuration Module

This module handles configuration loading for the Conway's Game of Life
piano generator. It includes:
- Command-line argument parsing
- Environment variable loading
- Configuration file handling
"""

import argparse
import os
import enum
import configparser
from jproperties import Properties
from pathlib import Path
from typing import Optional, Dict, Any, List, Union

class BoardType(enum.Enum):
    """Type of board initialization to use"""
    RANDOM = "random"
    STATIC = "static"
    FUR_ELISE = "fur_elise"
    COMPLEX = "complex"
    SHOWCASE = "showcase"
    
    @classmethod
    def from_string(cls, value: str) -> 'BoardType':
        """Convert string to BoardType enum"""
        # Normalize input: lowercase and replace hyphens/spaces with underscores
        normalized = value.lower().replace('-', '_').replace(' ', '_')
        
        for member in cls:
            if member.value == normalized:
                return member
        raise ValueError(f"Invalid board type: {value}")

class GenerationLimit:
    """Configuration for generation limit"""
    UNLIMITED = "unlimited"
    
    def __init__(self, value: Union[str, int, Dict[str, int]] = UNLIMITED):
        """Initialize with 'unlimited', a number, or a dict {'limited': number}"""
        if isinstance(value, str) and value.lower() == self.UNLIMITED.lower():
            self.is_limited = False
            self.limit = None
        elif isinstance(value, int) or (isinstance(value, str) and value.isdigit()):
            self.is_limited = True
            self.limit = int(value)
        elif isinstance(value, dict) and 'limited' in value:
            self.is_limited = True
            self.limit = int(value['limited'])
        else:
            raise ValueError(f"Invalid generation limit: {value}")
    
    def __str__(self) -> str:
        if not self.is_limited:
            return self.UNLIMITED
        return f"Limited({self.limit})"

class Config:
    """Configuration for Conway's Steinway"""
    
    def __init__(self):
        """Initialize with default values"""
        # Board initialization type
        self.board_type = BoardType.RANDOM
        
        # Control audio output (silent = no audio)
        self.silent = False
        
        # Generation limit
        self.generations = GenerationLimit(GenerationLimit.UNLIMITED)
        
        # Delay between steps in milliseconds
        self.step_delay_ms = 200
        
        # Musical tempo in beats per minute (optional)
        self.tempo_bpm: Optional[float] = None
        
        # Config file path
        self.config_file: Optional[Path] = None
        
        # Audio configuration
        self.note_duration_ms = 200
        self.gap_ms = 50
        self.chord_duration_ms = 300
        self.initial_delay_ms = 50
        self.detect_chords = True
        self.volume = 0.6
        self.pitch_shift = True
        
        # Random board configuration
        self.alive_probability = 0.2
        
        # Board dimensions
        self.board_width = 88  # Fixed at 88 cells to match piano keys
        self.board_height = 40
    
    @classmethod
    def from_args_and_env(cls) -> 'Config':
        """Create a configuration from command-line args and environment variables"""
        config = cls()
        
        # Parse command-line arguments
        parser = argparse.ArgumentParser(description="Conway's Steinway - Game of Life Piano Generator")
        parser.add_argument('--config', type=str, help='Path to configuration file')
        parser.add_argument('--board-type', type=str, choices=['random', 'static', 'fur_elise', 'complex', 'showcase'], 
                           help='Board initialization type')
        parser.add_argument('--silent', action='store_true', help='Disable audio output')
        parser.add_argument('--audio', action='store_true', dest='audio', 
                           help='Enable audio output')
        parser.add_argument('--generations', type=str, 
                           help='Generation limit (number or "Unlimited")')
        parser.add_argument('--step-delay', type=int, 
                           help='Delay between steps in milliseconds')
        parser.add_argument('--tempo', type=float, 
                           help='Musical tempo in beats per minute')
        
        # Audio settings
        parser.add_argument('--note-duration', type=int, dest='note_duration_ms',
                           help='Duration of individual notes in milliseconds')
        parser.add_argument('--gap', type=int, dest='gap_ms',
                           help='Gap between notes in milliseconds')
        parser.add_argument('--chord-duration', type=int, dest='chord_duration_ms',
                           help='Duration of chords in milliseconds')
        parser.add_argument('--initial-delay', type=int, dest='initial_delay_ms',
                           help='Initial delay between notes in milliseconds')
        parser.add_argument('--detect-chords', action='store_true', dest='detect_chords',
                           help='Enable automatic chord detection')
        parser.add_argument('--no-detect-chords', action='store_false', dest='detect_chords',
                           help='Disable automatic chord detection')
        parser.add_argument('--volume', type=float,
                           help='Audio volume (0.0-1.0)')
        parser.add_argument('--pitch-shift', action='store_true', dest='pitch_shift',
                           help='Enable pitch shifting')
        parser.add_argument('--no-pitch-shift', action='store_false', dest='pitch_shift',
                           help='Disable pitch shifting')
        
        # Random board settings
        parser.add_argument('--alive-probability', type=float,
                           help='Probability of cells being alive in random boards (0.0-1.0)')
        
        # Board dimensions
        parser.add_argument('--height', type=int, dest='board_height',
                           help='Board height in cells')
        
        args = parser.parse_args()
        
        # Apply command-line arguments if provided
        if args.config:
            config.config_file = Path(args.config)
            # Load configuration from file
            config.load_from_file(config.config_file)
        
        if args.board_type:
            config.board_type = BoardType.from_string(args.board_type)
        
        if args.silent:
            config.silent = True
            
        if args.audio:
            config.silent = False
        
        if args.generations:
            config.generations = GenerationLimit(args.generations)
        
        if args.step_delay:
            config.step_delay_ms = args.step_delay
        
        if args.tempo:
            config.tempo_bpm = args.tempo
            
        # Apply audio settings from command line
        if args.note_duration_ms:
            config.note_duration_ms = args.note_duration_ms
            
        if args.gap_ms:
            config.gap_ms = args.gap_ms
            
        if args.chord_duration_ms:
            config.chord_duration_ms = args.chord_duration_ms
            
        if args.initial_delay_ms:
            config.initial_delay_ms = args.initial_delay_ms
            
        if args.detect_chords is not None:
            config.detect_chords = args.detect_chords
            
        if args.volume:
            config.volume = args.volume
            
        if args.pitch_shift is not None:
            config.pitch_shift = args.pitch_shift
            
        # Apply random board settings
        if args.alive_probability:
            config.alive_probability = args.alive_probability
            
        # Apply board dimensions
        if args.board_height:
            config.board_height = args.board_height
        
        # Apply environment variables if present (standardized format)
        if 'CONWAYS_STEINWAY_BOARD_TYPE' in os.environ:
            config.board_type = BoardType.from_string(os.environ['CONWAYS_STEINWAY_BOARD_TYPE'])
        
        if 'CONWAYS_STEINWAY_SILENT' in os.environ:
            value = os.environ['CONWAYS_STEINWAY_SILENT'].lower()
            config.silent = value in ('1', 'true', 'yes', 'on')
        
        if 'CONWAYS_STEINWAY_GENERATIONS' in os.environ:
            config.generations = GenerationLimit(os.environ['CONWAYS_STEINWAY_GENERATIONS'])
        
        if 'CONWAYS_STEINWAY_STEP_DELAY' in os.environ:
            config.step_delay_ms = int(os.environ['CONWAYS_STEINWAY_STEP_DELAY'])
        
        if 'CONWAYS_STEINWAY_TEMPO' in os.environ:
            config.tempo_bpm = float(os.environ['CONWAYS_STEINWAY_TEMPO'])
            
        # Audio settings from environment variables
        if 'CONWAYS_STEINWAY_NOTE_DURATION' in os.environ:
            config.note_duration_ms = int(os.environ['CONWAYS_STEINWAY_NOTE_DURATION'])
            
        if 'CONWAYS_STEINWAY_GAP' in os.environ:
            config.gap_ms = int(os.environ['CONWAYS_STEINWAY_GAP'])
            
        if 'CONWAYS_STEINWAY_CHORD_DURATION' in os.environ:
            config.chord_duration_ms = int(os.environ['CONWAYS_STEINWAY_CHORD_DURATION'])
            
        if 'CONWAYS_STEINWAY_INITIAL_DELAY' in os.environ:
            config.initial_delay_ms = int(os.environ['CONWAYS_STEINWAY_INITIAL_DELAY'])
            
        if 'CONWAYS_STEINWAY_DETECT_CHORDS' in os.environ:
            value = os.environ['CONWAYS_STEINWAY_DETECT_CHORDS'].lower()
            config.detect_chords = value in ('1', 'true', 'yes', 'on')
            
        if 'CONWAYS_STEINWAY_VOLUME' in os.environ:
            config.volume = float(os.environ['CONWAYS_STEINWAY_VOLUME'])
            
        if 'CONWAYS_STEINWAY_PITCH_SHIFT' in os.environ:
            value = os.environ['CONWAYS_STEINWAY_PITCH_SHIFT'].lower()
            config.pitch_shift = value in ('1', 'true', 'yes', 'on')
            
        # Random board settings from environment variables
        if 'CONWAYS_STEINWAY_ALIVE_PROBABILITY' in os.environ:
            config.alive_probability = float(os.environ['CONWAYS_STEINWAY_ALIVE_PROBABILITY'])
            
        # Board dimensions from environment variables
        if 'CONWAYS_STEINWAY_BOARD_HEIGHT' in os.environ:
            config.board_height = int(os.environ['CONWAYS_STEINWAY_BOARD_HEIGHT'])
        
        return config
    
    def load_from_file(self, file_path: Path) -> None:
        """Load configuration from a properties file using the jproperties library"""
        try:
            if not file_path.exists():
                raise FileNotFoundError(f"Config file not found: {file_path}")
            
            # Use jproperties to parse the file
            properties = Properties()
            with open(file_path, 'rb') as f:
                properties.load(f)
            
            # Get raw file contents to check for presence of 'silent' key
            with open(file_path, 'r') as f:
                raw_content = f.read()
                self.silent = 'silent' in raw_content
            
            # Apply core configuration values from file
            if 'board.type' in properties:
                board_type_val = properties.get('board.type').data
                self.board_type = BoardType.from_string(board_type_val)
            
            if 'generations' in properties:
                generations_val = properties.get('generations').data
                self.generations = GenerationLimit(generations_val)
            
            if 'step.delay.ms' in properties:
                delay_val = properties.get('step.delay.ms').data
                self.step_delay_ms = int(delay_val)
            
            if 'tempo.bpm' in properties:
                tempo_val = properties.get('tempo.bpm').data
                self.tempo_bpm = float(tempo_val)
                
            # Load audio configuration
            if 'audio.note.duration.ms' in properties:
                self.note_duration_ms = int(properties.get('audio.note.duration.ms').data)
                
            if 'audio.gap.ms' in properties:
                self.gap_ms = int(properties.get('audio.gap.ms').data)
                
            if 'audio.chord.duration.ms' in properties:
                self.chord_duration_ms = int(properties.get('audio.chord.duration.ms').data)
                
            if 'audio.initial.delay.ms' in properties:
                self.initial_delay_ms = int(properties.get('audio.initial.delay.ms').data)
                
            if 'audio.detect.chords' in properties:
                value = properties.get('audio.detect.chords').data.lower()
                self.detect_chords = value in ('true', 'yes', 'on', '1')
                    
            # Load random board configuration
            if 'random.alive.probability' in properties:
                prob_val = properties.get('random.alive.probability').data
                self.alive_probability = float(prob_val)
            
            # Additional audio settings
            if 'audio.volume' in properties:
                self.volume = float(properties.get('audio.volume').data)
            elif 'volume' in properties:
                self.volume = float(properties.get('volume').data)
                
            if 'audio.pitch.shift' in properties:
                value = properties.get('audio.pitch.shift').data.lower()
                self.pitch_shift = value in ('true', 'yes', 'on', '1')
            elif 'pitch.shift' in properties:
                value = properties.get('pitch.shift').data.lower()
                self.pitch_shift = value in ('true', 'yes', 'on', '1')
            
            # Apply board height if present
            if 'board.height' in properties:
                self.board_height = int(properties.get('board.height').data)
                
        except Exception as e:
            print(f"Error loading config file: {e}")
            
    def __str__(self) -> str:
        """String representation of the configuration"""
        result = (
            f"Config:\n"
            f"  Board Type: {self.board_type.value}\n"
            f"  Silent Mode: {self.silent}\n"
            f"  Generations: {self.generations}\n"
            f"  Step Delay: {self.step_delay_ms}ms\n"
            f"  Tempo: {self.tempo_bpm if self.tempo_bpm else 'Not set'}\n"
            f"  Board: {self.board_width}×{self.board_height}\n"
        )
        
        # Audio settings
        result += (f"  Audio Settings:\n"
                  f"    Note Duration: {self.note_duration_ms}ms\n"
                  f"    Chord Duration: {self.chord_duration_ms}ms\n"
                  f"    Gap Between Notes: {self.gap_ms}ms\n"
                  f"    Volume: {self.volume}\n"
                  f"    Detect Chords: {self.detect_chords}\n"
                  f"    Pitch Shift: {self.pitch_shift}\n")
        
        # Random board settings
        if self.board_type == BoardType.RANDOM:
            result += f"  Random Board: {self.alive_probability*100:.1f}% alive cells\n"
            
        return result
        
    def print_config(self) -> None:
        """Print the configuration to the console"""
        print(str(self))
        
    def get_effective_delay(self) -> int:
        """Get the effective delay in milliseconds based on configuration"""
        if self.tempo_bpm is not None:
            # Convert tempo in BPM to delay in ms
            # 60000 ms in a minute / BPM = ms per beat
            return int(60000 / self.tempo_bpm)
        return self.step_delay_ms

def get_config_path() -> Path:
    """Get the path to the config directory"""
    # Find config relative to project root
    base_path = Path(__file__).resolve().parent.parent
    config_path = base_path / "config"
    
    if not config_path.exists():
        # Fallback to assuming we're in the project root
        config_path = Path('.').resolve() / "config"
    
    return config_path

def get_default_config_file() -> Path:
    """Get the default config file path"""
    return get_config_path() / 'conways_steinway.properties'

def load_config() -> Config:
    """Load configuration from command-line args, environment variables, and file"""
    config = Config.from_args_and_env()
    
    # If no config file was specified, try the default location
    if config.config_file is None:
        default_config = get_default_config_file()
        if default_config.exists():
            config.config_file = default_config
            config.load_from_file(default_config)
    
    return config
