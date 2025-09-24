#!/bin/python3

"""
This module holds a class to play piano notes based on a given input row.
It is implemented following a similar approach to the Rust version of Conway's Steinway.
"""

import os
import time
from abc import ABC, abstractmethod

from ..core.life import Life, Row


# Define a dummy pygame module for type hints
class DummyPygame:
    class mixer:
        class Sound:
            def play(self) -> None:
                pass


try:
    import pygame

    PYGAME_AVAILABLE = True
except ImportError:
    pygame = DummyPygame()  # type: ignore
    PYGAME_AVAILABLE = False
    print("Warning: pygame not available, using NullAudioEngine instead")


class AudioPlayer(ABC):
    """
    Abstract base class for audio players, defining the interface.
    Similar to the AudioPlayer trait in the Rust implementation.
    """

    @abstractmethod
    def play_piano_keys(self, keys: list[int]) -> None:
        """Play multiple piano keys with slight delays between them."""
        pass

    @abstractmethod
    def play_chord(self, keys: list[int], duration_ms: int) -> None:
        """Play multiple keys simultaneously as a chord."""
        pass

    @abstractmethod
    def play_note_sequence(
        self, keys: list[int], note_duration_ms: int, gap_ms: int
    ) -> None:
        """Play keys sequentially with specified durations and gaps."""
        pass


class AudioEngine(AudioPlayer):
    """
    Audio engine that can play piano notes using pygame.
    Analogous to the AudioEngine struct in the Rust implementation.
    """

    def __init__(self) -> None:
        self.sample_cache: dict[int, pygame.mixer.Sound] = {}

        # Initialize pygame mixer if available
        if PYGAME_AVAILABLE:
            pygame.mixer.init(frequency=44100, size=-16, channels=2, buffer=1024)
            self._load_samples()
        else:
            print("AudioEngine initialized but pygame not available")

    def _load_samples(self) -> None:
        """Load piano sample files."""
        # Piano key mapping: A0=0, A#0=1, B0=2, C1=3, C#1=4, D1=5, D#1=6, E1=7, F1=8, F#1=9, G1=10, G#1=11, A1=12...
        sample_files = [
            # Low range samples
            (9, "../static/audio/piano_a1.wav"),  # A1 (key 9)
            (21, "../static/audio/piano_a2.wav"),  # A2 (key 21)
            (24, "../static/audio/piano_c2.wav"),  # C2 (key 24)
            # Mid-low range (Octave 3) - Better chromatic coverage
            (36, "../static/audio/piano_c3.wav"),  # C3 (key 36)
            (38, "../static/audio/piano_d3.wav"),  # D3 (key 38)
            (41, "../static/audio/piano_f3.wav"),  # F3 (key 41)
            (43, "../static/audio/piano_g3.wav"),  # G3 (key 43)
            # Mid range (Octave 4) - Even better coverage
            (48, "../static/audio/piano_c4_ivory.wav"),  # C4 (key 48) - Best quality
            (50, "../static/audio/piano_d4.wav"),  # D4 (key 50)
            (53, "../static/audio/piano_f4.wav"),  # F4 (key 53)
            (55, "../static/audio/piano_g4.wav"),  # G4 (key 55)
            # Alternative samples for comparison/backup
            (36, "../static/audio/piano_c3_kawai.wav"),  # Alternative C3
            (48, "../static/audio/piano_c4.wav"),  # Alternative C4
            (48, "../static/audio/piano_c4_kawai.wav"),  # Another C4 option
            # Upper range samples
            (60, "../static/audio/piano_c5.wav"),  # C5 (key 60)
            (72, "../static/audio/piano_c6.wav"),  # C6 (key 72)
            (84, "../static/audio/piano_c7.wav"),  # C7 (key 84)
        ]

        for key, file_path in sample_files:
            if os.path.exists(file_path):
                try:
                    sound = pygame.mixer.Sound(file_path)
                    self.sample_cache[key] = sound
                    note_name = self._key_to_note_name(key)
                    print(f"Loaded sample for key {key} ({note_name}): {file_path}")
                except Exception as e:
                    print(f"Failed to load sample file: {file_path} - {e}")
            else:
                print(f"Could not find sample file: {file_path}")

        print(f"Loaded {len(self.sample_cache)} piano samples")
        self._print_coverage_analysis()

    def _key_to_note_name(self, key: int) -> str:
        """Convert key number to note name (e.g. 48 -> 'C4')."""
        note_names = ["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"]
        octave = key // 12
        note_in_octave = key % 12
        return f"{note_names[note_in_octave]}{octave}"

    def _print_coverage_analysis(self) -> None:
        """Print analysis of sample coverage."""
        print("=== Chromatic Coverage Analysis ===")
        keys = sorted(self.sample_cache.keys())

        for key in keys:
            note_name = self._key_to_note_name(key)
            print(f"  {note_name} (key {key})")

        # Analysis of gaps
        gaps = []
        for i in range(1, len(keys)):
            gap = keys[i] - keys[i - 1]
            if gap > 3:
                gaps.append((keys[i - 1], keys[i], gap))

        if gaps:
            print("=== Coverage Gaps (>3 semitones) ===")
            for from_key, to_key, gap in gaps:
                print(
                    f"  {self._key_to_note_name(from_key)} to {self._key_to_note_name(to_key)} ({gap} semitones)"
                )
        else:
            print("Excellent chromatic coverage - no major gaps!")

    def _get_sample_for_key(self, key: int) -> pygame.mixer.Sound | None:
        """Find the closest available sample with intelligent chromatic selection."""
        available_keys = list(self.sample_cache.keys())
        if not available_keys:
            return None

        # Advanced sample selection algorithm for better chromatic coverage
        def key_distance_score(sample_key: int) -> int:
            distance = abs(sample_key - key)

            # Chromatic optimization: prefer samples that result in better pitch shifts
            if distance == 0:
                return 0  # Perfect match
            elif distance <= 2:
                return distance  # Minimal shift penalty (within major second)
            elif distance <= 6:
                return distance + 1  # Slight penalty for larger shifts (up to tritone)
            elif distance <= 12:
                return distance * 2  # Higher penalty for shifts over an octave
            else:
                return distance * 3  # Very high penalty for extreme shifts

        closest_key = min(available_keys, key=key_distance_score)
        return self.sample_cache.get(closest_key)

    def _play_sample(self, key: int) -> None:
        """Play a sample for the given key with pitch adjustment."""
        if not PYGAME_AVAILABLE:
            print(f"Would play key {key} ({self._key_to_note_name(key)})")
            return

        sample = self._get_sample_for_key(key)
        if sample:
            # In a real implementation, we would adjust pitch and volume like in the Rust version
            # Here we just play the closest sample
            closest_key = min(self.sample_cache.keys(), key=lambda k: abs(k - key))
            semitone_difference = key - closest_key

            if abs(semitone_difference) > 0.1:
                print(
                    f"Key {key}: using sample {closest_key} (shift: {semitone_difference} semitones)"
                )

            sample.play()
        else:
            print(f"No sample available for key {key}")

    def _is_chord_pattern(self, keys: list[int]) -> bool:
        """Detect if the keys form a chord pattern."""
        if len(keys) < 3:
            return False

        # Check for common chord patterns
        sorted_keys = sorted(keys)

        # Check for triads (3 notes)
        if len(sorted_keys) >= 3:
            for i in range(len(sorted_keys) - 2):
                root = sorted_keys[i]
                third = sorted_keys[i + 1]
                fifth = sorted_keys[i + 2]

                interval1 = third - root
                interval2 = fifth - root

                # Major chord: 4 and 7 semitones
                # Minor chord: 3 and 7 semitones
                # Diminished chord: 3 and 6 semitones
                # Augmented chord: 4 and 8 semitones
                if (interval1 == 3 or interval1 == 4) and (6 <= interval2 <= 8):
                    return True

        # Check for dense clusters (many consecutive notes)
        if len(sorted_keys) >= 5:
            consecutive_count = 1
            for i in range(1, len(sorted_keys)):
                if sorted_keys[i] - sorted_keys[i - 1] <= 2:
                    consecutive_count += 1
                    if consecutive_count >= 5:
                        return True
                else:
                    consecutive_count = 1

        return False

    def play_piano_keys(self, keys: list[int]) -> None:
        """Play multiple piano keys with slight delays between them."""
        if not keys:
            return

        # Detect chord patterns and play accordingly
        if self._is_chord_pattern(keys):
            self.play_chord(keys, 300)
        else:
            # Play individual keys with slight delay
            for key in keys:
                self._play_sample(key)
                time.sleep(0.05)  # 50ms delay

        # Wait for audio to finish
        time.sleep(0.3)  # 300ms delay

    def play_chord(self, keys: list[int], duration_ms: int) -> None:
        """Play multiple keys simultaneously as a chord."""
        if not keys:
            return

        # Play chord with slight timing offset for natural feel
        for i, key in enumerate(keys):
            # Add slight delay between notes for natural chord attack
            if i > 0:
                time.sleep(0.01)  # 10ms delay
            self._play_sample(key)

        time.sleep(duration_ms / 1000)  # Convert ms to seconds

    def play_note_sequence(
        self, keys: list[int], note_duration_ms: int, gap_ms: int
    ) -> None:
        """Play keys sequentially with specified durations and gaps."""
        if not keys:
            return

        for key in keys:
            self._play_sample(key)
            time.sleep(note_duration_ms / 1000)  # Convert ms to seconds

            if gap_ms > 0:
                time.sleep(gap_ms / 1000)  # Convert ms to seconds


class NullAudioEngine(AudioPlayer):
    """
    Null audio engine that implements the AudioPlayer interface but doesn't produce sound.
    This is useful for testing or when audio is not needed/available.
    """

    def play_piano_keys(self, keys: list[int]) -> None:
        """Play multiple piano keys (no-op)."""
        pass

    def play_chord(self, keys: list[int], duration_ms: int) -> None:
        """Play multiple keys simultaneously as a chord (no-op)."""
        pass

    def play_note_sequence(
        self, keys: list[int], note_duration_ms: int, gap_ms: int
    ) -> None:
        """Play keys sequentially with specified durations and gaps (no-op)."""
        pass


class Piano:
    """
    Piano class that interfaces with an AudioPlayer to generate sound based on
    Conway's Game of Life patterns.
    """

    def __init__(
        self,
        generations: int | None = 10,
        width: int = 88,
        height: int = 40,
        audio_enabled: bool = True,
    ):
        """
        Initialize a Piano instance.

        Args:
            generations: Number of generations to play (None for unlimited)
            width: Width of the Life board (default 88, matching piano keys)
            height: Height of the Life board
            audio_enabled: Whether to use audio output (True) or silent mode (False)
        """
        self.generations = generations
        self.width = width
        self.height = height
        self.is_muted = False
        self.delay_ms = 200  # Default delay between generations
        self.game = Life(width=width, height=height)

        # Choose appropriate audio engine
        if audio_enabled and PYGAME_AVAILABLE:
            self.audio_engine: AudioPlayer = AudioEngine()
        else:
            self.audio_engine = NullAudioEngine()

    def play(self) -> None:
        """
        Execute the game for the specified number of generations.
        For each generation, get the bottom row and play the corresponding notes.
        """
        # For unlimited generations
        if self.generations is None:
            try:
                self._play_unlimited()
            except KeyboardInterrupt:
                print("\nPlayback stopped by user.")
        else:
            self._play_limited(self.generations)

    def _play_unlimited(self) -> None:
        """
        Play the piano indefinitely until interrupted.
        """
        gen_count = 0
        while True:
            self._play_generation()
            gen_count += 1
            print(f"\rGeneration {gen_count}", end="")
            time.sleep(self.delay_ms / 1000)

    def _play_limited(self, count: int) -> None:
        """
        Play the piano for a specified number of generations.
        """
        for gen in range(count):
            self._play_generation()
            if gen < count - 1:  # Don't delay after the last generation
                time.sleep(self.delay_ms / 1000)

    def _play_generation(self) -> None:
        """
        Play a single generation: get bottom row, play notes, advance.
        """
        # Use the GameBoard utility method to get bottom row and advance the game
        from ..core.game_board import GameBoard

        bottom_row_keys = GameBoard.get_bottom_row_and_advance(self.game)

        # Play notes corresponding to live cells
        if not self.is_muted and bottom_row_keys:
            print(f"Playing keys: {bottom_row_keys}")
            for key in bottom_row_keys:
                print(f"Playing key {key}")
            self.audio_engine.play_piano_keys(bottom_row_keys)

    def _row_to_keys(self, row: Row) -> list[int]:
        """
        Convert a row from the Life board to a list of piano key indices.

        Args:
            row: A Row object representing the bottom row of the Life board

        Returns:
            List of key indices (0-87) where cells are alive (1)
        """
        cells = row.get_cells()
        return [i for i, cell in enumerate(cells) if cell == 1]

    def set_mute(self, mute: bool = True) -> None:
        """
        Enable or disable sound output.
        Needed to ensure thread-safety/scalability.
        Only one piano can be using the speakers at once.

        Args:
            mute: True to mute the piano, False to unmute
        """
        self.is_muted = mute

    def get_mute_status(self) -> bool:
        """
        Get the current mute status of the piano.

        Returns:
            True if the piano is muted, False otherwise
        """
        return self.is_muted

    def play_keys(self, keys: list[int]) -> None:
        """
        Play the specified piano keys.

        Args:
            keys: List of key indices (0-87) to play
        """
        if not self.is_muted:
            self.audio_engine.play_piano_keys(keys)

    def play_chord(self, keys: list[int], duration_ms: int = 500) -> None:
        """
        Play the specified keys as a chord.

        Args:
            keys: List of key indices (0-87) to play as a chord
            duration_ms: Duration in milliseconds to hold the chord
        """
        if not self.is_muted:
            self.audio_engine.play_chord(keys, duration_ms)

    def play_sequence(
        self, keys: list[int], note_duration_ms: int = 200, gap_ms: int = 50
    ) -> None:
        """
        Play the specified keys as a sequence.

        Args:
            keys: List of key indices (0-87) to play in sequence
            note_duration_ms: Duration in milliseconds to play each note
            gap_ms: Gap in milliseconds between notes
        """
        if not self.is_muted:
            self.audio_engine.play_note_sequence(keys, note_duration_ms, gap_ms)
