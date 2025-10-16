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
                    if let Some(ref colors_config) = custom.terminal_colors {
                        let color_lines = render_terminal_colors_lines(colors_config);
                        lines.extend(color_lines);
                    }
                }
            }
            config::ModuleType::Clock => {
                if let Some(ref custom) = app.config.custom {
                    if custom.clock.is_some() {
                        let time = Local::now().format("%H:%M:%S").to_string();
                        lines.push(Line::from(Span::styled(time, Style::default().fg(Color::Cyan))));
                    }
                }
            }
            config::ModuleType::Help => {
                lines.push(Line::from(Span::styled(
                    "↑/k: Up | ↓/j: Down | Enter: Select | u: Reload Config | q/Esc: Quit".to_string(),
                    Style::default().fg(Color::DarkGray)
                )));
            }
            config::ModuleType::Break => {
                let break_lines = app.config.get_break_lines();
                for _ in 0..break_lines {
                    lines.push(Line::from(""));
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


