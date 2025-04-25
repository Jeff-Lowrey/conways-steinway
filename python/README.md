# Python Backend for Conway's Steinway.

## Design
This is the Python implementation of the backend for Conway's Steinway.

It should have the following components:

### A class to execute the Game Of Life on a board that's at least 40 cells high, and exactly 88 cells wide.

This exposes a method to get the bottom row of the board which
  1. Removes the bottom row and returns the bottom row
  2. Adds a new row to the top
  3. Calculates the next generation.

### A class to play the piano notes.

This exposes a method to accept a row from the Game of Life board.

  1. Iterates over the elements in the row
  2. If the cell is Alive
     * Plays a note that corresponds to the piano key in the same left-to-right position as the cell.
  3. If the cell is Dead or Empty
     * Does not play a note.

### Example
If a given row has live cells at positions 5, 11, 45, and 82 - the piano keys at the same positions will be played simultaneously.

## Decisions to be made:

1. How will a single row be represented?
   * A list of 88 values with state represented as [-1,0,1] (Dead, Empty, Alive)? 
   * A list of 88 values with state represented as Boolean (Play or don't Play)?
   * A list of ints representing the keys to actually play? (i.e. a list of the positions of all Alive Cells)
2. How will we implement a controller to run the game?
3. How will we display the board and rows during testing?

