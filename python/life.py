#!/bin/python3

'''
This module holds a class to initialize and execute an individual Game of Life board.
It is expected to be thread-safe.
'''

import random
import copy
import hashlib
import struct

# Constants
BOARD_WIDTH = 88  # Default width (matching piano keys)
BOARD_HEIGHT = 40  # Default height


class Cell:
    """
    Represents a cell state in Conway's Game of Life.
    """
    DEAD = 0
    ALIVE = 1


class Row:
    """
    Represents a single row of cells in the Game of Life.
    A live cell is represented by Cell.ALIVE, a dead cell by Cell.DEAD.
    """
    def __init__(self, cells=None, width=BOARD_WIDTH):
        if cells is None:
            self.cells = [Cell.DEAD] * width
        else:
            self.cells = cells

    def get_cells(self):
        return self.cells

    def set_cell(self, index, value):
        if 0 <= index < len(self.cells):
            self.cells[index] = value

    def __len__(self):
        return len(self.cells)


class Life:
    """
    Implements Conway's Game of Life with a configurable board size.
    Board width is always 88 cells wide (matching piano keys) and height is configurable.
    """
    def __init__(self, width=BOARD_WIDTH, height=BOARD_HEIGHT):
        # Always use exactly 88 cells for board width (matching piano keys)
        # This is a fixed requirement and cannot be changed
        self.width = BOARD_WIDTH
        self.height = height
        self.board = []
        self.generation = 0
        
        # Initialize with an empty board
        for _ in range(height):
            self.board.append(Row([Cell.DEAD] * width))
    
    def new_random_board(self, width=BOARD_WIDTH, height=BOARD_HEIGHT, alive_probability=0.2):
        """
        Create a new board with the given height.
        Initializes the board with random live and dead cells.
        Board width is fixed at exactly 88 cells to match piano keys.
        """
        # Width is always 88 cells (matching piano keys)
        self.width = BOARD_WIDTH
        self.height = height
        self.generation = 0
        
        # Create a new board with random cells
        board = []
        for _ in range(height):
            row_cells = [Cell.ALIVE if random.random() < alive_probability else Cell.DEAD 
                         for _ in range(width)]
            board.append(Row(row_cells))
        
        self.board = board
        return self
    
    def from_pattern(self, pattern):
        """
        Initialize the board from a list of strings representing cell patterns.
        Each character in the string represents a cell:
        'O', 'X', '*' for alive cells, any other character for dead cells.
        """
        # Reset the board to all dead cells
        self.board = []
        for _ in range(self.height):
            self.board.append(Row([Cell.DEAD] * self.width))
        
        self.generation = 0
        
        # Set alive cells based on the pattern
        for row_idx, row_str in enumerate(pattern):
            if row_idx >= self.height:
                break
                
            for col_idx, ch in enumerate(row_str):
                if col_idx >= self.width:
                    break
                    
                if ch in ('O', 'X', '*'):
                    self.set_cell(row_idx, col_idx, Cell.ALIVE)
        
        return self
        
    def get_row(self):
        """
        Removes the last row from the board, adds a new empty row at the top,
        calculates the next generation, and returns the removed row.
        
        This is a simplified version. Use get_bottom_row_and_advance for more control.
        """
        # Get the bottom row indices of live cells
        bottom_row_keys = self.get_bottom_row_and_advance()
        
        # Create a Row object with only the live cells marked
        result_row = Row([Cell.DEAD] * self.width)
        for idx in bottom_row_keys:
            result_row.set_cell(idx, Cell.ALIVE)
            
        return result_row
    
    def get_bottom_row_and_advance(self):
        """
        Gets the indices of live cells in the bottom row,
        shifts the entire board down, adds a new random row at the top,
        and advances to the next generation.
        
        Returns a list of indices (0-based) of live cells in the bottom row.
        """
        # Get indices of live cells in the bottom row
        bottom_row_keys = []
        for col in range(self.width):
            if self.is_cell_alive(self.height - 1, col):
                bottom_row_keys.append(col)
        
        # Shift the board down one row
        for row in range(self.height - 1, 0, -1):
            for col in range(self.width):
                cell_value = self.board[row - 1].get_cells()[col]
                self.board[row].set_cell(col, cell_value)
        
        # Clear the top row
        for col in range(self.width):
            self.board[0].set_cell(col, Cell.DEAD)
            
        # Add random cells to the top row
        self.add_random_top_row()
        
        # Calculate the next generation
        self.next_generation()
        
        return bottom_row_keys
    
    def add_random_top_row(self):
        """
        Adds random live cells to the top row of the board.
        Uses the current generation as a seed for deterministic randomness.
        """
        # Create a hash of the current generation as a seed
        seed = hashlib.md5(str(self.generation).encode()).digest()
        seed_int = struct.unpack('Q', seed[:8])[0]  # Convert first 8 bytes to integer
        
        # Use LCG algorithm similar to Rust implementation
        rng_state = seed_int
        for col in range(self.width):
            # Linear Congruential Generator parameters
            a = 1664525
            c = 1013904223
            m = 2**32
            
            # Update RNG state
            rng_state = (a * rng_state + c) % m
            
            # Add a live cell with 1/5 probability (similar to Rust)
            if rng_state % 5 == 0:
                self.board[0].set_cell(col, Cell.ALIVE)
    
    def next_generation(self):
        """
        Calculates the next generation of the board based on Conway's Game of Life rules:
        1. Any live cell with fewer than two live neighbors dies (underpopulation)
        2. Any live cell with two or three live neighbors survives
        3. Any live cell with more than three live neighbors dies (overpopulation)
        4. Any dead cell with exactly three live neighbors becomes a live cell (reproduction)
        """
        # Create a copy of the current board
        new_board = copy.deepcopy(self.board)
        
        for row_idx in range(self.height):
            for col_idx in range(self.width):
                # Count the number of live neighbors
                live_neighbors = self.count_live_neighbors(row_idx, col_idx)
                
                # Apply Conway's Game of Life rules
                if self.is_cell_alive(row_idx, col_idx):
                    # Cell is alive
                    if live_neighbors < 2 or live_neighbors > 3:
                        # Die due to underpopulation or overpopulation
                        new_board[row_idx].set_cell(col_idx, Cell.DEAD)
                else:
                    # Cell is dead
                    if live_neighbors == 3:
                        # Become alive due to reproduction
                        new_board[row_idx].set_cell(col_idx, Cell.ALIVE)
        
        # Update the board
        self.board = new_board
        self.generation += 1
    
    def count_live_neighbors(self, row, col):
        """
        Count the number of live neighbors for a cell at the given row and column.
        """
        count = 0
        
        # Check all 8 neighboring cells
        for dr in [-1, 0, 1]:
            for dc in [-1, 0, 1]:
                # Skip the cell itself
                if dr == 0 and dc == 0:
                    continue
                
                # Calculate the neighbor's position with boundary checking
                nr = row + dr
                nc = col + dc
                
                # Check boundaries (using edge behavior matching the Rust implementation)
                if 0 <= nr < self.height and 0 <= nc < self.width:
                    if self.is_cell_alive(nr, nc):
                        count += 1
        
        return count
    
    def is_cell_alive(self, row, col):
        """
        Check if a cell at the given row and column is alive.
        """
        if 0 <= row < self.height and 0 <= col < self.width:
            return self.board[row].get_cells()[col] == Cell.ALIVE
        return False
    
    def set_cell(self, row, col, value):
        """
        Set the value of a cell at the given row and column.
        """
        if 0 <= row < self.height and 0 <= col < self.width:
            self.board[row].set_cell(col, value)
    
    def get_cell(self, row, col):
        """
        Get the value of a cell at the given row and column.
        Returns Cell.DEAD if out of bounds.
        """
        if 0 <= row < self.height and 0 <= col < self.width:
            return self.board[row].get_cells()[col]
        return Cell.DEAD
    
    def get_board(self):
        """
        Return a representation of the current board as a list of lists.
        """
        return [row.get_cells() for row in self.board]
    
    def get_generation(self):
        """
        Return the current generation number.
        """
        return self.generation
    
    def __str__(self):
        """
        Return a string representation of the board.
        """
        result = f"Generation: {self.generation}\n"
        result += f"Piano Keys: 1-{self.width} (left to right)\n"
        result += "=" * (self.width + 4) + "\n"
        
        for row in self.board:
            result += "| "
            for cell in row.get_cells():
                if cell == Cell.ALIVE:
                    result += "O"
                else:
                    result += "."
            result += " |\n"
        
        result += "=" * (self.width + 4)
        return result