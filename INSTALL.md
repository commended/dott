# Installation Guide

## Prerequisites

- Rust 1.70 or later
- A terminal with ANSI color support
- Optional: `yazi` file manager for the "View Dotfiles" feature

## Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/commended/dott.git
   cd dott
   ```

2. Build the release version:
   ```bash
   cargo build --release
   ```

3. Install the binary:
   ```bash
   sudo cp target/release/dott /usr/local/bin/
   # Or to install for current user only:
   cp target/release/dott ~/.local/bin/
   ```

## Quick Start

Simply run:
```bash
dott
```

On first run, a configuration file will be created at `~/.config/dott/config.toml`.

## Customization

### Custom Logo

1. Create an ASCII art file (e.g., `~/.config/dott/my-logo.txt`)
2. Edit `~/.config/dott/config.toml`:
   ```toml
   logo_type = "custom"
   custom_logo_path = "/home/username/.config/dott/my-logo.txt"
   ```

### Custom Menu Items

Add new menu items in `~/.config/dott/config.toml`:

```toml
[[menu_items]]
name = "Edit Config"
command = "nvim"
args = ["/home/username/.config/dott/config.toml"]

[[menu_items]]
name = "Git Status"
command = "lazygit"
args = []
```

See `examples/config.toml` for more examples.

## Usage as Shell Startup

To make dott your terminal homepage, add to your `~/.bashrc` or `~/.zshrc`:

```bash
# Run dott on terminal start (skip for non-interactive shells)
if [[ $- == *i* ]]; then
    dott
fi
```

## Keyboard Shortcuts

- **↑** or **k**: Move selection up
- **↓** or **j**: Move selection down
- **Enter**: Execute selected menu item
- **q** or **Esc**: Quit dott

## Troubleshooting

### "yazi not found" error

Install yazi:
```bash
# Arch Linux
sudo pacman -S yazi

# Other distributions
cargo install --locked yazi-fm
```

Or remove/modify the "View Dotfiles" menu item in the config.
