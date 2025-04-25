#!/bin/python3

from life import Life, Row

'''

This module will hold a class to play piano notes based on a given input row.

It is expected to be thread-safe.

'''

class Piano:

    def __init__(self, stop_after=10):
        # Initialize any necessary variables or data structures here
        # stop_after needs a better name, it's the number of generations to play
        self.max_rows = stop_after
        self.mute = False
        pass  # Add initialization code as needed

    def play(self):
        # Execute the total number of rows allowed. 
        for x in range(self.max_rows):
            Life.get_row()
            # iterate over row elements
            # send to method to execute the notes
            # 
        pass

    def mute(self, mute=True):
        # Enable sound output - needed to ensure thread_safe/scalability.
        # Only one piano can be using the speakers at once.
        self.mute=mute
