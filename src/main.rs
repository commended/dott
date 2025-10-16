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
use sysinfo::System;

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

/// Display an image using Kitty's image protocol
/// This is an experimental feature that requires a terminal supporting Kitty's graphics protocol
fn display_kitty_image(path: &str) -> Result<(), String> {
    use std::fs;
    
    // Read the image file
    let image_data = fs::read(path).map_err(|e| format!("Failed to read image: {}", e))?;
    
    // Encode as base64
    let encoded = base64_encode(&image_data);
    
    // Kitty graphics protocol escape sequence
    // Format: \x1b_Gf=100,a=T,t=f;<base64_data>\x1b\\
    // where:
    // - f=100: format is PNG/auto-detect
    // - a=T: transmission medium is direct
    // - t=f: transmission is from file data
    let escape_seq = format!("\x1b_Gf=100,a=T,t=f;{}\x1b\\", encoded);
    
    print!("{}", escape_seq);
    io::stdout().flush().ok();
    
    Ok(())
}

/// Simple base64 encoding
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



/// Detect the user's shell and return the config file path (with ~ for display)
fn detect_shell_config() -> Option<String> {
    // Check the SHELL environment variable
    let shell = std::env::var("SHELL").ok()?;
    
    // Determine config file based on shell (using ~ notation)
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
        // Default to bashrc if we can't determine
        "~/.bashrc"
    };
    
    Some(config_file.to_string())
}

/// Get OS information (no version, just name)
fn get_os_info() -> String {
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    // Remove version numbers and extra info, keep just the base name
    os_name.split_whitespace().next().unwrap_or("Unknown").to_string()
}

/// Get WM/DE information
fn get_wm_de_info() -> String {
    // Try to detect window manager or desktop environment
    if let Ok(desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        return desktop.split(':').next().unwrap_or("Unknown").to_string();
    }
    if let Ok(desktop) = std::env::var("DESKTOP_SESSION") {
        return desktop;
    }
    if let Ok(wm) = std::env::var("WINDOWMANAGER") {
        return wm.split('/').last().unwrap_or("Unknown").to_string();
    }
    "Unknown".to_string()
}

/// Get CPU information (no version/generation, just model)
fn get_cpu_info() -> String {
    let sys = System::new_all();
    if let Some(cpu) = sys.cpus().first() {
        let brand = cpu.brand();
        // Extract just the base CPU name without detailed specs
        // E.g., "Intel Core i7" from "Intel(R) Core(TM) i7-9750H CPU @ 2.60GHz"
        let parts: Vec<&str> = brand.split_whitespace().collect();
        let mut result = Vec::new();
        for part in parts {
            if part.contains("@") || part.starts_with("CPU") {
                break;
            }
            if !part.contains("(R)") && !part.contains("(TM)") {
                let clean = part.replace("(R)", "").replace("(TM)", "");
                if !clean.is_empty() {
                    result.push(clean);
                }
            }
        }
        let cpu_name = result.join(" ");
        // Remove generation numbers (e.g., i7-9750H -> i7)
        cpu_name.split('-').next().unwrap_or("Unknown").to_string()
    } else {
        "Unknown".to_string()
    }
}

/// Get GPU information (basic, no version)
fn get_gpu_info() -> String {
    // Try to get GPU info from common Linux paths
    if let Ok(content) = std::fs::read_to_string("/sys/class/drm/card0/device/uevent") {
        for line in content.lines() {
            if line.starts_with("PCI_ID=") {
                // Basic fallback
                return "GPU".to_string();
            }
        }
    }
    
    // Try lspci if available
    if let Ok(output) = std::process::Command::new("lspci").output() {
        if let Ok(stdout) = String::from_utf8(output.stdout) {
            for line in stdout.lines() {
                if line.contains("VGA") || line.contains("3D") {
                    // Extract just the vendor and basic model
                    if let Some(info) = line.split(':').nth(2) {
                        let parts: Vec<&str> = info.trim().split_whitespace().collect();
                        // Get first 2-3 words (vendor and basic model)
                        let gpu_name = parts.iter().take(3).map(|s| *s).collect::<Vec<&str>>().join(" ");
                        // Remove version numbers and detailed specs
                        return gpu_name.split('[').next().unwrap_or("GPU").trim().to_string();
                    }
                }
            }
        }
    }
    
    "Unknown".to_string()
}

/// Get memory usage (used/total percentage)
fn get_memory_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_memory();
    let total = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0); // Convert to GB
    let used = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0); // Convert to GB
    let percentage = (used / total * 100.0) as u32;
    format!("{:.1}GB/{:.1}GB {}%", used, total, percentage)
}

/// Get system uptime
fn get_uptime_info() -> String {
    let uptime_seconds = System::uptime();
    let hours = uptime_seconds / 3600;
    let minutes = (uptime_seconds % 3600) / 60;
    
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

struct App {
    selected: usize,
    config: Config,
    detected_shell_config: Option<String>,
    // Flattened list of all entries with their group names
    all_entries: Vec<(String, config::MenuItem)>,
}

impl App {
    fn new() -> App {
        let config = Config::load();
        let detected_shell_config = detect_shell_config();
        
        // Build a flattened list of all entries in the order they appear in structure.build
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
            detected_shell_config,
            all_entries,
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
    // Create app state early to check for image logo
    let app = App::new();
    
    // If using image logo, display it before entering TUI (experimental feature)
    if let config::LogoType::Image = app.config.logo_type {
        if let Some(ref image_path) = app.config.image_logo_path {
            // Expand ~ to home directory
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let expanded_path = image_path.replace("~", &home);
            
            println!("\n");
            if let Err(e) = display_kitty_image(&expanded_path) {
                eprintln!("Warning: Failed to display image logo: {}", e);
                eprintln!("Note: This feature requires a terminal with Kitty graphics protocol support");
            }
            println!("\n");
            
            // Small delay to let image render
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let mut app = app;
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
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

        // Poll for events with a timeout to update the clock
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Down | KeyCode::Char('j') => app.next(),
                    KeyCode::Up | KeyCode::Char('k') => app.previous(),
                    KeyCode::Char('u') => {
                        // Exit TUI temporarily
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;

                        // Reload config
                        println!("Reloading config...");
                        app.config = Config::load();
                        app.selected = 0;
                        
                        // Rebuild all_entries list
                        app.all_entries.clear();
                        for module in app.config.get_ordered_modules() {
                            if let config::ModuleType::Entries(group_name) = module.module_type {
                                let entries = app.config.get_entries(&group_name);
                                for entry in entries {
                                    app.all_entries.push((group_name.clone(), entry.clone()));
                                }
                            }
                        }
                        
                        println!("✓ Config reloaded!");

                        // Small delay to let user see the message
                        std::thread::sleep(std::time::Duration::from_millis(500));

                        // Restore TUI
                        enable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            EnterAlternateScreen,
                            EnableMouseCapture
                        )?;
                        terminal.clear()?;
                    }
                    KeyCode::Enter => {
                    if let Some(selected) = app.get_selected_item() {
                    
                    match selected.name.as_str() {
                        "Quit" => return Ok(()),
                        "View Shell" => {
                            // Detect shell config and open in nvim
                            if let Some(shell_config) = detect_shell_config() {
                                // Exit TUI temporarily
                                disable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    LeaveAlternateScreen,
                                    DisableMouseCapture
                                )?;
                                terminal.show_cursor()?;

                                // Expand ~ to home directory
                                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                                let expanded_path = shell_config.replace("~", &home);

                                // Open shell config in nvim
                                let _ = std::process::Command::new("nvim")
                                    .arg(&expanded_path)
                                    .status();

                                // Restore TUI
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
                            // Exit TUI temporarily
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;

                            // Open dott config in nvim
                            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                            let config_path = format!("{}/.config/dott/config.toml", home);
                            let _ = std::process::Command::new("nvim")
                                .arg(&config_path)
                                .status();

                            // Restore TUI
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
                                // Exit TUI temporarily
                                disable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    LeaveAlternateScreen,
                                    DisableMouseCapture
                                )?;
                                terminal.show_cursor()?;

                                // Execute command with args
                                // Expand ~ in arguments
                                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                                let expanded_args: Vec<String> = selected.args.iter()
                                    .map(|arg| arg.replace("~", &home))
                                    .collect();
                                let args: Vec<&str> = expanded_args.iter().map(|s| s.as_str()).collect();
                                
                                let _ = std::process::Command::new(&selected.command)
                                    .args(&args)
                                    .status();

                                // Restore TUI
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
    
    // Get ordered modules from config
    let ordered_modules = app.config.get_ordered_modules();
    
    // Build the layout dynamically based on modules
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
                if let Some(ref custom) = app.config.custom {
                    let color_lines = render_terminal_colors_lines(&custom.terminal_colors);
                    lines.extend(color_lines);
                }
            }
            config::ModuleType::Clock => {
                if app.config.custom.is_some() {
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
                if app.config.custom.is_some() {
                    if let Some(selected_entry) = app.get_selected_item() {
                        let command_text = if selected_entry.command.is_empty() {
                            // Handle special entries that don't have commands
                            match selected_entry.name.as_str() {
                                "Quit" => "Exit application".to_string(),
                                "Edit Dott Config" => "Edit dott config in nvim".to_string(),
                                "View Shell" => format!("View shell config in nvim"),
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
            config::ModuleType::SysInfo => {
                // Only display if at least one option is enabled
                if app.config.should_display_sysinfo() {
                    if let Some(ref custom) = app.config.custom {
                        let sysinfo = &custom.sysinfo;
                        
                        if sysinfo.os {
                            let os = get_os_info();
                            lines.push(Line::from(vec![
                                Span::styled(" ", Style::default().fg(Color::Cyan)),
                                Span::styled(format!(" {}", os), Style::default().fg(Color::White))
                            ]));
                        }
                        
                        if sysinfo.wm {
                            let wm = get_wm_de_info();
                            lines.push(Line::from(vec![
                                Span::styled(" ", Style::default().fg(Color::Cyan)),
                                Span::styled(format!(" {}", wm), Style::default().fg(Color::White))
                            ]));
                        }
                        
                        if sysinfo.cpu {
                            let cpu = get_cpu_info();
                            lines.push(Line::from(vec![
                                Span::styled(" ", Style::default().fg(Color::Cyan)),
                                Span::styled(format!(" {}", cpu), Style::default().fg(Color::White))
                            ]));
                        }
                        
                        if sysinfo.gpu {
                            let gpu = get_gpu_info();
                            lines.push(Line::from(vec![
                                Span::styled("󰍛 ", Style::default().fg(Color::Cyan)),
                                Span::styled(format!(" {}", gpu), Style::default().fg(Color::White))
                            ]));
                        }
                        
                        if sysinfo.memory {
                            let memory = get_memory_info();
                            lines.push(Line::from(vec![
                                Span::styled(" ", Style::default().fg(Color::Cyan)),
                                Span::styled(format!(" {}", memory), Style::default().fg(Color::White))
                            ]));
                        }
                        
                        if sysinfo.uptime {
                            let uptime = get_uptime_info();
                            lines.push(Line::from(vec![
                                Span::styled(" ", Style::default().fg(Color::Cyan)),
                                Span::styled(format!(" {}", uptime), Style::default().fg(Color::White))
                            ]));
                        }
                    }
                }
            }
            config::ModuleType::Quit => {
                // Quit is a special entry that should be handled like other entries
                // This is for when "quit" is used as a standalone module
            }
        }
    }
    
    // Render everything centered
    let paragraph = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
    
    f.render_widget(paragraph, size);
}

fn get_logo_text(config: &Config) -> String {
    get_logo_text_with_type(&config.logo_type, config)
}

fn get_logo_text_with_type(logo_type: &config::LogoType, config: &Config) -> String {
    match logo_type {
        config::LogoType::Default => DOTT_LOGO.to_string(),
        config::LogoType::Custom => {
            if let Some(ref path) = config.custom_logo_path {
                std::fs::read_to_string(path).unwrap_or_else(|_| DOTT_LOGO.to_string())
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
            // First row
            let mut spans_row1 = vec![];
            for color in colors.iter().take(4) {
                spans_row1.push(Span::styled("■ ", Style::default().fg(*color)));
            }
            lines.push(Line::from(spans_row1));
            
            // Second row
            let mut spans_row2 = vec![];
            for color in colors.iter().skip(4).take(4) {
                spans_row2.push(Span::styled("■ ", Style::default().fg(*color)));
            }
            lines.push(Line::from(spans_row2));
        }
    }
    
    lines
}


