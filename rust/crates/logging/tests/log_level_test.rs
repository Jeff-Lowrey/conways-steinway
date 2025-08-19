// Integration tests for logging configuration constants
// Tests the logging constants without actual logging

use logging::{CONSOLE_PATTERN, FILE_PATTERN};

#[test]
fn test_log_patterns() {
    // Verify logging patterns are non-empty and properly formatted
    assert!(!CONSOLE_PATTERN.is_empty());
    assert!(!FILE_PATTERN.is_empty());
    
    // Console pattern should include log level
    assert!(CONSOLE_PATTERN.contains("{l}"));
    
    // File pattern should include date, level, and thread
    assert!(FILE_PATTERN.contains("{d"));
    assert!(FILE_PATTERN.contains("{l}"));
    assert!(FILE_PATTERN.contains("{t}"));
}
