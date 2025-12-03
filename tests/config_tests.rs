use core::config::Config;
use std::fs;

const TEST_CONFIG_FILE: &str = "test_config.toml";

fn cleanup_test_config() {
    let _ = fs::remove_file(TEST_CONFIG_FILE);
}

#[test]
fn test_config_default_values() {
    cleanup_test_config();

    std::env::set_current_dir(std::env::temp_dir()).unwrap();

    let config = Config::new().unwrap();

    assert_eq!(config.network.host, "0.0.0.0");
    assert_eq!(config.network.port, 19132);
    assert_eq!(config.server.name, "Snow Server");
    assert_eq!(config.server.max_players, 20);

    cleanup_test_config();
}

#[test]
fn test_config_creates_file_if_missing() {
    cleanup_test_config();

    std::env::set_current_dir(std::env::temp_dir()).unwrap();

    assert!(!std::path::Path::new("config.toml").exists());

    let _ = Config::new();

    assert!(std::path::Path::new("config.toml").exists());

    cleanup_test_config();
}

#[test]
fn test_config_deserialize_valid_toml() {
    cleanup_test_config();

    std::env::set_current_dir(std::env::temp_dir()).unwrap();

    let custom_config = r#"
[network]
host = "127.0.0.1"
port = 25565

[server]
name = "Custom Server"
max_players = 100
"#;

    fs::write("config.toml", custom_config).unwrap();

    let config = Config::new().unwrap();

    assert_eq!(config.network.host, "127.0.0.1");
    assert_eq!(config.network.port, 25565);
    assert_eq!(config.server.name, "Custom Server");
    assert_eq!(config.server.max_players, 100);

    cleanup_test_config();
}

#[test]
fn test_config_deserialize_partial_fields() {
    cleanup_test_config();

    std::env::set_current_dir(std::env::temp_dir()).unwrap();

    let partial_config = r#"
[network]
host = "192.168.1.1"
port = 30000

[server]
name = "Partial Server"
max_players = 50
"#;

    fs::write("config.toml", partial_config).unwrap();

    let config = Config::new().unwrap();

    assert_eq!(config.network.host, "192.168.1.1");
    assert_eq!(config.network.port, 30000);
    assert_eq!(config.server.name, "Partial Server");
    assert_eq!(config.server.max_players, 50);

    cleanup_test_config();
}

#[test]
fn test_config_invalid_toml_format() {
    cleanup_test_config();

    std::env::set_current_dir(std::env::temp_dir()).unwrap();

    let invalid_config = r#"
[network
host = "invalid"
"#;

    fs::write("config.toml", invalid_config).unwrap();

    let result = Config::new();

    assert!(result.is_err());

    cleanup_test_config();
}

#[test]
fn test_config_missing_required_fields() {
    cleanup_test_config();

    std::env::set_current_dir(std::env::temp_dir()).unwrap();

    let incomplete_config = r#"
[network]
host = "127.0.0.1"
"#;

    fs::write("config.toml", incomplete_config).unwrap();

    let result = Config::new();

    assert!(result.is_err());

    cleanup_test_config();
}
