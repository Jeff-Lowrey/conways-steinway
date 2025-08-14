// Simple program to verify the audio files are in the correct location

use std::fs;
use std::path::Path;

fn main() {
    println!("Testing audio file locations...");
    
    // Check both the old and new paths
    let old_path = "../audio-samples";
    let new_path = "../../static/audio";
    
    println!("Checking old path: {}", old_path);
    if Path::new(old_path).exists() {
        println!("  ✓ Old path exists");
        let files = fs::read_dir(old_path).unwrap();
        let count = files.count();
        println!("  - Contains {} files", count);
    } else {
        println!("  ✗ Old path does not exist");
    }
    
    println!("Checking new path: {}", new_path);
    if Path::new(new_path).exists() {
        println!("  ✓ New path exists");
        let files = fs::read_dir(new_path).unwrap();
        let file_count = files.count();
        println!("  - Contains {} files", file_count);
        
        // List some files in the new directory
        println!("  - Files in new directory:");
        for entry in fs::read_dir(new_path).unwrap().take(5) {
            let entry = entry.unwrap();
            println!("    - {}", entry.file_name().to_string_lossy());
        }
        
        // In a full test, we would instantiate the AudioEngine here to verify
        // that it can load files from the new location
        println!("\nNOTE: AudioEngine would attempt to load samples from: {}", new_path);
    } else {
        println!("  ✗ New path does not exist");
    }
    
    println!("Audio file check complete.");
}