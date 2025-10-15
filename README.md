# dott

A customizable TUI (Terminal User Interface) homepage for your terminal, written in Rust.

## Features

- 🎨 **Customizable ASCII Logo** - Use the default "dott" logo or provide your own ASCII art
- 📋 **Configurable Menu** - Add your own menu items with custom commands
- 🎯 **Centered Layout** - Logo positioned 20% from top, perfectly centered
- ⌨️ **Keyboard Navigation** - Vim-style (j/k) or arrow keys
- 📁 **Dotfile Management** - Built-in integration with yazi file manager to view `~/.config`
- 🌐 **Browser Launcher** - Automatically detects and launches your default browser
- 🐚 **Shell Config Editor** - Detects your shell and opens the config file in nvim
- 🔧 **Configuration File** - TOML-based configuration at `~/.config/dott/config.toml`

## Installation

```bash
cargo install --git https://github.com/commended/dott
```

### From Source

```bash
cargo build --release
sudo cp target/release/dott /usr/local/bin/
```

## Usage

Simply run:

```bash
dott
```

### Keyboard Controls

- **↑/k**: Move up
- **↓/j**: Move down  
- **Enter**: Select menu item
- **q/Esc**: Quit

## Configuration

The configuration file is automatically created at `~/.config/dott/config.toml` on first run.

### Default Menu Items

#### View Dotfiles
Opens the `~/.config` directory in yazi file manager for easy dotfile browsing.

#### Launch Browser
Automatically detects and launches your default browser. Detection order:
1. Checks `$BROWSER` environment variable
2. Searches for common browsers: firefox, google-chrome, chromium, brave, microsoft-edge, opera, vivaldi, safari

#### View Shell
Detects your shell from `$SHELL` environment variable and opens the appropriate config file in nvim:
- zsh → `~/.zshrc`
- bash → `~/.bashrc`
- fish → `~/.config/fish/config.fish`
- ksh → `~/.kshrc`
- tcsh → `~/.tcshrc`

### Default Configuration

```toml
logo_type = "default"

[[menu_items]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[menu_items]]
name = "Launch Browser"
command = ""
args = []

[[menu_items]]
name = "View Shell"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []
```

### Using a Custom Logo

1. Create your ASCII art file (e.g., `~/.config/dott/custom-logo.txt`)
2. Update your config:

```toml
logo_type = "custom"
custom_logo_path = "/home/username/.config/dott/custom-logo.txt"
```

### Adding Custom Menu Items

Add new menu items with commands:

```toml
[[menu_items]]
name = "Edit Config"
command = "nvim"
args = ["/home/username/.config/dott/config.toml"]

[[menu_items]]
name = "System Monitor"
command = "htop"
args = []
```

## ASCII Image Support

Based on concepts from [ascii-view](https://github.com/gouwsxander/ascii-view), dott includes support for displaying images as ASCII art. This functionality is designed to be extended for custom logo creation.

## Requirements

- Rust 1.70+ (for building)
- A terminal with ANSI color support
- Optional: `yazi` for the "View Dotfiles" feature

## Inspiration

- [ascii-view](https://github.com/gouwsxander/ascii-view) - ASCII image conversion concepts
- Similar to activation menus like `dotter` for managing dotfiles

## License

MIT

