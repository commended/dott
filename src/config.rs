use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_structure")]
    pub structure: Structure,

    #[serde(default = "default_logo_type")]
    pub logo_type: LogoType,
    
    #[serde(default = "default_logo_path")]
    pub custom_logo_path: Option<String>,

    #[serde(default = "default_image_path")]
    pub image_logo_path: Option<String>,
    
    #[serde(default = "default_entries")]
    pub entries: Vec<MenuItem>,

    #[serde(default)]
    pub entries2: Vec<MenuItem>,

    #[serde(default)]
    pub entries3: Vec<MenuItem>,

    #[serde(default)]
    pub entries4: Vec<MenuItem>,

    #[serde(default)]
    pub entries5: Vec<MenuItem>,

    #[serde(default)]
    pub custom: Option<CustomModules>,
}

#[derive(Debug, Clone)]
pub struct OrderedModule {
    pub order: u32,
    pub module_type: ModuleType,
}

#[derive(Debug, Clone)]
pub enum ModuleType {
    Logo(LogoType),
    Entries(String), // String indicates which entries array (e.g., "entries", "entries2")
    Colors,
    Clock,
    Help,
    Break,
    Quit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Structure {
    #[serde(default = "default_position")]
    pub position: Position,
    
    #[serde(default = "default_build")]
    #[serde(deserialize_with = "deserialize_string_key_map")]
    pub build: BTreeMap<u32, String>,
}

fn deserialize_string_key_map<'de, D>(deserializer: D) -> Result<BTreeMap<u32, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    let string_map: BTreeMap<String, String> = BTreeMap::deserialize(deserializer)?;
    let mut result = BTreeMap::new();
    
    for (key, value) in string_map {
        let num_key = key.parse::<u32>()
            .map_err(|e| D::Error::custom(format!("Failed to parse key '{}' as u32: {}", key, e)))?;
        result.insert(num_key, value);
    }
    
    Ok(result)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Center,
    Left,
    Right,
}

fn default_position() -> Position {
    Position::Center
}

fn default_build() -> BTreeMap<u32, String> {
    let mut build = BTreeMap::new();
    build.insert(1, "logo".to_string());
    build.insert(2, "entries".to_string());
    build.insert(3, "help".to_string());
    build
}

fn default_structure() -> Structure {
    Structure {
        position: default_position(),
        build: default_build(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomModules {
    #[serde(default)]
    pub terminal_colors: Option<TerminalColorsConfig>,

    #[serde(default)]
    pub clock: Option<ClockConfig>,

    #[serde(default)]
    pub break_: Option<BreakConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogoType {
    Default,
    Custom,
    Image,
}

fn default_logo_type() -> LogoType {
    LogoType::Default
}

fn default_logo_path() -> Option<String> {
    None
}

fn default_image_path() -> Option<String> {
    None
}

fn default_entries() -> Vec<MenuItem> {
    vec![
        MenuItem {
            name: "View Dotfiles".to_string(),
            command: "yazi".to_string(),
            args: vec!["~/.config".to_string()],
        },
        MenuItem {
            name: "Edit Dott Config".to_string(),
            command: "".to_string(),
            args: vec![],
        },
        MenuItem {
            name: "View Shell".to_string(),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TerminalColorsConfig {
    #[serde(default = "default_color_shape")]
    pub shape: ColorShape,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ColorShape {
    Circles,
    Squares,
}

fn default_color_shape() -> ColorShape {
    ColorShape::Circles
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClockConfig {
    // Position is now determined by structure.build order
    // No position field needed
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BreakConfig {
    #[serde(default = "default_break_lines")]
    pub lines: usize,
}

fn default_break_lines() -> usize {
    2
}

impl Default for Config {
    fn default() -> Self {
        Config {
            structure: default_structure(),
            logo_type: default_logo_type(),
            custom_logo_path: default_logo_path(),
            image_logo_path: default_image_path(),
            entries: default_entries(),
            entries2: Vec::new(),
            entries3: Vec::new(),
            entries4: Vec::new(),
            entries5: Vec::new(),
            custom: None,
        }
    }
}

impl Config {
    /// Get all entries for a specific entries group name (e.g., "entries", "entries2")
    pub fn get_entries(&self, name: &str) -> &[MenuItem] {
        match name {
            "entries" => &self.entries,
            "entries2" => &self.entries2,
            "entries3" => &self.entries3,
            "entries4" => &self.entries4,
            "entries5" => &self.entries5,
            _ => &[],
        }
    }

    /// Get all modules ordered according to structure.build
    pub fn get_ordered_modules(&self) -> Vec<OrderedModule> {
        let mut modules = Vec::new();
        
        for (order, module_name) in &self.structure.build {
            let module_type = if module_name.starts_with("logo") {
                // Parse logo type from "logo", "logo:default", "logo:custom", or "logo:image"
                let logo_type = if module_name == "logo" {
                    // Use the top-level logo_type if just "logo" is specified
                    self.logo_type.clone()
                } else {
                    // Parse the type from "logo:TYPE"
                    let parts: Vec<&str> = module_name.split(':').collect();
                    if parts.len() == 2 {
                        match parts[1] {
                            "default" => LogoType::Default,
                            "custom" => LogoType::Custom,
                            "image" => LogoType::Image,
                            _ => self.logo_type.clone(), // Fallback to top-level
                        }
                    } else {
                        self.logo_type.clone()
                    }
                };
                Some(ModuleType::Logo(logo_type))
            } else {
                match module_name.as_str() {
                    "entries" => Some(ModuleType::Entries("entries".to_string())),
                    "entries2" => Some(ModuleType::Entries("entries2".to_string())),
                    "entries3" => Some(ModuleType::Entries("entries3".to_string())),
                    "entries4" => Some(ModuleType::Entries("entries4".to_string())),
                    "entries5" => Some(ModuleType::Entries("entries5".to_string())),
                    "colors" => Some(ModuleType::Colors),
                    "clock" => Some(ModuleType::Clock),
                    "help" => Some(ModuleType::Help),
                    "break" => Some(ModuleType::Break),
                    "quit" => Some(ModuleType::Quit),
                    _ => None,
                }
            };
            
            if let Some(mt) = module_type {
                modules.push(OrderedModule {
                    order: *order,
                    module_type: mt,
                });
            }
        }
        
        modules
    }

    /// Get the number of lines for a break module
    pub fn get_break_lines(&self) -> usize {
        if let Some(ref custom) = self.custom {
            if let Some(ref break_config) = custom.break_ {
                return break_config.lines;
            }
        }
        2 // Default to 2 lines
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
