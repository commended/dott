mod config;

use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
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
}

impl App {
    fn new() -> App {
        let config = Config::load();
        let detected_shell_config = detect_shell_config();
        App {
            selected: 0,
            config,
            detected_shell_config,
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.config.entries.len();
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.config.entries.len() - 1;
        }
    }

    fn get_selected_item(&self) -> &config::MenuItem {
        &self.config.entries[self.selected]
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
                    let selected = app.get_selected_item();
                    
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
                _ => {}
            }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Calculate 10% from top for logo positioning
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Min(0),
            Constraint::Percentage(20),
        ])
        .split(size);

    // Get logo based on configuration
    let logo_text = match app.config.logo_type {
        config::LogoType::Default => DOTT_LOGO.to_string(),
        config::LogoType::Custom => {
            if let Some(ref path) = app.config.custom_logo_path {
                // Try to load custom ASCII art from file
                std::fs::read_to_string(path).unwrap_or_else(|_| DOTT_LOGO.to_string())
            } else {
                DOTT_LOGO.to_string()
            }
        }
        config::LogoType::Image => {
            // For image logos, show a placeholder message
            // Note: Kitty image protocol doesn't work well within ratatui
            // Images should be displayed before launching the TUI
            if let Some(ref path) = app.config.image_logo_path {
                format!("\n\n  [Image Logo: {}]\n  (Experimental: Use Kitty terminal)\n  (Image displayed before TUI launch)\n\n", path)
            } else {
                DOTT_LOGO.to_string()
            }
        }
    };

    // Center the logo
    let logo_lines: Vec<Line> = logo_text
        .lines()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(Color::Cyan))))
        .collect();

    let logo_line_count = logo_lines.len();
    let logo = Paragraph::new(logo_lines)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(logo, vertical_chunks[1]);

    // Calculate menu area (below logo)
    let menu_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(logo_line_count as u16 + 2),
            Constraint::Min(0),
        ])
        .split(vertical_chunks[1]);

    // Create centered menu
    let menu_width = 30;
    let menu_horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((size.width.saturating_sub(menu_width)) / 2),
            Constraint::Length(menu_width),
            Constraint::Length((size.width.saturating_sub(menu_width)) / 2),
        ])
        .split(menu_area[1]);

    let items: Vec<ListItem> = app
        .config
        .entries
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let (prefix, style) = if i == app.selected {
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
            ListItem::new(Line::from(Span::styled(format!("{}{}  ", prefix, item.name), style)))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default())
        .style(Style::default().fg(Color::White));

    f.render_widget(list, menu_horizontal[1]);

    // Display the command for the selected item
    let selected_item = &app.config.entries[app.selected];
    let command_text = if !selected_item.command.is_empty() {
        if selected_item.args.is_empty() {
            format!("Command: {}", selected_item.command)
        } else {
            format!("Command: {} {}", selected_item.command, selected_item.args.join(" "))
        }
    } else {
        // Special cases for built-in commands
        match selected_item.name.as_str() {
            "View Shell" => {
                if let Some(ref shell_config) = app.detected_shell_config {
                    format!("Command: nvim {}", shell_config)
                } else {
                    "Command: nvim <shell config>".to_string()
                }
            },
            "Edit Dott Config" => "Command: nvim ~/.config/dott/config.toml".to_string(),
            "Quit" => "Command: exit".to_string(),
            _ => String::new(),
        }
    };

    if !command_text.is_empty() {
        let command_display = Paragraph::new(command_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray));
        
        // Position command display below the menu
        let command_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(logo_line_count as u16 + app.config.entries.len() as u16 + 3),
                Constraint::Min(0),
            ])
            .split(vertical_chunks[1]);
        
        let command_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((size.width.saturating_sub(menu_width + 20)) / 2),
                Constraint::Length(menu_width + 20),
                Constraint::Length((size.width.saturating_sub(menu_width + 20)) / 2),
            ])
            .split(command_area[1]);
        
        f.render_widget(command_display, command_horizontal[1]);
    }

    // Render terminal colors module if configured (after menu items)
    if let Some(ref custom) = app.config.custom {
        if custom.terminal_colors.is_some() {
            let colors_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(logo_line_count as u16 + app.config.entries.len() as u16 + 4),
                    Constraint::Min(0),
                ])
                .split(vertical_chunks[1]);
            
            render_terminal_colors(f, colors_area[1], &app.config);
        }
    }

    // Help text at bottom
    let help = Paragraph::new("↑/k: Up | ↓/j: Down | Enter: Select | u: Reload Config | q/Esc: Quit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, vertical_chunks[2]);

    // Render clock directly under keybindings if configured
    if let Some(ref custom) = app.config.custom {
        if custom.clock.is_some() {
            render_clock_under_keybindings(f, vertical_chunks[2]);
        }
    }
}

fn render_clock_under_keybindings(f: &mut Frame, area: ratatui::layout::Rect) {
    let time = Local::now().format("%H:%M:%S").to_string();
    
    let clock_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .split(area);
    
    let clock = Paragraph::new(time)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan));
    
    f.render_widget(clock, clock_area[1]);
}

fn render_terminal_colors(f: &mut Frame, area: ratatui::layout::Rect, config: &Config) {
    if let Some(ref custom) = config.custom {
        if let Some(ref colors_config) = custom.terminal_colors {
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

            match colors_config.shape {
                config::ColorShape::Circles => {
                    // 1 row of 8 circles
                    let color_area = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(1),
                            Constraint::Min(0),
                        ])
                        .split(area);

                    let mut spans = vec![];
                    for color in &colors {
                        spans.push(Span::styled("● ", Style::default().fg(*color)));
                    }
                    
                    let line = Line::from(spans);
                    let colors_display = Paragraph::new(line)
                        .alignment(Alignment::Center);
                    
                    f.render_widget(colors_display, color_area[0]);
                }
                config::ColorShape::Squares => {
                    // 2 rows with 4 squares each
                    let color_area = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(2),
                            Constraint::Min(0),
                        ])
                        .split(area);

                    // First row
                    let mut spans_row1 = vec![];
                    for color in colors.iter().take(4) {
                        spans_row1.push(Span::styled("■ ", Style::default().fg(*color)));
                    }
                    
                    // Second row
                    let mut spans_row2 = vec![];
                    for color in colors.iter().skip(4).take(4) {
                        spans_row2.push(Span::styled("■ ", Style::default().fg(*color)));
                    }
                    
                    let lines = vec![Line::from(spans_row1), Line::from(spans_row2)];
                    let colors_display = Paragraph::new(lines)
                        .alignment(Alignment::Center);
                    
                    f.render_widget(colors_display, color_area[0]);
                }
            }
        }
    }
}
