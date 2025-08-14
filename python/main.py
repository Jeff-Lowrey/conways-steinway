#!/bin/python3

"""
Main application entry point for Conway's Steinway.
Processes configuration, initializes the game board, and plays music.
"""

import time

from piano import Piano
import sys

# Load configuration from config directory
from config_loader import ensure_config_in_path
ensure_config_in_path()

# Now we can import from the config module
from config import Config, BoardType, GenerationLimit
from game_board import GameBoard


def main():
    """
    Main function to run Conway's Steinway.
    Processes configuration, initializes the game, and plays music.
    """
    print("Conway's Steinway - Conway's Game of Life generating piano music")
    print("=" * 60)
    
    # Load configuration from command line, environment, and config file
    config = Config.from_args_and_env()
    config.print_config()
    
    # Determine the number of generations to play
    generations = None if not config.generations.is_limited else config.generations.limit
    
    # Create a piano with configured settings
    piano = Piano(generations=generations, audio_enabled=not config.silent)
    
    # Initialize the game board based on configuration
    if config.board_type == BoardType.FUR_ELISE:
        piano.game = GameBoard.create_fur_elise_board()
        print("Initialized with Für Elise board configuration")
    elif config.board_type == BoardType.STATIC:
        piano.game = GameBoard.create_showcase_board()
        print("Initialized with showcase board configuration")
    else:  # Default to random
        piano.game = GameBoard.create_random_board()
        print("Initialized with random board configuration")
    
    # Set delay from configuration
    delay_ms = config.get_effective_delay()
    piano.delay_ms = delay_ms
    
    # Display information about the performance
    if generations is None:
        print("Playing indefinitely (press Ctrl+C to stop)...")
    else:
        print(f"Playing for {generations} generations...")
    
    if config.tempo_bpm is not None:
        print(f"Tempo: {config.tempo_bpm:.1f} BPM ({delay_ms}ms per step)")
    else:
        print(f"Step delay: {delay_ms}ms")
    
    print("Each '♪' represents a key being played.")
    print("-" * 60)
    
    # Replace the standard output with a visual representation
    original_print = print
    
    def piano_print(message):
        if "Playing key" in message:
            try:
                key_num = int(message.split("Playing key ")[1])
                # Use different symbols based on the key position (bass, middle, treble)
                if key_num < 30:
                    original_print("♭", end="")
                elif key_num < 60:
                    original_print("♪", end="")
                else:
                    original_print("♯", end="")
            except (IndexError, ValueError):
                # Handle case where the message format doesn't match expected pattern
                original_print(message)
        elif "Playing keys:" in message:
            # Handle multiple keys being played
            original_print("♫", end="")
        else:
            original_print(message)
    
    # Replace the built-in print function temporarily
    import builtins
    builtins.print = piano_print
    
    try:
        # Play the piano
        piano.play()
    except KeyboardInterrupt:
        original_print("\nPerformance interrupted by user.")
    finally:
        # Restore the original print function
        builtins.print = original_print
    
    print("\n" + "-" * 60)
    print("Performance complete!")


if __name__ == "__main__":
    main()