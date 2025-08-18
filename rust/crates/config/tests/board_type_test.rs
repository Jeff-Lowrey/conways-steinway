// Integration tests for board type configuration
// These tests verify board type configuration without relying on mocks

use config::{Config, BoardType, GenerationLimit};

#[test]
fn test_board_type_configuration() {
    // Test different board types in configuration
    
    // 1. Test Random board type (default)
    let random_config = Config {
        board_type: BoardType::Random,
        ..Default::default()
    };
    
    assert!(matches!(random_config.board_type, BoardType::Random));
    
    // 2. Test Static board type
    let static_config = Config {
        board_type: BoardType::Static,
        ..Default::default()
    };
    
    assert!(matches!(static_config.board_type, BoardType::Static));
    
    // 3. Test FurElise board type
    let fur_elise_config = Config {
        board_type: BoardType::FurElise,
        ..Default::default()
    };
    
    assert!(matches!(fur_elise_config.board_type, BoardType::FurElise));
    
    // 4. Test Complex board type
    let complex_config = Config {
        board_type: BoardType::Complex,
        ..Default::default()
    };
    
    assert!(matches!(complex_config.board_type, BoardType::Complex));
    
    // 5. Test Showcase board type
    let showcase_config = Config {
        board_type: BoardType::Showcase,
        ..Default::default()
    };
    
    assert!(matches!(showcase_config.board_type, BoardType::Showcase));
}

#[test]
fn test_generation_limit_values() {
    // Test different generation limit values
    
    // 1. Unlimited generations (default)
    let unlimited_config = Config {
        generations: GenerationLimit::Unlimited,
        ..Default::default()
    };
    
    assert!(matches!(unlimited_config.generations, GenerationLimit::Unlimited));
    
    // 2. Limited to specific number of generations
    let limits = [1, 5, 10, 100, 1000];
    
    for &limit in &limits {
        let limited_config = Config {
            generations: GenerationLimit::Limited(limit),
            ..Default::default()
        };
        
        assert!(matches!(limited_config.generations, GenerationLimit::Limited(n) if n == limit));
    }
}

#[test]
fn test_audio_settings() {
    // Test audio configuration settings
    
    // 1. Test audio enabled/disabled
    let audio_enabled_config = Config {
        audio_enabled: true,
        ..Default::default()
    };
    
    let audio_disabled_config = Config {
        audio_enabled: false,
        ..Default::default()
    };
    
    assert!(audio_enabled_config.audio_enabled);
    assert!(!audio_disabled_config.audio_enabled);
    
    // 2. Test note duration settings
    let note_durations = [50, 100, 200, 500];
    
    for &duration in &note_durations {
        let config = Config {
            note_duration_ms: duration,
            ..Default::default()
        };
        
        assert_eq!(config.note_duration_ms, duration);
    }
    
    // 3. Test tempo settings
    let tempos = [60.0, 90.0, 120.0, 180.0];
    
    for &tempo in &tempos {
        let config = Config {
            tempo_bpm: Some(tempo),
            ..Default::default()
        };
        
        assert_eq!(config.tempo_bpm, Some(tempo));
        
        // Effective delay should be calculated from tempo
        let expected_delay = Config::tempo_to_delay_ms(tempo);
        assert_eq!(config.get_effective_delay(), expected_delay);
    }
    
    // 4. When tempo is None, should use step_delay_ms
    let delay_config = Config {
        tempo_bpm: None,
        step_delay_ms: 250,
        ..Default::default()
    };
    
    assert_eq!(delay_config.get_effective_delay(), 250);
}