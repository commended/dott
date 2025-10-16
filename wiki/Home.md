# Dott Wiki

Welcome to the **dott** documentation! Dott is a beautiful and fast Terminal User Interface (TUI) application written in Rust, designed to help you manage your dotfiles and execute common commands with ease.

## What is dott?

Dott provides an elegant, keyboard-driven menu interface that lets you:
- Navigate and edit your dotfiles
- Execute custom commands and scripts
- Launch your favorite tools (yazi, neovim, lazygit, etc.)
- Customize the interface with logos, colors, and themes
- Work efficiently with vim-style keyboard shortcuts

## Quick Links

- **[Installation](Installation.md)** - Get dott up and running
- **[Configuration](Configuration.md)** - Configure dott to your liking
- **[Usage](Usage.md)** - Learn how to use dott
- **[Features](Features.md)** - Explore all features
- **[Customization](Customization.md)** - Make dott your own
- **[Examples](Examples.md)** - Example configurations
- **[Troubleshooting](Troubleshooting.md)** - Common issues and solutions
- **[FAQ](FAQ.md)** - Frequently asked questions
- **[Development](Development.md)** - Contributing to dott

## Key Features

- 🎨 **Beautiful Interface** - Cyan-accented TUI with custom ASCII art logos
- ⌨️ **Vim-style Navigation** - Use `j`/`k` or arrow keys for navigation
- ⚡ **Fast & Lightweight** - Written in Rust for maximum performance
- 🔧 **Highly Configurable** - TOML-based configuration for easy customization
- 🖼️ **Logo Support** - Default ASCII art, custom ASCII, or images (experimental)
- 🎨 **Terminal Colors** - Display terminal color palette with circles or squares
- 🕐 **Live Clock** - Optional real-time clock display
- 🔍 **Shell Detection** - Automatically detects and opens your shell config

## Getting Started

1. **Install dott** - See [Installation](Installation.md)
2. **Run it** - Simply execute `dott` in your terminal
3. **Configure** - Edit `~/.config/dott/config.toml` to customize
4. **Enjoy** - Navigate with `j`/`k`, select with Enter, quit with `q`

## Requirements

- Rust 1.70+ (for building from source)
- A terminal with ANSI color support
- Optional: yazi (file manager)
- Optional: neovim (text editor)
- Optional: Kitty terminal (for image logo support)

## Community

Found a bug or have a feature request? Please open an issue on [GitHub](https://github.com/commended/dott).

## License

Dott is licensed under the MIT License. See [LICENSE](../LICENSE) for details.
