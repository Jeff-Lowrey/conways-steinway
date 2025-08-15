// Simple test program that verifies logging is working

use std::fs;
use std::path::Path;
use log::{info, warn, error, debug, trace};
use env_logger;

fn main() {
    // Initialize logger
    env_logger::init();
    
    info!("Conway's Steinway - Logging Test");
    debug!("This is a debug message");
    trace!("This is a trace message");
    
    // Check audio paths
    let old_path = "../audio-samples";
    let new_path = "../../static/audio";
    
    info!("Checking old path: {}", old_path);
    if Path::new(old_path).exists() {
        info!("  ✓ Old path exists");
        let files = fs::read_dir(old_path).unwrap();
        let count = files.count();
        info!("  - Contains {} files", count);
    } else {
        warn!("  ✗ Old path does not exist");
    }
    
    info!("Checking new path: {}", new_path);
    if Path::new(new_path).exists() {
        info!("  ✓ New path exists");
        let files = fs::read_dir(new_path).unwrap();
        let file_count = files.count();
        info!("  - Contains {} files", file_count);
        
        // List some files in the new directory
        debug!("  - Files in new directory:");
        for entry in fs::read_dir(new_path).unwrap().take(5) {
            let entry = entry.unwrap();
            debug!("    - {}", entry.file_name().to_string_lossy());
        }
        
        info!("AudioEngine would attempt to load samples from: {}", new_path);
    } else {
        error!("  ✗ New path does not exist - critical error");
    }
    
    info!("Logging test complete. Set RUST_LOG=debug or RUST_LOG=trace to see more messages");
}