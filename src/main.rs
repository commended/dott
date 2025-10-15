mod ascii_image;
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
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

const DOTT_LOGO: &str = r#"
     _       _   _   
  __| | ___ | |_| |_ 
 / _` |/ _ \| __| __|
| (_| | (_) | |_| |_ 
 \__,_|\___/ \__|\__|
"#;

/// Detect the user's default browser
fn detect_browser() -> Option<String> {
    // Check environment variables first
    if let Ok(browser) = std::env::var("BROWSER") {
        return Some(browser);
    }
    
    // Try common browsers in order of preference
    let browsers = vec![
        "firefox",
        "google-chrome",
        "chromium",
        "brave",
        "microsoft-edge",
        "opera",
        "vivaldi",
        "safari",
    ];
    
    for browser in browsers {
        if std::process::Command::new("which")
            .arg(browser)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            return Some(browser.to_string());
        }
    }
    
    None
}

/// Detect the user's shell and return the config file path
fn detect_shell_config() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    
    // Check the SHELL environment variable
    let shell = std::env::var("SHELL").ok()?;
    
    // Determine config file based on shell
    let config_file = if shell.contains("zsh") {
        ".zshrc"
    } else if shell.contains("bash") {
        ".bashrc"
    } else if shell.contains("fish") {
        ".config/fish/config.fish"
    } else if shell.contains("ksh") {
        ".kshrc"
    } else if shell.contains("tcsh") {
        ".tcshrc"
    } else {
        // Default to bashrc if we can't determine
        ".bashrc"
    };
    
    Some(format!("{}/{}", home, config_file))
}

struct App {
    selected: usize,
    config: Config,
}

impl App {
    fn new() -> App {
        let config = Config::load();
        App {
            selected: 0,
            config,
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.config.menu_items.len();
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.config.menu_items.len() - 1;
        }
    }

    fn get_selected_item(&self) -> &config::MenuItem {
        &self.config.menu_items[self.selected]
    }
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();
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

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Enter => {
                    let selected = app.get_selected_item();
                    
                    match selected.name.as_str() {
                        "Quit" => return Ok(()),
                        "Launch Browser" => {
                            // Detect and launch the browser
                            if let Some(browser) = detect_browser() {
                                // Exit TUI temporarily
                                disable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    LeaveAlternateScreen,
                                    DisableMouseCapture
                                )?;
                                terminal.show_cursor()?;

                                // Launch browser
                                let _ = std::process::Command::new(&browser)
                                    .spawn();

                                // Restore TUI
                                enable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    EnterAlternateScreen,
                                    EnableMouseCapture
                                )?;
                            }
                        }
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

                                // Open shell config in nvim
                                let _ = std::process::Command::new("nvim")
                                    .arg(&shell_config)
                                    .status();

                                // Restore TUI
                                enable_raw_mode()?;
                                execute!(
                                    terminal.backend_mut(),
                                    EnterAlternateScreen,
                                    EnableMouseCapture
                                )?;
                            }
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
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Calculate 20% from top
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
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
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(format!("  {}  ", item.name), style)))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Menu")
                .style(Style::default().fg(Color::Cyan)),
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(list, menu_horizontal[1]);

    // Help text at bottom
    let help = Paragraph::new("↑/k: Up | ↓/j: Down | Enter: Select | q/Esc: Quit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));
    f.render_widget(help, vertical_chunks[2]);
}
