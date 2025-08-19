// Integration test for Config module

use config::{Config, BoardType, GenerationLimit};

#[test]
fn test_config_defaults() {
    // Test the default configuration
    let config = Config::default();
    
    // Verify default values
    assert!(matches!(config.board_type, BoardType::Random));
    assert!(!config.silent); // Audio is enabled by default (silent=false)
    assert!(matches!(config.generations, GenerationLimit::Unlimited));
    assert_eq!(config.step_delay_ms, 200);
    assert!(config.tempo_bpm.is_none());
}

#[test]
fn test_config_custom_values() {
    // Create a custom config
    let config = Config {
        board_type: BoardType::Static,
        silent: true, // Audio disabled
        generations: GenerationLimit::Limited(42),
        step_delay_ms: 300,
        tempo_bpm: Some(120.0),
        ..Default::default()
    };
    
    // Verify custom values
    assert!(matches!(config.board_type, BoardType::Static));
    assert!(config.silent); // Verify audio is disabled
    assert!(matches!(config.generations, GenerationLimit::Limited(42)));
    assert_eq!(config.step_delay_ms, 300);
    assert!(config.tempo_bpm.is_some());
    assert!((config.tempo_bpm.unwrap() - 120.0).abs() < 0.01);
}

#[test]
fn test_tempo_to_delay_conversion() {
    // Test tempo to delay conversion
    let delay_120_bpm = Config::tempo_to_delay_ms(120.0);
    let delay_60_bpm = Config::tempo_to_delay_ms(60.0);
    
    // At 120 BPM, eighth notes should be about 250ms
    assert!((delay_120_bpm as f64 - 250.0).abs() < 1.0);
    
    // 60 BPM should be twice as slow as 120 BPM
    assert_eq!(delay_60_bpm, delay_120_bpm * 2);
    
    // Test effective delay with and without tempo
    let mut config = Config::default();
    config.step_delay_ms = 100;
    assert_eq!(config.get_effective_delay(), 100); // Uses step_delay_ms
    
    config.tempo_bpm = Some(120.0);
    assert_eq!(config.get_effective_delay(), delay_120_bpm); // Uses tempo
}