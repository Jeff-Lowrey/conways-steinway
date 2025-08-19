// Common utility functions for Conway's Steinway
//
// This module provides shared utilities for finding paths relative to the repository root

use std::path::{Path, PathBuf};
use std::env;

/// Represents paths to important directories in the repository
pub struct RepoStructure {
    /// Path to the repository root
    pub root: PathBuf,
    /// Path to the static directory
    pub static_dir: PathBuf,
    /// Path to the logs directory
    pub logs_dir: PathBuf,
    /// Path to the config directory
    pub config_dir: PathBuf,
}

impl RepoStructure {
    /// Create a new RepoStructure by finding the repository root
    pub fn new() -> Self {
        let root = find_repo_root();
        let static_dir = root.join("static");
        let logs_dir = root.join("logs");
        let config_dir = root.join("config");
        
        Self {
            root,
            static_dir,
            logs_dir,
            config_dir,
        }
    }
    
    /// Get the path to the audio samples directory
    pub fn audio_samples_dir(&self) -> PathBuf {
        self.static_dir.join("audio")
    }
    
    /// Get the path to a specific audio sample file
    pub fn audio_sample_path(&self, filename: &str) -> PathBuf {
        self.audio_samples_dir().join(filename)
    }
    
    /// Get the path to the logs backend directory
    pub fn logs_backend_dir(&self) -> PathBuf {
        self.logs_dir.join("backend")
    }
    
    /// Get the path to the default log file
    pub fn default_log_file(&self) -> PathBuf {
        self.logs_backend_dir().join("conways_steinway.log")
    }
}

/// Find the repository root directory
///
/// This function tries to find the repository root by looking for certain marker directories
/// like "static", "rust", and "python" that indicate the root of the project.
pub fn find_repo_root() -> PathBuf {
    // Start with the current directory
    let mut current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // Look for the repository root
    loop {
        // Check if this looks like the repository root
        if is_repo_root(&current_dir) {
            return current_dir;
        }
        
        // Move up one directory
        if !current_dir.pop() {
            // If we can't go up anymore, just return the current directory
            return current_dir;
        }
    }
}

/// Check if the given directory is the repository root
fn is_repo_root(dir: &Path) -> bool {
    // Check for common directories that would be at the repository root
    let has_static = dir.join("static").is_dir();
    let has_rust = dir.join("rust").is_dir();
    let has_python = dir.join("python").is_dir();
    
    // The repository root should have at least static and one implementation directory
    has_static && (has_rust || has_python)
}

/// Get a path relative to the repository root
pub fn path_from_repo_root(rel_path: &str) -> PathBuf {
    let repo_root = find_repo_root();
    repo_root.join(rel_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_repo_structure() {
        let repo = RepoStructure::new();
        
        // Verify the structure has the expected directories
        assert!(repo.root.exists(), "Repository root should exist");
        assert!(repo.static_dir.exists(), "Static directory should exist");
        assert!(repo.audio_samples_dir().exists(), "Audio samples directory should exist");
        
        // The logs directory might not exist yet, so we don't assert its existence
    }
    
    #[test]
    fn test_find_repo_root() {
        let root = find_repo_root();
        
        // Verify this looks like a repository root
        assert!(root.join("static").is_dir(), "Repository root should contain a static directory");
        assert!(root.join("static").join("audio").is_dir(), "Static directory should contain an audio directory");
    }
    
    #[test]
    fn test_path_from_repo_root() {
        let path = path_from_repo_root("static/audio");
        assert!(path.exists(), "Path to static/audio should exist");
        assert!(path.is_dir(), "static/audio should be a directory");
    }
}