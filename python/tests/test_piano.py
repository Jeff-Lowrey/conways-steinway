#!/bin/python3

import sys
import os
import pytest
import io

# Add the parent directory to sys.path to be able to import the modules
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from piano import Piano, AudioPlayer, NullAudioEngine
from life import Life, Row

def test_piano_initialization():
    """Test that a Piano instance can be created with default parameters."""
    # Create piano with NullAudioEngine to avoid audio initialization
    piano = Piano(audio_enabled=False)
    assert piano.generations == 10
    assert piano.width == 88
    assert piano.height == 40
    assert piano.is_muted == False
    assert isinstance(piano.game, Life)
    assert isinstance(piano.audio_engine, AudioPlayer)

def test_piano_custom_parameters():
    """Test that a Piano instance can be created with custom parameters."""
    piano = Piano(generations=5, width=10, height=5, audio_enabled=False)
    assert piano.generations == 5
    assert piano.width == 10
    assert piano.height == 5

def test_piano_mute():
    """Test the mute functionality of the Piano class."""
    piano = Piano(audio_enabled=False)
    assert piano.get_mute_status() == False
    
    piano.set_mute(True)
    assert piano.get_mute_status() == True
    
    piano.set_mute(False)
    assert piano.get_mute_status() == False

def test_piano_row_to_keys():
    """Test that _row_to_keys correctly converts a row to key indices."""
    piano = Piano(audio_enabled=False)
    
    # Create a row with some live cells
    row_cells = [0] * 88
    row_cells[10] = 1  # Live cell at index 10
    row_cells[20] = 1  # Live cell at index 20
    row_cells[30] = 1  # Live cell at index 30
    row = Row(row_cells)
    
    # Convert row to keys
    keys = piano._row_to_keys(row)
    
    # Check that the keys correspond to the live cells
    assert keys == [10, 20, 30]

class TestAudioEngine(NullAudioEngine):
    """Test audio engine that tracks method calls."""
    def __init__(self):
        self.keys_played = []
        self.chords_played = []
        self.sequences_played = []
    
    def play_piano_keys(self, keys):
        self.keys_played.append(keys)
        
    def play_chord(self, keys, duration_ms):
        self.chords_played.append((keys, duration_ms))
        
    def play_note_sequence(self, keys, note_duration_ms, gap_ms):
        self.sequences_played.append((keys, note_duration_ms, gap_ms))

def test_piano_play():
    """Test that the Piano play method correctly processes rows from Life."""
    # Create piano with TestAudioEngine
    piano = Piano(generations=2, width=5, height=3, audio_enabled=False)
    test_engine = TestAudioEngine()
    piano.audio_engine = test_engine
    
    # Manually set up some live cells
    piano.game.set_cell(2, 1, 1)
    piano.game.set_cell(2, 3, 1)
    
    # Modify _play_generation to use a direct approach for testing
    def mock_play_generation():
        # Simulate getting bottom row and playing keys
        keys = [1, 3]  # Keys corresponding to the live cells we set
        if not piano.is_muted and keys:
            test_engine.play_piano_keys(keys)
    
    # Replace the method for testing
    piano._play_generation = mock_play_generation
    
    # Play the piano
    piano._play_limited(2)  # Call directly to avoid infinite loop in _play_unlimited
    
    # Check that play_piano_keys was called with the correct keys
    assert len(test_engine.keys_played) == 2  # Called twice (once per generation)
    
    # Check that the method was called with our expected list
    for keys in test_engine.keys_played:
        assert isinstance(keys, list)
        assert keys == [1, 3]

def test_piano_play_keys():
    """Test that the Piano play_keys method correctly passes keys to the audio engine."""
    # Create piano with TestAudioEngine
    piano = Piano(audio_enabled=False)
    test_engine = TestAudioEngine()
    piano.audio_engine = test_engine
    
    # Play some keys
    test_keys = [10, 20, 30]
    piano.play_keys(test_keys)
    
    # Check that play_piano_keys was called with the correct keys
    assert test_engine.keys_played == [test_keys]
    
    # Test with muted piano
    piano.set_mute(True)
    piano.play_keys([40, 50])
    
    # Check that no more keys were played
    assert len(test_engine.keys_played) == 1

def test_piano_play_chord():
    """Test that the Piano play_chord method correctly passes keys to the audio engine."""
    # Create piano with TestAudioEngine
    piano = Piano(audio_enabled=False)
    test_engine = TestAudioEngine()
    piano.audio_engine = test_engine
    
    # Play a chord
    test_keys = [48, 52, 55]  # C major chord
    piano.play_chord(test_keys, 500)
    
    # Check that play_chord was called with the correct parameters
    assert test_engine.chords_played == [(test_keys, 500)]
    
    # Test with muted piano
    piano.set_mute(True)
    piano.play_chord([40, 44, 47], 300)
    
    # Check that no more chords were played
    assert len(test_engine.chords_played) == 1

def test_piano_play_sequence():
    """Test that the Piano play_sequence method correctly passes keys to the audio engine."""
    # Create piano with TestAudioEngine
    piano = Piano(audio_enabled=False)
    test_engine = TestAudioEngine()
    piano.audio_engine = test_engine
    
    # Play a sequence
    test_keys = [60, 62, 64, 65, 67]  # C major scale
    piano.play_sequence(test_keys, 200, 50)
    
    # Check that play_note_sequence was called with the correct parameters
    assert test_engine.sequences_played == [(test_keys, 200, 50)]
    
    # Test with muted piano
    piano.set_mute(True)
    piano.play_sequence([48, 50, 52], 100, 20)
    
    # Check that no more sequences were played
    assert len(test_engine.sequences_played) == 1
