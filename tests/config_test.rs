#[test]
fn test_multiple_entry_groups() {
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
    
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    assert_eq!(config["structure"]["position"].as_str().unwrap(), "center");
    
    let build = config["structure"]["build"].as_array().unwrap();
    assert_eq!(build.len(), 5);
    assert_eq!(build[0]["module"].as_str().unwrap(), "logo");
    assert_eq!(build[1]["module"].as_str().unwrap(), "entries");
    assert_eq!(build[2]["module"].as_str().unwrap(), "break");
    assert_eq!(build[3]["module"].as_str().unwrap(), "entries2");
    assert_eq!(build[4]["module"].as_str().unwrap(), "help");
    
    let entries = config["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0]["name"].as_str().unwrap(), "First Group Item 1");
    assert_eq!(entries[1]["name"].as_str().unwrap(), "First Group Item 2");
    
    let entries2 = config["entries2"].as_array().unwrap();
    assert_eq!(entries2.len(), 2);
    assert_eq!(entries2[0]["name"].as_str().unwrap(), "Second Group Item 1");
    assert_eq!(entries2[1]["name"].as_str().unwrap(), "Second Group Item 2");
}

#[test]
fn test_break_configuration() {
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
    
    let build = config["structure"]["build"].as_array().unwrap();
    assert_eq!(build[1]["module"].as_str().unwrap(), "break");
}

#[test]
fn test_break_custom_lines() {
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
    
    assert!(config.get("custom").is_some());
    assert!(config["custom"].get("break").is_some());
    assert_eq!(config["custom"]["break"]["lines"].as_integer().unwrap(), 3);
}

#[test]
fn test_logo_type_in_structure_build() {
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
    
    let build = config["structure"]["build"].as_array().unwrap();
    assert_eq!(build[0]["module"].as_str().unwrap(), "logo:custom");
}

#[test]
fn test_custom_modules_required() {
    let config_content = r#"
[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "clock"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "entries"

logo_type = "default"

[[entries]]
name = "Test"
command = "cmd"
args = []

[custom]

[custom.terminal_colors]
shape = "circles"

[custom.clock]

[custom.break]
lines = 2
"#;
    
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    assert!(config.get("custom").is_some());
    
    let custom = &config["custom"];
    assert!(custom.get("terminal_colors").is_some());
    assert!(custom.get("clock").is_some());
    assert!(custom.get("break").is_some());
}

#[test]
fn test_selected_module() {
    let config_content = r#"
[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "help"

logo_type = "default"

[[entries]]
name = "Test Command"
command = "echo"
args = ["hello"]

[custom]

[custom.selected]
"#;
    
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    let build = config["structure"]["build"].as_array().unwrap();
    assert_eq!(build[2]["module"].as_str().unwrap(), "selected");
    
    assert!(config.get("custom").is_some());
    assert!(config["custom"].get("selected").is_some());
}

#[test]
fn test_structure_font_option() {
    let config_content = r#"
[structure]
position = "center"
font = "JetBrains Mono"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

logo_type = "default"

[[entries]]
name = "Test"
command = "cmd"
args = []
"#;
    
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    assert_eq!(config["structure"]["font"].as_str().unwrap(), "JetBrains Mono");
}

#[test]
fn test_creative_modules() {
    let config_content = r#"
[structure]
position = "center"

[[structure.build]]
module = "system_info"

[[structure.build]]
module = "uptime"

[[structure.build]]
module = "memory"

[[structure.build]]
module = "disk"

[[structure.build]]
module = "quote"

[[structure.build]]
module = "entries"

logo_type = "default"

[[entries]]
name = "Test"
command = "cmd"
args = []

[custom]

[custom.system_info]

[custom.uptime]

[custom.memory]

[custom.disk_usage]
path = "/"

[custom.quote]
quotes = ["Test quote"]
"#;
    
    let config: toml::Value = toml::from_str(config_content).expect("Failed to parse config");
    
    let build = config["structure"]["build"].as_array().unwrap();
    assert_eq!(build[0]["module"].as_str().unwrap(), "system_info");
    assert_eq!(build[1]["module"].as_str().unwrap(), "uptime");
    assert_eq!(build[2]["module"].as_str().unwrap(), "memory");
    assert_eq!(build[3]["module"].as_str().unwrap(), "disk");
    assert_eq!(build[4]["module"].as_str().unwrap(), "quote");
    
    let custom = &config["custom"];
    assert!(custom.get("system_info").is_some());
    assert!(custom.get("uptime").is_some());
    assert!(custom.get("memory").is_some());
    assert!(custom.get("disk_usage").is_some());
    assert!(custom.get("quote").is_some());
    
    assert_eq!(custom["disk_usage"]["path"].as_str().unwrap(), "/");
    
    let quotes = custom["quote"]["quotes"].as_array().unwrap();
    assert_eq!(quotes.len(), 1);
    assert_eq!(quotes[0].as_str().unwrap(), "Test quote");
}

