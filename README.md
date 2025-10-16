# dott

[![License: MIT](https://img.shields.io/badge/License-MIT-white.svg)](https://opensource.org/licenses/MIT)
![GitHub Repo stars](https://img.shields.io/github/stars/commended/dott)

a beautiful and fast tui written in rust


## Installation

```bash
cargo install --git https://github.com/commended/dott
```

### From Source

```bash
cargo build --release
sudo cp target/release/dott /usr/local/bin/
```

## Updating

```bash
cargo install --git https://github.com/commended/dott --force
```

## Usage

Simply run:

```bash
dott
```

The TUI features:
- **Highlighted selection**: Selected menu items are highlighted with a cyan background
- **Keyboard navigation**: Use arrow keys or vim-style `j`/`k` to navigate
- **Quick actions**: Press Enter to execute the selected menu item
- **Reload Config**: Press `u` to reload the config without restarting the app
- **Customizable Layout**: Configure the order and appearance of modules
- **Multiple Entry Groups**: Organize commands into separate groups

## Configuration

The config file is located at `~/.config/dott/config.toml`. You can customize:

- **Logo**: Choose between default, custom ASCII art, or image (Kitty protocol). Can be set via top-level `logo_type` or directly in `structure.build` (e.g., `"logo:default"`, `"logo:custom"`, `"logo:image"`)
- **Structure**: Define the order of modules (logo, entries, clock, colors, help, break)
- **Entries**: Terminal commands with name, command, and arguments
- **Multiple Entry Groups**: Create separate entry groups (entries, entries2, entries3, etc.)
- **Custom Modules**: Terminal colors, clock, help text, and configurable breaks
- **Break Lines**: Configure how many empty lines each break adds (default: 2)

Example structure with multiple entry groups:
```toml
[structure.build]
1 = "logo:default"   # Logo type can be specified here
2 = "clock"
3 = "entries"        # First group of commands
4 = "break"          # Adds empty lines (default: 2, configurable)
5 = "entries2"       # Second group of commands
6 = "colors"
7 = "help"

[custom.break]
lines = 2            # Configure break to add 2 empty lines (default)
```

See [CUSTOM_MODULES.md](CUSTOM_MODULES.md) for detailed configuration options.

## Requirements

- Rust 1.70+ (for building)
- A terminal with ANSI color support
- yazi (optional, for file manager)
- neovim (optional, for config editing)

