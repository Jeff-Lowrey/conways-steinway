#!/bin/python3

"""
This module contains functions to create and manipulate Game of Life boards
with various patterns and configurations.
"""

from life import Life, Cell, BOARD_WIDTH, BOARD_HEIGHT


class GameBoard:
    """
    A utility class for creating and manipulating Game of Life boards.
    """
    
    @staticmethod
    def create_random_board():
        """
        Create a new Game of Life board with random cells.
        """
        game = Life()
        return game.new_random_board(alive_probability=0.25)  # About 25% alive
    
    @staticmethod
    def create_complex_board():
        """
        Create a new Game of Life board with various interesting patterns.
        """
        game = Life()
        
        # Add multiple gliders at different positions
        GameBoard.create_glider(game, 0, 0)
        GameBoard.create_glider(game, 5, 20)
        GameBoard.create_glider(game, 10, 40)
        GameBoard.create_glider(game, 2, 60)
        GameBoard.create_glider(game, 15, 10)
        GameBoard.create_glider(game, 8, 70)
        GameBoard.create_glider(game, 20, 25)
        GameBoard.create_glider(game, 25, 50)
        GameBoard.create_glider(game, 30, 15)
        GameBoard.create_glider(game, 1, 75)
        GameBoard.create_glider(game, 22, 8)
        GameBoard.create_glider(game, 28, 65)
        
        # Add oscillators
        GameBoard.create_blinker(game, 5, 5)
        GameBoard.create_toad(game, 12, 30)
        GameBoard.create_beacon(game, 25, 5)
        GameBoard.create_pentadecathlon(game, 1, 34)
        
        # Add still lifes
        GameBoard.create_block(game, 15, 75)
        GameBoard.create_beehive(game, 10, 50)
        GameBoard.create_loaf(game, 18, 45)
        GameBoard.create_boat(game, 34, 25)
        
        # Add methuselah patterns
        GameBoard.create_r_pentomino(game, 6, 55)
        GameBoard.create_diehard(game, 18, 15)
        GameBoard.create_acorn(game, 32, 35)
        
        # Add spaceships
        GameBoard.create_lwss(game, 35, 60)
        
        return game
    
    @staticmethod
    def from_pattern(pattern):
        """
        Create a new Game of Life board from a pattern defined as a list of strings.
        """
        game = Life()
        return game.from_pattern(pattern)
    
    @staticmethod
    def get_bottom_row_and_advance(game):
        """
        Gets the indices of live cells in the bottom row,
        shifts the entire board down, adds a new random row at the top,
        and advances to the next generation.
        
        Returns a list of indices (0-based) of live cells in the bottom row.
        """
        return game.get_bottom_row_and_advance()
    
    @staticmethod
    def add_random_row(game):
        """
        Adds random live cells to the top row of the board.
        """
        game.add_random_top_row()
    
    # Still Life patterns
    @staticmethod
    def create_block(game, row, col):
        """Create a 2x2 block still life pattern."""
        game.set_cell(row, col, Cell.ALIVE)
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 1, Cell.ALIVE)
    
    @staticmethod
    def create_beehive(game, row, col):
        """Create a beehive still life pattern."""
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row, col + 2, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 3, Cell.ALIVE)
        game.set_cell(row + 2, col + 1, Cell.ALIVE)
        game.set_cell(row + 2, col + 2, Cell.ALIVE)
    
    @staticmethod
    def create_loaf(game, row, col):
        """Create a loaf still life pattern."""
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row, col + 2, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 3, Cell.ALIVE)
        game.set_cell(row + 2, col + 1, Cell.ALIVE)
        game.set_cell(row + 2, col + 3, Cell.ALIVE)
        game.set_cell(row + 3, col + 2, Cell.ALIVE)
    
    @staticmethod
    def create_boat(game, row, col):
        """Create a boat still life pattern."""
        game.set_cell(row, col, Cell.ALIVE)
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 2, Cell.ALIVE)
        game.set_cell(row + 2, col + 1, Cell.ALIVE)
    
    # Oscillator patterns
    @staticmethod
    def create_blinker(game, row, col):
        """Create a blinker oscillator pattern."""
        game.set_cell(row, col, Cell.ALIVE)
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row, col + 2, Cell.ALIVE)
    
    @staticmethod
    def create_toad(game, row, col):
        """Create a toad oscillator pattern."""
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row, col + 2, Cell.ALIVE)
        game.set_cell(row, col + 3, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 1, Cell.ALIVE)
        game.set_cell(row + 1, col + 2, Cell.ALIVE)
    
    @staticmethod
    def create_beacon(game, row, col):
        """Create a beacon oscillator pattern."""
        game.set_cell(row, col, Cell.ALIVE)
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 1, Cell.ALIVE)
        game.set_cell(row + 2, col + 2, Cell.ALIVE)
        game.set_cell(row + 2, col + 3, Cell.ALIVE)
        game.set_cell(row + 3, col + 2, Cell.ALIVE)
        game.set_cell(row + 3, col + 3, Cell.ALIVE)
    
    @staticmethod
    def create_pulsar(game, row, col):
        """Create a pulsar oscillator pattern."""
        # Top cross
        for i in range(3):
            game.set_cell(row + 2, col + 4 + i, Cell.ALIVE)
            game.set_cell(row + 2, col + 10 + i, Cell.ALIVE)
        
        # Vertical lines
        for i in range(3):
            game.set_cell(row + 4 + i, col + 2, Cell.ALIVE)
            game.set_cell(row + 4 + i, col + 7, Cell.ALIVE)
            game.set_cell(row + 4 + i, col + 9, Cell.ALIVE)
            game.set_cell(row + 4 + i, col + 14, Cell.ALIVE)
            
            game.set_cell(row + 10 + i, col + 2, Cell.ALIVE)
            game.set_cell(row + 10 + i, col + 7, Cell.ALIVE)
            game.set_cell(row + 10 + i, col + 9, Cell.ALIVE)
            game.set_cell(row + 10 + i, col + 14, Cell.ALIVE)
        
        # Horizontal lines
        for i in range(3):
            game.set_cell(row + 7, col + 4 + i, Cell.ALIVE)
            game.set_cell(row + 7, col + 10 + i, Cell.ALIVE)
            game.set_cell(row + 9, col + 4 + i, Cell.ALIVE)
            game.set_cell(row + 9, col + 10 + i, Cell.ALIVE)
        
        # Bottom cross
        for i in range(3):
            game.set_cell(row + 14, col + 4 + i, Cell.ALIVE)
            game.set_cell(row + 14, col + 10 + i, Cell.ALIVE)
    
    @staticmethod
    def create_pentadecathlon(game, row, col):
        """Create a pentadecathlon oscillator pattern."""
        # Central line
        for i in range(8):
            game.set_cell(row + i, col + 1, Cell.ALIVE)
        
        # End pieces
        game.set_cell(row + 3, col, Cell.ALIVE)
        game.set_cell(row + 3, col + 2, Cell.ALIVE)
        game.set_cell(row + 4, col, Cell.ALIVE)
        game.set_cell(row + 4, col + 2, Cell.ALIVE)
    
    # Spaceship patterns
    @staticmethod
    def create_glider(game, row, col):
        """Create a glider spaceship pattern."""
        game.set_cell(row, col + 2, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 2, Cell.ALIVE)
        game.set_cell(row + 2, col + 1, Cell.ALIVE)
        game.set_cell(row + 2, col + 2, Cell.ALIVE)
    
    @staticmethod
    def create_lwss(game, row, col):
        """Create a lightweight spaceship pattern."""
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row, col + 4, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 2, col, Cell.ALIVE)
        game.set_cell(row + 2, col + 4, Cell.ALIVE)
        game.set_cell(row + 3, col, Cell.ALIVE)
        game.set_cell(row + 3, col + 1, Cell.ALIVE)
        game.set_cell(row + 3, col + 2, Cell.ALIVE)
        game.set_cell(row + 3, col + 3, Cell.ALIVE)
    
    @staticmethod
    def create_mwss(game, row, col):
        """Create a middleweight spaceship pattern."""
        game.set_cell(row, col + 2, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 4, Cell.ALIVE)
        game.set_cell(row + 2, col + 5, Cell.ALIVE)
        game.set_cell(row + 3, col, Cell.ALIVE)
        game.set_cell(row + 3, col + 5, Cell.ALIVE)
        game.set_cell(row + 4, col + 1, Cell.ALIVE)
        game.set_cell(row + 4, col + 2, Cell.ALIVE)
        game.set_cell(row + 4, col + 3, Cell.ALIVE)
        game.set_cell(row + 4, col + 4, Cell.ALIVE)
        game.set_cell(row + 4, col + 5, Cell.ALIVE)
    
    @staticmethod
    def create_hwss(game, row, col):
        """Create a heavyweight spaceship pattern."""
        game.set_cell(row, col + 2, Cell.ALIVE)
        game.set_cell(row, col + 3, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 5, Cell.ALIVE)
        game.set_cell(row + 2, col + 6, Cell.ALIVE)
        game.set_cell(row + 3, col, Cell.ALIVE)
        game.set_cell(row + 3, col + 6, Cell.ALIVE)
        game.set_cell(row + 4, col + 1, Cell.ALIVE)
        game.set_cell(row + 4, col + 2, Cell.ALIVE)
        game.set_cell(row + 4, col + 3, Cell.ALIVE)
        game.set_cell(row + 4, col + 4, Cell.ALIVE)
        game.set_cell(row + 4, col + 5, Cell.ALIVE)
        game.set_cell(row + 4, col + 6, Cell.ALIVE)
    
    # Methuselah patterns
    @staticmethod
    def create_r_pentomino(game, row, col):
        """Create an R-pentomino methuselah pattern."""
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row, col + 2, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 1, Cell.ALIVE)
        game.set_cell(row + 2, col + 1, Cell.ALIVE)
    
    @staticmethod
    def create_diehard(game, row, col):
        """Create a diehard methuselah pattern."""
        game.set_cell(row, col + 6, Cell.ALIVE)
        game.set_cell(row + 1, col, Cell.ALIVE)
        game.set_cell(row + 1, col + 1, Cell.ALIVE)
        game.set_cell(row + 2, col + 1, Cell.ALIVE)
        game.set_cell(row + 2, col + 5, Cell.ALIVE)
        game.set_cell(row + 2, col + 6, Cell.ALIVE)
        game.set_cell(row + 2, col + 7, Cell.ALIVE)
    
    @staticmethod
    def create_acorn(game, row, col):
        """Create an acorn methuselah pattern."""
        game.set_cell(row, col + 1, Cell.ALIVE)
        game.set_cell(row + 1, col + 3, Cell.ALIVE)
        game.set_cell(row + 2, col, Cell.ALIVE)
        game.set_cell(row + 2, col + 1, Cell.ALIVE)
        game.set_cell(row + 2, col + 4, Cell.ALIVE)
        game.set_cell(row + 2, col + 5, Cell.ALIVE)
        game.set_cell(row + 2, col + 6, Cell.ALIVE)
    
    # Gun patterns
    @staticmethod
    def create_gosper_glider_gun(game, row, col):
        """Create a Gosper glider gun pattern."""
        # Left block
        GameBoard.create_block(game, row + 5, col)
        
        # Left part
        game.set_cell(row + 3, col + 10, Cell.ALIVE)
        game.set_cell(row + 4, col + 10, Cell.ALIVE)
        game.set_cell(row + 5, col + 10, Cell.ALIVE)
        game.set_cell(row + 2, col + 11, Cell.ALIVE)
        game.set_cell(row + 6, col + 11, Cell.ALIVE)
        game.set_cell(row + 1, col + 12, Cell.ALIVE)
        game.set_cell(row + 7, col + 12, Cell.ALIVE)
        game.set_cell(row + 1, col + 13, Cell.ALIVE)
        game.set_cell(row + 7, col + 13, Cell.ALIVE)
        game.set_cell(row + 4, col + 14, Cell.ALIVE)
        game.set_cell(row + 2, col + 15, Cell.ALIVE)
        game.set_cell(row + 6, col + 15, Cell.ALIVE)
        game.set_cell(row + 3, col + 16, Cell.ALIVE)
        game.set_cell(row + 4, col + 16, Cell.ALIVE)
        game.set_cell(row + 5, col + 16, Cell.ALIVE)
        game.set_cell(row + 4, col + 17, Cell.ALIVE)
        
        # Right part
        game.set_cell(row + 1, col + 20, Cell.ALIVE)
        game.set_cell(row + 2, col + 20, Cell.ALIVE)
        game.set_cell(row + 3, col + 20, Cell.ALIVE)
        game.set_cell(row + 1, col + 21, Cell.ALIVE)
        game.set_cell(row + 2, col + 21, Cell.ALIVE)
        game.set_cell(row + 3, col + 21, Cell.ALIVE)
        game.set_cell(row, col + 22, Cell.ALIVE)
        game.set_cell(row + 4, col + 22, Cell.ALIVE)
        game.set_cell(row - 1, col + 24, Cell.ALIVE)
        game.set_cell(row, col + 24, Cell.ALIVE)
        game.set_cell(row + 4, col + 24, Cell.ALIVE)
        game.set_cell(row + 5, col + 24, Cell.ALIVE)
        
        # Right block
        GameBoard.create_block(game, row + 3, col + 34)
    
    @staticmethod
    def create_fur_elise_board():
        """
        Create a board configuration to play "Für Elise" melody.
        """
        game = Life()
        
        # Für Elise melody notes (piano key numbers, 1-88):
        # E5-D#5-E5-D#5-E5-B4-D5-C5-A4 (main phrase)
        # Piano keys: 52-51-52-51-52-47-50-49-45
        
        # Create patterns that will hit the bottom row to play these notes
        # Using careful timing with different pattern types and positions
        
        # E5 (key 52) - First note, immediate impact
        GameBoard.create_glider(game, 36, 51)  # Will reach bottom quickly
        
        # D#5 (key 51) - Second note
        GameBoard.create_blinker(game, 35, 50)  # Oscillates, hits on step 2
        
        # E5 (key 52) - Third note
        GameBoard.create_glider(game, 34, 51)  # Delayed glider
        
        # D#5 (key 51) - Fourth note
        GameBoard.create_toad(game, 32, 49)  # Toad pattern, hits step 4
        
        # E5 (key 52) - Fifth note
        GameBoard.create_glider(game, 30, 51)  # Another glider
        
        # B4 (key 47) - Sixth note
        GameBoard.create_r_pentomino(game, 25, 45)  # Long-term pattern
        
        # D5 (key 50) - Seventh note
        GameBoard.create_lwss(game, 28, 46)  # Spaceship moving toward key 50
        
        # C5 (key 49) - Eighth note
        GameBoard.create_beacon(game, 26, 47)  # Beacon oscillator
        
        # A4 (key 45) - Ninth note
        GameBoard.create_acorn(game, 20, 42)  # Acorn methuselah
        
        # Add some supporting patterns for rhythm and harmony
        GameBoard.create_block(game, 15, 40)  # Bass note stability
        GameBoard.create_block(game, 15, 55)  # High note stability
        
        # Add gliders that will create sustained notes
        GameBoard.create_glider(game, 10, 30)  # Lower register accompaniment
        GameBoard.create_glider(game, 8, 60)  # Higher register accompaniment
        
        # Create a "conductor" pattern - pentadecathlon for timing
        GameBoard.create_pentadecathlon(game, 5, 44)
        
        # Add some harmonic patterns
        GameBoard.create_beehive(game, 12, 35)  # Harmonic support
        GameBoard.create_loaf(game, 18, 65)  # Treble harmony
        
        # Second phrase preparation - more complex patterns
        GameBoard.create_diehard(game, 15, 20)  # Dies and creates space
        GameBoard.create_gosper_glider_gun(game, 2, 10)  # Continuous glider generation
        
        # Add patterns for the second phrase melody
        # C4-E4-A4-B4 sequence (keys 41-44-45-47)
        GameBoard.create_hwss(game, 22, 38)  # Heavy spaceship for C4
        GameBoard.create_mwss(game, 24, 41)  # Medium spaceship for E4
        GameBoard.create_glider(game, 26, 44)  # Glider for A4
        GameBoard.create_pulsar(game, 1, 30)  # Pulsar for complex timing
        
        return game
    
    @staticmethod
    def create_showcase_board():
        """
        Create a board with various patterns for demonstration.
        """
        game = Life()
        
        # Add various patterns across the board
        GameBoard.create_glider(game, 1, 1)
        GameBoard.create_block(game, 5, 10)
        GameBoard.create_blinker(game, 8, 15)
        GameBoard.create_toad(game, 12, 20)
        GameBoard.create_beacon(game, 16, 25)
        GameBoard.create_lwss(game, 20, 30)
        GameBoard.create_r_pentomino(game, 25, 35)
        GameBoard.create_acorn(game, 30, 40)
        
        # Add some still lifes
        GameBoard.create_beehive(game, 10, 50)
        GameBoard.create_loaf(game, 15, 55)
        GameBoard.create_boat(game, 20, 60)
        
        return game