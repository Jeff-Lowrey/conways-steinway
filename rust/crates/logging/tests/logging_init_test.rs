// Integration tests for logging initialization and functionality
// Tests that logging can be initialized and used properly

use log::{info, debug, trace};
use env_logger;

#[test]
fn test_logger_initialization() {
    // Initialize logger with default configuration
    // This uses a special test mode that doesn't interfere with other tests
    let _ = env_logger::builder()
        .is_test(true)  // Send output to stderr for tests
        .try_init();    // May fail if already initialized, which is fine
    
    // Log different levels - we can't assert the output, but we can ensure no panics
    info!("Conway's Steinway - Logging Test");
    debug!("This is a debug message");
    trace!("This is a trace message");
    
    // Success is no panic/crash when logging
}
