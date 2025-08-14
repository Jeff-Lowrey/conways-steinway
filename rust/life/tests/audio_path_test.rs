// Test file to verify that audio files are loaded from the correct location

#[test]
fn test_audio_file_paths() {
    use std::path::Path;
    
    // Verify that the static/audio directory exists and contains audio files
    let audio_dir = Path::new("../../static/audio");
    assert!(audio_dir.exists(), "Static audio directory does not exist");
    
    // Check for a few sample files
    let sample_files = [
        "piano_a1.wav",
        "piano_c4.wav",
        "piano_g4.wav",
    ];
    
    for file in sample_files.iter() {
        let file_path = audio_dir.join(file);
        assert!(file_path.exists(), "Audio file {} does not exist", file);
        println!("Found audio file: {}", file_path.display());
    }
}