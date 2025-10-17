# dott

[![License: MIT](https://img.shields.io/badge/License-MIT-white.svg)](https://opensource.org/licenses/MIT)
![GitHub Repo stars](https://img.shields.io/github/stars/commended/dott)

a beautiful and fast tui written in rust

## Showcase
![alt text](https://github.com/commended/dott/blob/main/showcase/dottshowcase1.png)


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
~/.cargo/bin/dott
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

Default configuration:
```toml
logo_type = "default"

[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "clock"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "help"

[[entries]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[entries]]
name = "Edit Neovim Config"
command = "nvim"
args = ["~/.config/nvim/init.lua"]

[[entries]]
name = "Edit Shell Config"
command = "nvim"
args = ["~/.bashrc"]

[[entries]]
name = "System Monitor"
command = "btop"
args = []

[[entries]]
name = "Git Status"
command = "lazygit"
args = []

[[entries]]
name = "Configure Dott"
command = "nvim"
args = ["~/.config/dott/config.toml"]

[[entries]]
name = "Quit"
command = ""
args = []

[custom]

[custom.terminal_colors]
shape = "circles"

[custom.clock]

[custom.selected]

[custom.break]
lines = 2
```

## Requirements

- Rust 1.70+ (for building)
- A terminal with ANSI color support
- yazi (optional, for file manager)
- neovim (optional, for config editing)

