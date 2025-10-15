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

struct App {
    selected: usize,
    menu_items: Vec<String>,
}

impl App {
    fn new() -> App {
        App {
            selected: 0,
            menu_items: vec![
                "View Dotfiles".to_string(),
                "Configure".to_string(),
                "About".to_string(),
                "Quit".to_string(),
            ],
        }
    }

    fn next(&mut self) {
        self.selected = (self.selected + 1) % self.menu_items.len();
    }

    fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        } else {
            self.selected = self.menu_items.len() - 1;
        }
    }

    fn select(&self) -> String {
        self.menu_items[self.selected].clone()
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
                    let selected = app.select();
                    match selected.as_str() {
                        "Quit" => return Ok(()),
                        "View Dotfiles" => {
                            // Exit TUI and launch yazi
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;

                            // Launch yazi
                            let _ = std::process::Command::new("yazi")
                                .arg(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                                .status();

                            // Restore TUI
                            enable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                EnterAlternateScreen,
                                EnableMouseCapture
                            )?;
                        }
                        _ => {}
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

    // Center the logo
    let logo_lines: Vec<Line> = DOTT_LOGO
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
            ListItem::new(Line::from(Span::styled(format!("  {}  ", item), style)))
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
