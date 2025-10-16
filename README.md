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

- **Structure**: Define the order of modules (logo, entries, clock, colors, help, break)
- **Logo**: Choose between default, custom ASCII art, or image (Kitty protocol). Can be set via top-level `logo_type` or directly in `structure.build` (e.g., `"logo:default"`, `"logo:custom"`, `"logo:image"`)
- **Entries**: Terminal commands with name, command, and arguments
- **Multiple Entry Groups**: Create separate entry groups (entries, entries2, entries3, etc.)
- **Custom Modules**: Terminal colors, clock, help text, and configurable breaks (must be declared to use)
- **Creative Modules**: System info, uptime, memory usage, disk usage, and quotes (must be declared to use)
- **Break Lines**: Configure how many empty lines each break adds (default: 2)

Example structure with multiple entry groups:
```toml
[structure]
position = "center"

[[structure.build]]
module = "logo:default"   # Logo type can be specified here

[[structure.build]]
module = "clock"

[[structure.build]]
module = "entries"        # First group of commands

[[structure.build]]
module = "break"          # Adds empty lines (default: 2, configurable)

[[structure.build]]
module = "entries2"       # Second group of commands

[[structure.build]]
module = "colors"

[[structure.build]]
module = "help"

# Logo configuration
logo_type = "default"

# Custom modules must be declared
[custom]

[custom.terminal_colors]
shape = "circles"

[custom.clock]

[custom.break]
lines = 2            # Configure break to add 2 empty lines (default)
```

See [CUSTOM_MODULES.md](CUSTOM_MODULES.md) for detailed configuration options.

## Requirements

- Rust 1.70+ (for building)
- A terminal with ANSI color support
- yazi (optional, for file manager)
- neovim (optional, for config editing)

