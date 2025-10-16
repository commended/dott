# Development

This guide is for developers who want to contribute to dott or understand its internals.

## Table of Contents

- [Getting Started](#getting-started)
- [Project Structure](#project-structure)
- [Architecture](#architecture)
- [Building from Source](#building-from-source)
- [Code Style](#code-style)
- [Testing](#testing)
- [Contributing](#contributing)
- [Release Process](#release-process)

## Getting Started

### Prerequisites

- **Rust**: 1.70 or later
- **Git**: For version control
- **Code Editor**: VS Code, Neovim, or any Rust-compatible editor

### Setting Up Development Environment

1. **Clone the repository**:
   ```bash
   git clone https://github.com/commended/dott.git
   cd dott
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run in development mode**:
   ```bash
   cargo run
   ```

4. **Build optimized version**:
   ```bash
   cargo build --release
   ```

### Recommended Tools

- **rust-analyzer**: LSP for Rust (IDE support)
- **rustfmt**: Code formatter
- **clippy**: Linter
- **cargo-watch**: Auto-rebuild on changes

Install tools:
```bash
rustup component add rustfmt clippy
cargo install cargo-watch
```

## Project Structure

```
dott/
├── Cargo.toml          # Project dependencies and metadata
├── Cargo.lock          # Locked dependency versions
├── LICENSE             # MIT License
├── README.md           # Project readme
├── src/
│   ├── main.rs         # Main application logic and TUI
│   └── config.rs       # Configuration management
├── examples/
│   ├── config.toml                    # Example configuration
│   ├── config-with-image-logo.toml    # Image logo example
│   └── custom-logo.txt                # Custom ASCII logo example
└── wiki/               # Documentation (GitHub wiki)
    ├── Home.md
    ├── Installation.md
    ├── Configuration.md
    ├── Usage.md
    ├── Features.md
    ├── Customization.md
    ├── Examples.md
    ├── Troubleshooting.md
    ├── FAQ.md
    └── Development.md
```

## Architecture

### Overview

Dott follows a simple architecture:

```
┌─────────────────────────────────────┐
│         main.rs (Entry Point)       │
├─────────────────────────────────────┤
│  Config Loading (config.rs)         │
├─────────────────────────────────────┤
│  TUI Setup (ratatui + crossterm)    │
├─────────────────────────────────────┤
│  Event Loop                          │
│  ├── Handle keyboard input           │
│  ├── Update application state        │
│  └── Render UI                       │
├─────────────────────────────────────┤
│  Command Execution                   │
│  ├── Exit TUI                        │
│  ├── Run command                     │
│  └── Restore TUI                     │
└─────────────────────────────────────┘
```

### Key Components

#### 1. Configuration System (`config.rs`)

**Purpose**: Load and manage user configuration

**Key structures**:
```rust
pub struct Config {
    pub logo_type: LogoType,
    pub custom_logo_path: Option<String>,
    pub image_logo_path: Option<String>,
    pub menu_items: Vec<MenuItem>,
    pub terminal_colors: Option<TerminalColorsConfig>,
    pub clock: Option<ClockConfig>,
}

pub struct MenuItem {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}
```

**Responsibilities**:
- Load configuration from `~/.config/dott/config.toml`
- Parse TOML format using serde
- Provide defaults if config missing/invalid
- Save configuration

#### 2. Application State (`main.rs`)

**Purpose**: Manage TUI state

**Key structure**:
```rust
struct App {
    selected: usize,
    config: Config,
    detected_shell_config: Option<String>,
}
```

**Methods**:
- `new()`: Initialize app with config
- `next()`: Move selection down
- `previous()`: Move selection up
- `get_selected_item()`: Get current menu item

#### 3. User Interface (`main.rs`)

**Purpose**: Render TUI and handle layout

**Key function**: `ui(f: &mut Frame, app: &App)`

**Responsibilities**:
- Render logo (ASCII or placeholder for image)
- Render menu items with selection highlight
- Render command preview
- Render optional features (colors, clock)
- Render help text

#### 4. Event Loop (`run_app`)

**Purpose**: Main application loop

**Flow**:
1. Draw UI
2. Poll for events (100ms timeout for clock updates)
3. Handle keyboard input
4. Update state
5. Execute commands if needed
6. Repeat

#### 5. Command Execution

**Purpose**: Run selected commands

**Process**:
1. Disable raw mode
2. Leave alternate screen
3. Show cursor
4. Execute command with args
5. Wait for user to press 'q'
6. Enable raw mode
7. Enter alternate screen
8. Clear screen
9. Resume TUI

### Dependencies

#### Core Dependencies

- **ratatui** (0.28): TUI framework
  - Provides widgets, layout, styling
  - Backend-agnostic terminal rendering

- **crossterm** (0.28): Terminal handling
  - Raw mode control
  - Event handling (keyboard, mouse)
  - Terminal state management

- **serde** (1.0): Serialization framework
  - Deserialize TOML configuration
  - Derive macros for automatic implementation

- **toml** (0.8): TOML parser
  - Parse configuration files
  - Generate TOML from structures

- **chrono** (0.4): Date and time
  - Clock feature implementation

## Building from Source

### Debug Build

```bash
cargo build
```

Binary location: `target/debug/dott`

### Release Build

```bash
cargo build --release
```

Binary location: `target/release/dott`

### Development Build with Auto-Reload

```bash
cargo watch -x run
```

Automatically rebuilds and runs on file changes.

### Check for Errors

```bash
cargo check
```

Faster than full build, checks for compilation errors.

## Code Style

### Formatting

Use rustfmt for consistent formatting:

```bash
cargo fmt
```

Configuration in `rustfmt.toml` (if present) or use defaults.

### Linting

Use clippy for linting:

```bash
cargo clippy
```

Fix warnings:
```bash
cargo clippy --fix
```

### Style Guidelines

1. **Naming Conventions**:
   - `snake_case` for functions and variables
   - `PascalCase` for types and structs
   - `SCREAMING_SNAKE_CASE` for constants

2. **Comments**:
   - Use `///` for documentation comments
   - Use `//` for inline comments
   - Document public APIs

3. **Error Handling**:
   - Use `Result<T, E>` for fallible operations
   - Provide meaningful error messages
   - Gracefully handle errors (don't panic in library code)

4. **Module Organization**:
   - Keep related functionality together
   - Use private functions for internal logic
   - Expose minimal public API

### Example Code Style

```rust
/// Loads configuration from the user's config directory.
///
/// If the configuration file doesn't exist, creates a default one.
/// Returns the loaded configuration or default on error.
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
```

## Testing

### Running Tests

Currently, dott doesn't have automated tests. Future contributions should include tests.

To set up testing:

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Manual Testing

1. **Test configuration loading**:
   - Missing config file
   - Invalid TOML syntax
   - Valid config with all features

2. **Test menu navigation**:
   - Up/down with j/k
   - Up/down with arrows
   - Wrapping at ends

3. **Test command execution**:
   - Simple commands
   - Commands with arguments
   - Interactive commands
   - Failed commands

4. **Test features**:
   - Logo types (default, custom, image)
   - Terminal colors (circles, squares)
   - Clock display
   - Shell detection

### Test Cases to Add

Future contributors should add tests for:

- Configuration parsing
- Menu item navigation
- Path expansion (`~` to `$HOME`)
- Shell detection algorithm
- Error handling
- Default configuration generation

## Contributing

### How to Contribute

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/my-feature`
3. **Make your changes**
4. **Test thoroughly**
5. **Commit with clear messages**: `git commit -m "Add feature X"`
6. **Push to your fork**: `git push origin feature/my-feature`
7. **Open a Pull Request**

### Contribution Guidelines

1. **Code Quality**:
   - Run `cargo fmt` before committing
   - Run `cargo clippy` and fix warnings
   - Write clear, documented code
   - Add tests for new features

2. **Commit Messages**:
   - Use clear, descriptive commit messages
   - Start with verb: "Add", "Fix", "Update", "Remove"
   - Reference issues: "Fix #123: Description"

3. **Pull Requests**:
   - Describe what and why
   - Reference related issues
   - Include screenshots for UI changes
   - Keep PRs focused (one feature/fix per PR)

4. **Documentation**:
   - Update wiki documentation if needed
   - Add code comments for complex logic
   - Update README.md if adding major features

### Areas for Contribution

- **Features**: New functionality (see planned features in Features.md)
- **Bug fixes**: Fix reported issues
- **Documentation**: Improve wiki pages
- **Testing**: Add test coverage
- **Performance**: Optimize slow code paths
- **Refactoring**: Improve code structure

### Bug Reports

When reporting bugs:
1. Check if already reported
2. Provide clear reproduction steps
3. Include system information
4. Include error messages
5. Attach configuration file (if relevant)

### Feature Requests

When requesting features:
1. Search for existing requests
2. Describe the use case
3. Explain expected behavior
4. Provide examples or mockups

## Release Process

### Versioning

Dott follows [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.0.0): Breaking changes
- **MINOR** (0.1.0): New features, backwards compatible
- **PATCH** (0.0.1): Bug fixes, backwards compatible

### Creating a Release

1. **Update version in Cargo.toml**:
   ```toml
   [package]
   version = "0.2.0"
   ```

2. **Update CHANGELOG** (if exists):
   ```markdown
   ## [0.2.0] - 2024-01-15
   ### Added
   - New feature X
   ### Fixed
   - Bug Y
   ```

3. **Build and test**:
   ```bash
   cargo build --release
   cargo test
   ```

4. **Commit and tag**:
   ```bash
   git commit -am "Release v0.2.0"
   git tag -a v0.2.0 -m "Release version 0.2.0"
   git push origin main --tags
   ```

5. **Create GitHub release**:
   - Go to GitHub repository
   - Click "Releases" → "Create a new release"
   - Select tag, add release notes
   - Attach binaries (optional)

6. **Publish to crates.io** (if applicable):
   ```bash
   cargo publish
   ```

## Debugging

### Logging

Add logging for development:

```rust
// Add to Cargo.toml
[dependencies]
log = "0.4"
env_logger = "0.10"

// In main.rs
use log::{info, debug, error};

fn main() {
    env_logger::init();
    info!("Starting dott");
    // ...
}
```

Run with logging:
```bash
RUST_LOG=debug cargo run
```

### Debugging TUI

TUI debugging is tricky because the app owns the terminal. Options:

1. **Write to file**:
   ```rust
   use std::fs::OpenOptions;
   use std::io::Write;
   
   let mut file = OpenOptions::new()
       .create(true)
       .append(true)
       .open("/tmp/dott-debug.log")
       .unwrap();
   writeln!(file, "Debug: {:?}", value).ok();
   ```

2. **Use a separate terminal**: Run dott in one terminal, tail debug log in another:
   ```bash
   # Terminal 1
   cargo run
   
   # Terminal 2
   tail -f /tmp/dott-debug.log
   ```

3. **GDB/LLDB**: Use debugger with TUI mode:
   ```bash
   rust-gdb target/debug/dott
   ```

## Resources

### Rust Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings)

### Ratatui Resources

- [Ratatui Documentation](https://docs.rs/ratatui/)
- [Ratatui Book](https://ratatui.rs/)
- [Ratatui Examples](https://github.com/ratatui-org/ratatui/tree/main/examples)

### Crossterm Resources

- [Crossterm Documentation](https://docs.rs/crossterm/)
- [Crossterm Examples](https://github.com/crossterm-rs/crossterm/tree/master/examples)

## Getting Help

For development questions:

1. **Read the code**: Source is well-structured
2. **Check issues**: Existing discussions may help
3. **Ask in issues**: Create a discussion issue
4. **Discord/Chat**: If available, join community chat

## See Also

- [Home](Home.md) - Project overview
- [Configuration](Configuration.md) - Configuration system details
- [Features](Features.md) - Feature descriptions
- [Contributing Guide](https://github.com/commended/dott/blob/main/CONTRIBUTING.md) - If exists
