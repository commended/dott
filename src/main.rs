mod config;

use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame, Terminal,
};
use std::io;
use chrono::Local;
use std::io::Write;
use rand::seq::SliceRandom;
use sysinfo::{System, Disks};
use clap::Parser;
use std::path::PathBuf;

const DOTT_LOGO: &str = r#"
    ;'*¨'`·- .,  ‘                   , ·. ,.-·~·.,   ‘             ,  . .,  °             ,  . .,  °    
    \`:·-,. ,   '` ·.  '            /  ·'´,.-·-.,   `,'‚       ;'´    ,   ., _';\'     ;'´    ,   ., _';\'  
     '\:/   ;\:'`:·,  '`·, '        /  .'´\:::::::'\   '\ °     \:´¨¯:;'   `;::'\:'\    \:´¨¯:;'   `;::'\:'\ 
      ;   ;'::\;::::';   ;\     ,·'  ,'::::\:;:-·-:';  ';\‚       \::::;   ,'::_'\;'      \::::;   ,'::_'\;' 
      ;  ,':::;  `·:;;  ,':'\'  ;.   ';:::;´       ,'  ,':'\‚          ,'  ,'::;'  ‘            ,'  ,'::;'  ‘   
     ;   ;:::;    ,·' ,·':::;   ';   ;::;       ,'´ .'´\::';‚         ;  ;:::;  °            ;  ;:::;  °   
     ;  ;:::;'  ,.'´,·´:::::;   ';   ':;:   ,.·´,.·´::::\;'°         ;  ;::;'  ‘             ;  ;::;'  ‘    
    ':,·:;::-·´,.·´\:::::;´'     \·,   `*´,.·'´::::::;·´            ;  ;::;'‚               ;  ;::;'‚      
     \::;. -·´:::::;\;·´         \\:¯::\:::::::;:·´               ',.'\::;'‚               ',.'\::;'‚      
      \;'\::::::::;·´'             `\:::::\;::·'´  °                 \::\:;'‚                \::\:;'‚      
         `\;::-·´                     ¯                             \;:'      ‘             \;:'      ‘  
                                       ‘                               °                      °         
                      
"#;

#[derive(Parser, Debug)]
#[command(name = "dott-tui")]
#[command(about = "A beautiful and fast TUI written in Rust", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short = 'C', long = "config", value_name = "FILE")]
    config: Option<PathBuf>,
}

fn display_kitty_image(path: &str) -> Result<(), String> {
    use std::fs;
    
    let image_data = fs::read(path).map_err(|e| format!("Failed to read image: {}", e))?;
    
    let encoded = base64_encode(&image_data);
    

    let escape_seq = format!("\x1b_Gf=100,a=T,t=f;{}\x1b\\", encoded);
    
    print!("{}", escape_seq);
    io::stdout().flush().ok();
    
    Ok(())
}

fn base64_encode(data: &[u8]) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in data.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b1 = (buf[0] >> 2) & 0x3F;
        let b2 = ((buf[0] & 0x03) << 4) | ((buf[1] >> 4) & 0x0F);
        let b3 = ((buf[1] & 0x0F) << 2) | ((buf[2] >> 6) & 0x03);
        let b4 = buf[2] & 0x3F;
        
        result.push(BASE64_CHARS[b1 as usize] as char);
        result.push(BASE64_CHARS[b2 as usize] as char);
        
        if chunk.len() > 1 {
            result.push(BASE64_CHARS[b3 as usize] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(BASE64_CHARS[b4 as usize] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}



fn detect_shell_config() -> Option<String> {
    let shell = std::env::var("SHELL").ok()?;
    
    let config_file = if shell.contains("zsh") {
        "~/.zshrc"
    } else if shell.contains("bash") {
        "~/.bashrc"
    } else if shell.contains("fish") {
        "~/.config/fish/config.fish"
    } else if shell.contains("ksh") {
        "~/.kshrc"
    } else if shell.contains("tcsh") {
        "~/.tcshrc"
    } else {
        "~/.bashrc"
    };
    
    Some(config_file.to_string())
}

struct App {
    selected: usize,
    config: Config,
    all_entries: Vec<(String, config::MenuItem)>,
    config_path: Option<PathBuf>,
}

impl App {
    fn new(config_path: Option<PathBuf>) -> App {
        let config = Config::load_from(config_path.clone());
        
        let mut all_entries = Vec::new();
        for module in config.get_ordered_modules() {
            if let config::ModuleType::Entries(group_name) = module.module_type {
                let entries = config.get_entries(&group_name);
                for entry in entries {
                    all_entries.push((group_name.clone(), entry.clone()));
                }
            }
        }
        
        App {
            selected: 0,
            config,
            all_entries,
            config_path,
        }
    }

    fn next(&mut self) {
        if !self.all_entries.is_empty() {
            self.selected = (self.selected + 1) % self.all_entries.len();
        }
    }

    fn previous(&mut self) {
        if !self.all_entries.is_empty() {
            if self.selected > 0 {
                self.selected -= 1;
            } else {
                self.selected = self.all_entries.len() - 1;
            }
        }
    }

    fn get_selected_item(&self) -> Option<&config::MenuItem> {
        self.all_entries.get(self.selected).map(|(_, item)| item)
    }
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    
    // Expand ~ in config path if provided
    let config_path = args.config.map(|path| {
        let path_str = path.to_string_lossy().to_string();
        if path_str == "~" || path_str.starts_with("~/") {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(path_str.replacen("~", &home, 1))
        } else {
            path
        }
    });
    
    let app = App::new(config_path);
    
    if let config::LogoType::Image = app.config.logo_type {
        if let Some(ref image_path) = app.config.image_logo_path {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let expanded_path = image_path.replace("~", &home);
            
            println!("\n");
            if let Err(e) = display_kitty_image(&expanded_path) {
                eprintln!("Warning: Failed to display image logo: {}", e);
                eprintln!("Note: This feature requires a terminal with Kitty graphics protocol support");
            }
            println!("\n");
            
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app;
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend + std::io::Write>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('u') => {
                        app.config = Config::load_from(app.config_path.clone());
                        app.selected = 0;
                        
                        app.all_entries.clear();
                        for module in app.config.get_ordered_modules() {
                            if let config::ModuleType::Entries(group_name) = module.module_type {
                                let entries = app.config.get_entries(&group_name);
                                for entry in entries {
                                    app.all_entries.push((group_name.clone(), entry.clone()));
                                }
                            }
                        }
                        
                        terminal.clear()?;
                    }
                    KeyCode::Enter => {
                    if let Some(selected) = app.get_selected_item() {
                    
                    match selected.name.as_str() {
                        "Quit" => return Ok(()),
                        "View Shell" => {
                            if let Some(shell_config) = detect_shell_config() {
                                disable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    LeaveAlternateScreen,
                                    DisableMouseCapture
                                )?;
                                terminal.show_cursor()?;

                                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                                let expanded_path = shell_config.replace("~", &home);

                                let _ = std::process::Command::new("nvim")
                                    .arg(&expanded_path)
                                    .status();

                                enable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    EnterAlternateScreen,
                                    EnableMouseCapture
                                )?;
                                terminal.clear()?;
                            }
                        }
                        "Edit Dott Config" => {
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;

                            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                            let config_path = format!("{}/.config/dott/config.toml", home);
                            let _ = std::process::Command::new("nvim")
                                .arg(&config_path)
                                .status();

                            enable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                EnterAlternateScreen,
                                EnableMouseCapture
                            )?;
                            terminal.clear()?;
                        }
                        _ => {
                            if !selected.command.is_empty() {
                                disable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    LeaveAlternateScreen,
                                    DisableMouseCapture
                                )?;
                                terminal.show_cursor()?;

                                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                                let expanded_args: Vec<String> = selected.args.iter()
                                    .map(|arg| arg.replace("~", &home))
                                    .collect();
                                let args: Vec<&str> = expanded_args.iter().map(|s| s.as_str()).collect();
                                
                                let _ = std::process::Command::new(&selected.command)
                                    .args(&args)
                                    .status();

                                enable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    EnterAlternateScreen,
                                    EnableMouseCapture
                                )?;
                                terminal.clear()?;
                            }
                        }
                    }
                }
                }
                _ => {}
            }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();
    
    let ordered_modules = app.config.get_ordered_modules();
    
    let mut lines = Vec::new();
    let mut current_entry_index = 0;
    
    for module in &ordered_modules {
        match &module.module_type {
            config::ModuleType::Logo(logo_type) => {
                let logo_text = get_logo_text_with_type(logo_type, &app.config);
                for line in logo_text.lines() {
                    lines.push(Line::from(Span::styled(line.to_string(), Style::default().fg(Color::Cyan))));
                }
            }
            config::ModuleType::Entries(group_name) => {
                let entries = app.config.get_entries(group_name);
                for entry in entries {
                    let is_selected = current_entry_index == app.selected;
                    let (prefix, style) = if is_selected {
                        (
                            "> ",
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                                .add_modifier(Modifier::BOLD)
                        )
                    } else {
                        (
                            "> ",
                            Style::default().fg(Color::White)
                        )
                    };
                    lines.push(Line::from(Span::styled(format!("{}{}  ", prefix, entry.name), style)));
                    current_entry_index += 1;
                }
            }
            config::ModuleType::Colors => {
                if let Some(ref _custom) = app.config.custom {
                    let color_lines = render_terminal_colors_lines(&app.config.custom.as_ref().unwrap().terminal_colors);
                    lines.extend(color_lines);
                }
            }
            config::ModuleType::Clock => {
                if let Some(ref _custom) = app.config.custom {
                    let time = Local::now().format("%H:%M:%S").to_string();
                    lines.push(Line::from(Span::styled(time, Style::default().fg(Color::Cyan))));
                }
            }
            config::ModuleType::Help => {
                lines.push(Line::from(Span::styled(
                    "↑/k: Up | ↓/j: Down | Enter: Select | u: Reload Config | q/Esc: Quit".to_string(),
                    Style::default().fg(Color::DarkGray)
                )));
            }
            config::ModuleType::Selected => {
                if let Some(ref _custom) = app.config.custom {
                    if let Some(selected_entry) = app.get_selected_item() {
                        let command_text = if selected_entry.command.is_empty() {
                            match selected_entry.name.as_str() {
                                "Quit" => "Exit application".to_string(),
                                "Edit Dott Config" => "Edit dott config in nvim".to_string(),
                                "View Shell" => "View shell config in nvim".to_string(),
                                _ => "No command".to_string(),
                            }
                        } else {
                            let args_str = if selected_entry.args.is_empty() {
                                String::new()
                            } else {
                                format!(" {}", selected_entry.args.join(" "))
                            };
                            format!("{}{}", selected_entry.command, args_str)
                        };
                        lines.push(Line::from(Span::styled(
                            format!("Selected: {}", command_text),
                            Style::default().fg(Color::Yellow)
                        )));
                    }
                }
            }
            config::ModuleType::Break => {
                let break_lines = app.config.get_break_lines();
                for _ in 0..break_lines {
                    lines.push(Line::from(""));
                }
            }
            config::ModuleType::Quit => {
            }
            config::ModuleType::SystemInfo => {
                if let Some(ref _custom) = app.config.custom {
                    let system_lines = render_system_info();
                    lines.extend(system_lines);
                }
            }

            config::ModuleType::Quote => {
                if let Some(ref custom) = app.config.custom {
                    let quote_lines = render_quote(&custom.quote);
                    lines.extend(quote_lines);
                }
            }
            config::ModuleType::Uptime => {
                if let Some(ref _custom) = app.config.custom {
                    let uptime_lines = render_uptime();
                    lines.extend(uptime_lines);
                }
            }
            config::ModuleType::DiskUsage => {
                if let Some(ref custom) = app.config.custom {
                    let disk_lines = render_disk_usage(&custom.disk_usage);
                    lines.extend(disk_lines);
                }
            }
            config::ModuleType::Memory => {
                if let Some(ref _custom) = app.config.custom {
                    let memory_lines = render_memory();
                    lines.extend(memory_lines);
                }
            }
        }
    }
    
    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(paragraph, size);
}

fn get_logo_text_with_type(logo_type: &config::LogoType, config: &Config) -> String {
    match logo_type {
        config::LogoType::Default => DOTT_LOGO.to_string(),
        config::LogoType::Custom => {
            if let Some(ref path) = config.custom_logo_path {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                let expanded_path = path.replace("~", &home);
                std::fs::read_to_string(&expanded_path).unwrap_or_else(|_| DOTT_LOGO.to_string())
            } else {
                DOTT_LOGO.to_string()
            }
        }
        config::LogoType::Image => {
            if let Some(ref path) = config.image_logo_path {
                format!("\n\n  [Image Logo: {}]\n  (Experimental: Use Kitty terminal)\n  (Image displayed before TUI launch)\n\n", path)
            } else {
                DOTT_LOGO.to_string()
            }
        }
    }
}

fn render_terminal_colors_lines(colors_config: &config::TerminalColorsConfig) -> Vec<Line<'static>> {
    let colors = vec![
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];
    
    let mut lines = Vec::new();
    
    match colors_config.shape {
        config::ColorShape::Circles => {
            let mut spans = vec![];
            for color in &colors {
                spans.push(Span::styled("● ", Style::default().fg(*color)));
            }
            lines.push(Line::from(spans));
        }
        config::ColorShape::Squares => {
            let mut spans_row1 = vec![];
            for color in colors.iter().take(4) {
                spans_row1.push(Span::styled("■ ", Style::default().fg(*color)));
            }
            lines.push(Line::from(spans_row1));
            
            let mut spans_row2 = vec![];
            for color in colors.iter().skip(4).take(4) {
                spans_row2.push(Span::styled("■ ", Style::default().fg(*color)));
            }
            lines.push(Line::from(spans_row2));
        }
    }
    
    lines
}

fn render_system_info() -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_else(|_| {
            std::fs::read_to_string("/etc/hostname")
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|_| "unknown".to_string())
        });
    
    let os = std::env::consts::OS;
    
    let kernel = if cfg!(target_os = "linux") {
        std::fs::read_to_string("/proc/version")
            .ok()
            .and_then(|v| v.split_whitespace().nth(2).map(|s| s.to_string()))
            .unwrap_or_else(|| "unknown".to_string())
    } else {
        std::env::consts::ARCH.to_string()
    };
    
    lines.push(Line::from(Span::styled(
        format!("󰇄 {} | {} | {}", hostname, os, kernel),
        Style::default().fg(Color::Magenta)
    )));
    
    lines
}

fn render_quote(config: &config::QuoteConfig) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    if !config.quotes.is_empty() {
        let mut rng = rand::thread_rng();
        if let Some(quote) = config.quotes.choose(&mut rng) {
            let max_width = 80;
            let words: Vec<&str> = quote.split_whitespace().collect();
            let mut current_line = String::new();
            
            for word in words {
                if current_line.len() + word.len() + 1 > max_width
                    && !current_line.is_empty() {
                    lines.push(Line::from(Span::styled(
                        current_line.clone(),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC)
                    )));
                    current_line.clear();
                }
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
            
            if !current_line.is_empty() {
                lines.push(Line::from(Span::styled(
                    current_line,
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC)
                )));
            }
        }
    } else {
        lines.push(Line::from(Span::styled(
            " Add quotes to your config!".to_string(),
            Style::default().fg(Color::DarkGray)
        )));
    }
    
    lines
}

fn render_uptime() -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    let uptime = if cfg!(target_os = "linux") {
        std::fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|content| {
                content.split_whitespace().next().and_then(|s| s.parse::<f64>().ok())
            })
            .map(|seconds| {
                let days = (seconds / 86400.0) as u64;
                let hours = ((seconds % 86400.0) / 3600.0) as u64;
                let minutes = ((seconds % 3600.0) / 60.0) as u64;
                
                if days > 0 {
                    format!("{}d {}h {}m", days, hours, minutes)
                } else if hours > 0 {
                    format!("{}h {}m", hours, minutes)
                } else {
                    format!("{}m", minutes)
                }
            })
            .unwrap_or_else(|| "unknown".to_string())
    } else {
        "not supported".to_string()
    };
    
    lines.push(Line::from(Span::styled(
        format!(" Uptime: {}", uptime),
        Style::default().fg(Color::Green)
    )));
    
    lines
}

fn render_disk_usage(config: &config::DiskUsageConfig) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    let disks = Disks::new_with_refreshed_list();
    
    for disk in &disks {
        if disk.mount_point().to_str() == Some(&config.path) {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            let usage_percent = if total > 0 {
                (used as f64 / total as f64 * 100.0) as u64
            } else {
                0
            };
            
            let total_gb = total / (1024 * 1024 * 1024);
            let used_gb = used / (1024 * 1024 * 1024);
            
            lines.push(Line::from(Span::styled(
                format!(" Disk: {} / {} GB ({}%)", used_gb, total_gb, usage_percent),
                Style::default().fg(Color::Cyan)
            )));
            break;
        }
    }
    
    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            format!(" Disk: path not found ({})", config.path),
            Style::default().fg(Color::DarkGray)
        )));
    }
    
    lines
}

fn render_memory() -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    
    let mut sys = System::new_all();
    sys.refresh_memory();
    
    let total = sys.total_memory();
    let used = sys.used_memory();
    let usage_percent = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };
    
    let total_gb = total / (1024 * 1024);
    let used_gb = used / (1024 * 1024);
    
    lines.push(Line::from(Span::styled(
        format!(" Memory: {} / {} MB ({}%)", used_gb, total_gb, usage_percent),
        Style::default().fg(Color::LightBlue)
    )));
    
    lines
}


