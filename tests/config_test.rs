use std::fs;
use std::path::PathBuf;

#[test]
fn test_multiple_entry_groups() {
    // Create a test config with multiple entry groups
    let config_content = r#"
logo_type = "default"

[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries2"

[[structure.build]]
module = "help"

[[entries]]
name = "First Group Item 1"
command = "cmd1"
args = []

[[entries]]
name = "First Group Item 2"
command = "cmd2"
args = []

[[entries2]]
name = "Second Group Item 1"
command = "cmd3"
args = []

[[entries2]]
name = "Second Group Item 2"
command = "cmd4"
args = []
"#;
    
    // Parse the config
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    // Verify structure
    assert_eq!(config["structure"]["position"].as_str().unwrap(), "center");
    
    // Verify build order
    let build = config["structure"]["build"].as_array().unwrap();
    assert_eq!(build.len(), 5);
    assert_eq!(build[0]["module"].as_str().unwrap(), "logo");
    assert_eq!(build[1]["module"].as_str().unwrap(), "entries");
    assert_eq!(build[2]["module"].as_str().unwrap(), "break");
    assert_eq!(build[3]["module"].as_str().unwrap(), "entries2");
    assert_eq!(build[4]["module"].as_str().unwrap(), "help");
    
    // Verify entries
    let entries = config["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0]["name"].as_str().unwrap(), "First Group Item 1");
    assert_eq!(entries[1]["name"].as_str().unwrap(), "First Group Item 2");
    
    // Verify entries2
    let entries2 = config["entries2"].as_array().unwrap();
    assert_eq!(entries2.len(), 2);
    assert_eq!(entries2[0]["name"].as_str().unwrap(), "Second Group Item 1");
    assert_eq!(entries2[1]["name"].as_str().unwrap(), "Second Group Item 2");
}

#[test]
fn test_example_config_parses() {
    let example_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("config-multiple-entries.toml");
    
    let content = fs::read_to_string(&example_path)
        .expect("Failed to read example config");
    
    let config: toml::Value = toml::from_str(&content)
        .expect("Failed to parse example config");
    
    // Verify it has the expected structure
    assert!(config.get("structure").is_some());
    assert!(config.get("entries").is_some());
    assert!(config.get("entries2").is_some());
}

#[test]
fn test_break_configuration() {
    // Test default break lines (should be 2)
    let config_content = r#"
logo_type = "default"

[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries"

[[entries]]
name = "Test"
command = "cmd"
args = []
"#;
    
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    // Verify that break is in the structure
    let build = config["structure"]["build"].as_array().unwrap();
    assert_eq!(build[1]["module"].as_str().unwrap(), "break");
}

#[test]
fn test_break_custom_lines() {
    // Test custom break lines
    let config_content = r#"
logo_type = "default"

[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries"

[[entries]]
name = "Test"
command = "cmd"
args = []

[custom]

[custom.break]
lines = 3
"#;
    
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    // Verify custom break config
    assert!(config.get("custom").is_some());
    assert!(config["custom"].get("break").is_some());
    assert_eq!(config["custom"]["break"]["lines"].as_integer().unwrap(), 3);
}

#[test]
fn test_logo_type_in_structure_build() {
    // Test logo type specified in structure.build
    let config_content = r#"
[structure]
position = "center"

[[structure.build]]
module = "logo:custom"

[[structure.build]]
module = "entries"

custom_logo_path = "/path/to/logo.txt"

[[entries]]
name = "Test"
command = "cmd"
args = []
"#;
    
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    // Verify logo type in structure build
    let build = config["structure"]["build"].as_array().unwrap();
    assert_eq!(build[0]["module"].as_str().unwrap(), "logo:custom");
}
