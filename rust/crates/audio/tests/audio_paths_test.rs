// Integration tests for audio path validation
// Tests that audio sample paths exist and are accessible

use std::fs;
use common::RepoStructure;

#[test]
fn test_audio_sample_paths() {
    // Get audio path using repository structure
    let repo = RepoStructure::new();
    let audio_dir = repo.audio_samples_dir();
    
    // Verify the audio path exists and contains files
    assert!(audio_dir.exists(), "Audio path does not exist: {}", audio_dir.display());
    
    // Check file count
    let files = fs::read_dir(&audio_dir).unwrap();
    let file_count = files.count();
    assert!(file_count > 0, "Audio path exists but contains no files");
    
    // Count audio files with proper extensions
    let audio_files = fs::read_dir(&audio_dir).unwrap()
        .filter_map(Result::ok)
        .filter(|entry| {
            if let Some(ext) = entry.path().extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                return ext == "wav" || ext == "mp3" || ext == "ogg";
            }
            false
        })
        .count();
    
    assert!(audio_files > 0, "Audio path contains no audio files");
    
    // List audio files for debugging
    println!("Found {} audio files at {}", audio_files, audio_dir.display());
}
