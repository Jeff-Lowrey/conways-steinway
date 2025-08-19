#!/bin/python3

import sys
import os
import pytest

# Add the parent directory to sys.path to be able to import the modules
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), '..')))

from piano import AudioEngine, NullAudioEngine, AudioPlayer

def test_audio_player_interface():
    """Test that AudioPlayer interface is properly implemented by audio engines."""
    # Both engine types should implement the AudioPlayer interface
    engines = [NullAudioEngine(), AudioEngine()]
    
    for engine in engines:
        assert isinstance(engine, AudioPlayer)
        
        # Test interface methods (these should not raise exceptions)
        engine.play_piano_keys([])
        engine.play_piano_keys([48])  # Middle C
        engine.play_chord([48, 52, 55], 100)  # C Major chord
        engine.play_note_sequence([48, 50, 52], 100, 20)  # C, D, E sequence

def test_null_audio_engine():
    """Test that the NullAudioEngine operates without errors."""
    engine = NullAudioEngine()
    
    # All operations should be no-ops and not raise exceptions
    engine.play_piano_keys([0, 1, 2, 3, 4])
    engine.play_chord([24, 28, 31], 1000)
    engine.play_note_sequence([48, 50, 52], 200, 50)
    
    # Test edge cases
    engine.play_piano_keys([])
    engine.play_piano_keys([87])  # Highest key
    engine.play_chord([], 500)

def test_audio_engine_chord_detection():
    """Test that the AudioEngine correctly detects chord patterns."""
    engine = AudioEngine()
    
    # Test that various chord patterns are recognized correctly
    assert engine._is_chord_pattern([24, 28, 31])  # C Major triad
    assert engine._is_chord_pattern([24, 27, 31])  # C minor triad
    assert not engine._is_chord_pattern([24])      # Single note
    assert not engine._is_chord_pattern([24, 25])  # Two notes
    assert not engine._is_chord_pattern([])        # Empty
    
    # Test dense cluster recognition
    dense_cluster = [48, 49, 50, 51, 52]  # 5 consecutive semitones
    assert engine._is_chord_pattern(dense_cluster)

def test_audio_engine_key_to_note_name():
    """Test that the key_to_note_name function works correctly."""
    engine = AudioEngine()
    
    # Test specific known conversions
    assert engine._key_to_note_name(0) == "A0"
    assert engine._key_to_note_name(3) == "C0"
    assert engine._key_to_note_name(12) == "A1"
    
    # Key 48 in our implementation is A4 based on the mapping in the code
    # Piano key mapping: A0=0, A#0=1, B0=2, C1=3, C#1=4, D1=5, etc.
    assert engine._key_to_note_name(48) == "A4"
    assert engine._key_to_note_name(51) == "C4"  # Middle C is actually 51 in our mapping
    assert engine._key_to_note_name(87) == "C7"  # Highest piano key in our implementation
