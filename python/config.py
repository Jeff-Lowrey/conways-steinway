#!/bin/python3

"""
Configuration management for Conway's Steinway.
Handles command-line arguments, environment variables, and configuration files.
"""

import argparse
import os
import json
from enum import Enum
from pathlib import Path
from typing import Optional, Union


class BoardType(Enum):
    """
    Enum for different board initialization types.
    """
    RANDOM = "random"
    STATIC = "static"
    FUR_ELISE = "fur-elise"


class GenerationLimit:
    """
    Class to represent generation limit, either a specific number or unlimited.
    """
    def __init__(self, limit: Optional[int] = None):
        self.limit = limit

    @property
    def is_unlimited(self) -> bool:
        return self.limit is None

    def __repr__(self) -> str:
        return f"Unlimited" if self.is_unlimited else f"Limited({self.limit})"


class Config:
    """
    Configuration class for Conway's Steinway.
    Handles command-line arguments, environment variables, and configuration files.
    """
    def __init__(self):
        self.board_type: BoardType = BoardType.RANDOM
        self.silent: bool = False  # Audio is enabled by default (silent=False)
        self.generations: GenerationLimit = GenerationLimit()  # Unlimited by default
        self.step_delay_ms: int = 200
        self.tempo_bpm: Optional[float] = None
        self.config_file: Optional[Path] = None

    @classmethod
    def from_args_and_env(cls) -> 'Config':
        """
        Create a Config instance from command-line arguments and environment variables.
        """
        config = cls()
        
        # Load from environment variables first
        config._load_from_env()
        
        # Parse command line arguments
        parser = argparse.ArgumentParser(
            description="A musical interpretation of Conway's Game of Life using piano sounds"
        )
        
        parser.add_argument(
            "-c", "--config",
            dest="config",
            help="Configuration file path",
            type=str
        )
        
        parser.add_argument(
            "-b", "--board-type",
            dest="board_type",
            help="Board initialization type",
            choices=["random", "static", "fur-elise"],
            default=None
        )
        
        # Using the standard approach for boolean flags:
        # - silent flag disables audio (audio is enabled by default)
        parser.add_argument(
            "-s", "--silent",
            dest="silent",
            help="Disable audio output (audio is enabled by default)",
            action="store_true"
        )
        
        parser.add_argument(
            "-g", "--generations",
            dest="generations",
            help="Number of generations to run (0 for unlimited)",
            type=int,
            default=None
        )
        
        parser.add_argument(
            "-d", "--delay",
            dest="delay",
            help="Delay between steps in milliseconds",
            type=int,
            default=None
        )
        
        parser.add_argument(
            "-t", "--tempo",
            dest="tempo",
            help="Musical tempo in beats per minute (overrides delay)",
            type=float,
            default=None
        )
        
        args = parser.parse_args()
        
        # Load from config file if specified
        if args.config:
            config_path = Path(args.config)
            config.config_file = config_path
            config._load_from_file(config_path)
        
        # Override with command line arguments
        if args.board_type:
            config.board_type = BoardType(args.board_type)
        
        # Set silent mode if the flag is present
        if args.silent:
            config.silent = True
        
        if args.generations is not None:
            config.generations = (
                GenerationLimit() if args.generations == 0 
                else GenerationLimit(args.generations)
            )
        
        if args.delay is not None:
            config.step_delay_ms = args.delay
        
        if args.tempo is not None:
            config.tempo_bpm = args.tempo
        
        return config

    def _load_from_env(self) -> None:
        """
        Load configuration from environment variables.
        """
        # Board type
        if board_type_env := os.environ.get("CONWAYS_STEINWAY_BOARD_TYPE"):
            try:
                self.board_type = BoardType(board_type_env.lower())
            except ValueError:
                pass  # Invalid value, keep default
        
        # Audio enabled by default, check for silent mode in environment
        if silent_env := os.environ.get("CONWAYS_STEINWAY_SILENT"):
            self.silent = silent_env.lower() in ("1", "true", "yes")
        
        # Generations
        if generations_env := os.environ.get("CONWAYS_STEINWAY_GENERATIONS"):
            try:
                generations = int(generations_env)
                self.generations = (
                    GenerationLimit() if generations == 0 
                    else GenerationLimit(generations)
                )
            except ValueError:
                pass  # Invalid value, keep default
        
        # Step delay
        if delay_env := os.environ.get("CONWAYS_STEINWAY_DELAY"):
            try:
                self.step_delay_ms = int(delay_env)
            except ValueError:
                pass  # Invalid value, keep default
        
        # Tempo
        if tempo_env := os.environ.get("CONWAYS_STEINWAY_TEMPO"):
            try:
                self.tempo_bpm = float(tempo_env)
            except ValueError:
                pass  # Invalid value, keep default

    def _load_from_file(self, path: Path) -> None:
        """
        Load configuration from a JSON file.
        """
        if path.exists():
            try:
                with open(path, 'r') as f:
                    config_data = json.load(f)
                
                # Board type
                if board_type := config_data.get("board_type"):
                    try:
                        self.board_type = BoardType(board_type)
                    except ValueError:
                        pass  # Invalid value, keep default
                
                # Check for silent mode in config file
                if "silent" in config_data:
                    self.silent = bool(config_data["silent"])
                # For backward compatibility
                elif "audio_enabled" in config_data:
                    self.silent = not bool(config_data["audio_enabled"])
                
                # Generations
                if generations := config_data.get("generations"):
                    if isinstance(generations, dict):
                        if "Limited" in generations:
                            self.generations = GenerationLimit(generations["Limited"])
                        elif "Unlimited" in generations:
                            self.generations = GenerationLimit()
                    elif generations == "Unlimited":
                        self.generations = GenerationLimit()
                    elif isinstance(generations, int):
                        self.generations = GenerationLimit(generations)
                
                # Step delay
                if "step_delay_ms" in config_data:
                    self.step_delay_ms = int(config_data["step_delay_ms"])
                
                # Tempo
                if "tempo_bpm" in config_data:
                    tempo = config_data["tempo_bpm"]
                    self.tempo_bpm = float(tempo) if tempo is not None else None
                
            except (json.JSONDecodeError, TypeError, ValueError) as e:
                print(f"Error loading config file: {e}")

    def save_to_file(self, path: Path) -> None:
        """
        Save configuration to a JSON file.
        """
        config_data = {
            "board_type": self.board_type.value,
            "silent": self.silent,
            "generations": repr(self.generations),
            "step_delay_ms": self.step_delay_ms,
            "tempo_bpm": self.tempo_bpm
        }
        
        with open(path, 'w') as f:
            json.dump(config_data, f, indent=4)

    @staticmethod
    def tempo_to_delay_ms(bpm: float) -> int:
        """
        Convert tempo in BPM to delay in milliseconds.
        Uses eighth note subdivision for musical feel.
        """
        # BPM = beats per minute, so ms per beat = (60 * 1000) / BPM
        # Using eighth note subdivision: delay = (60000 / BPM) / 2
        delay = (60000.0 / bpm) / 2.0
        return round(delay)

    def get_effective_delay(self) -> int:
        """
        Get the effective delay in milliseconds, considering tempo if set.
        """
        if self.tempo_bpm is not None:
            return self.tempo_to_delay_ms(self.tempo_bpm)
        return self.step_delay_ms

    def print_config(self) -> None:
        """
        Print the current configuration.
        """
        print("Configuration:")
        print(f"  Board Type: {self.board_type.name}")
        print(f"  Silent Mode: {self.silent}")
        print(f"  Generations: {self.generations}")
        
        if self.tempo_bpm is not None:
            effective_delay = self.get_effective_delay()
            print(f"  Tempo: {self.tempo_bpm:.1f} BPM ({effective_delay}ms per step)")
        else:
            print(f"  Step Delay: {self.step_delay_ms}ms")
        
        if self.config_file:
            print(f"  Config File: {self.config_file}")
        print()


if __name__ == "__main__":
    # Test the configuration
    config = Config.from_args_and_env()
    config.print_config()