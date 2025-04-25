#!/bin/python3

'''
This module will hold a class to initialize and execute an individual Game of Life board. 
It is expected to be thread-safe.

'''

class Row:
    # This represents a single row.
    # It does not have to be the same data structure that the board is built
    # out of.    

    def __init__(self, row=[]):
        # setting the default for row to a list is just an example.
        self.row = row

    def get_row(self):
        return self.row

class Life:

    def __init__(self):
        # Initialize any necessary variables or data structures here
        pass  # Add initialization code as needed
    
    def new_board(self, width=88, height=40):
        #create a new board with the given width and height
        pass
    
    def get_row(self):
        # pop the last row
        # do something to create a Row object from that row
        # add a new row to the top of the board
        # calculate the next generation
        # return the new row object
        # 
        pass