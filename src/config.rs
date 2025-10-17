use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
    Selected,
    Quit,
    SystemInfo,
    Quote,
    Uptime,
    DiskUsage,
    Memory,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Structure {
    #[serde(default = "default_position")]
    pub position: Position,
    
    #[serde(default = "default_build")]
    pub build: Vec<StructureBuildItem>,
    
    #[serde(default)]
    pub font: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StructureBuildItem {
    pub module: String,
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

fn default_build() -> Vec<StructureBuildItem> {
    vec![
        StructureBuildItem { module: "logo".to_string() },
        StructureBuildItem { module: "entries".to_string() },
        StructureBuildItem { module: "help".to_string() },
    ]
}

fn default_structure() -> Structure {
    Structure {
        position: default_position(),
        build: default_build(),
        font: None,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomModules {
    #[serde(default = "default_terminal_colors")]
    pub terminal_colors: TerminalColorsConfig,

    #[serde(default)]
    pub clock: ClockConfig,

    #[serde(default = "default_break_config")]
    pub break_: BreakConfig,
    
    #[serde(default)]
    pub selected: SelectedConfig,
    
    #[serde(default)]
    pub system_info: SystemInfoConfig,
    
    #[serde(default)]
    pub quote: QuoteConfig,
    
    #[serde(default)]
    pub uptime: UptimeConfig,
    
    #[serde(default)]
    pub disk_usage: DiskUsageConfig,
    
    #[serde(default)]
    pub memory: MemoryConfig,
}

fn default_terminal_colors() -> TerminalColorsConfig {
    TerminalColorsConfig {
        shape: default_color_shape(),
    }
}

fn default_break_config() -> BreakConfig {
    BreakConfig {
        lines: default_break_lines(),
    }
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SelectedConfig {
    // No configurable settings for now
    // This module will display the command for the currently selected entry
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SystemInfoConfig {
    // Display system information: hostname, OS, kernel
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuoteConfig {
    #[serde(default)]
    pub quotes: Vec<String>,
}

impl Default for QuoteConfig {
    fn default() -> Self {
        QuoteConfig {
            quotes: vec![
                "The only way to do great work is to love what you do. - Steve Jobs".to_string(),
                "Innovation distinguishes between a leader and a follower. - Steve Jobs".to_string(),
                "Stay hungry, stay foolish. - Steve Jobs".to_string(),
                "Code is like humor. When you have to explain it, it's bad. - Cory House".to_string(),
                "First, solve the problem. Then, write the code. - John Johnson".to_string(),
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UptimeConfig {
    // Display system uptime
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskUsageConfig {
    #[serde(default)]
    pub path: String,
}

impl Default for DiskUsageConfig {
    fn default() -> Self {
        DiskUsageConfig {
            path: "/".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MemoryConfig {
    // Display memory usage
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
        
        for build_item in self.structure.build.iter() {
            let module_name = &build_item.module;
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
                    "selected" => Some(ModuleType::Selected),
                    "quit" => Some(ModuleType::Quit),
                    "system_info" | "systeminfo" => Some(ModuleType::SystemInfo),
                    "quote" => Some(ModuleType::Quote),
                    "uptime" => Some(ModuleType::Uptime),
                    "disk_usage" | "diskusage" | "disk" => Some(ModuleType::DiskUsage),
                    "memory" | "mem" => Some(ModuleType::Memory),
                    _ => None,
                }
            };
            
            if let Some(mt) = module_type {
                modules.push(OrderedModule {
                    module_type: mt,
                });
            }
        }
        
        modules
    }

    /// Get the number of lines for a break module
    pub fn get_break_lines(&self) -> usize {
        if let Some(ref custom) = self.custom {
            return custom.break_.lines;
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
