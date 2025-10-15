use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_logo_type")]
    pub logo_type: LogoType,
    
    #[serde(default = "default_logo_path")]
    pub custom_logo_path: Option<String>,
    
    #[serde(default = "default_menu_items")]
    pub menu_items: Vec<MenuItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogoType {
    Default,
    Custom,
}

fn default_logo_type() -> LogoType {
    LogoType::Default
}

fn default_logo_path() -> Option<String> {
    None
}

fn default_menu_items() -> Vec<MenuItem> {
    vec![
        MenuItem {
            name: "View Dotfiles".to_string(),
            command: "yazi".to_string(),
            args: vec![],
        },
        MenuItem {
            name: "Configure".to_string(),
            command: "".to_string(),
            args: vec![],
        },
        MenuItem {
            name: "About".to_string(),
            command: "".to_string(),
            args: vec![],
        },
        MenuItem {
            name: "Quit".to_string(),
            command: "".to_string(),
            args: vec![],
        },
    ]
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuItem {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            logo_type: default_logo_type(),
            custom_logo_path: default_logo_path(),
            menu_items: default_menu_items(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::config_path();
        
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!("Error parsing config: {}. Using defaults.", e);
                        Config::default()
                    }
                },
                Err(e) => {
                    eprintln!("Error reading config: {}. Using defaults.", e);
                    Config::default()
                }
            }
        } else {
            let config = Config::default();
            let _ = config.save();
            config
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path();
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> PathBuf {
        let config_dir = std::env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".config")
            });
        config_dir.join("dott").join("config.toml")
    }
}
