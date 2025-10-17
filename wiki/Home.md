# Welcome to the dott Wiki

**dott** is a beautiful and fast terminal user interface (TUI) written in Rust that provides a customizable launcher for your most frequently used terminal commands and tools.

## What is dott?

dott transforms your terminal into an elegant command launcher with:
- **Beautiful UI**: Clean, minimal interface with customizable logos and colors
- **Fast Navigation**: Keyboard-driven navigation with vim-style keybindings
- **Highly Customizable**: Configure every aspect through a simple TOML config file
- **Modular Design**: Mix and match modules to create your perfect layout
- **System Information**: Display system stats, uptime, memory usage, and more
- **Multiple Entry Groups**: Organize commands into logical groups

## Quick Start

Install dott with cargo:

```bash
cargo install --git https://github.com/commended/dott
```

Run it:

```bash
~/.cargo/bin/dott
```

## Key Features

- **Highlighted Selection**: Currently selected items stand out with cyan highlighting
- **Keyboard Navigation**: Navigate with arrow keys or vim-style `j`/`k`
- **Quick Actions**: Press Enter to execute commands instantly
- **Hot Reload**: Press `u` to reload config without restarting
- **Custom Layouts**: Define your own structure with various modules
- **ASCII Art Logos**: Default logo, custom ASCII art, or Kitty terminal images
- **Terminal Colors**: Beautiful color palettes in circles or squares
- **Clock Display**: Show current time in your TUI
- **Creative Modules**: System info, quotes, uptime, memory, and disk usage

## Documentation

This wiki contains everything you need to know about dott:

- **Getting Started** - Installation, first run, and basic usage
- **Configuration** - Complete guide to the config.toml file
- **Modules** - Detailed documentation of all available modules
- **Customization** - Advanced customization tips and tricks
- **Troubleshooting** - Solutions to common problems

## Requirements

- Rust 1.70+ (for building from source)
- A terminal with ANSI color support
- Optional: Kitty terminal (for image logo support)
- Optional: yazi, neovim, btop, lazygit (for example commands)

## Contributing

dott is open source under the MIT license. Contributions are welcome!

## Quick Example

Here's a minimal config to get you started:

```toml
logo_type = "default"

[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "help"

[[entries]]
name = "Terminal File Manager"
command = "yazi"
args = ["~"]

[[entries]]
name = "Edit Config"
command = "nvim"
args = ["~/.config/dott/config.toml"]

[[entries]]
name = "Quit"
command = ""
args = []

[custom]

[custom.selected]
```

## Getting Help

If you encounter any issues or have questions, please check the Troubleshooting page or open an issue on GitHub.
