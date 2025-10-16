# Frequently Asked Questions (FAQ)

Common questions about dott and their answers.

## General Questions

### What is dott?

Dott is a beautiful, fast Terminal User Interface (TUI) application written in Rust. It provides a menu-driven interface for managing dotfiles and executing commands. Think of it as a launcher for your most-used terminal commands and tools.

### Why use dott?

- **Speed**: Faster than remembering and typing long commands
- **Organization**: Keep all your dotfile tools in one place
- **Efficiency**: Vim-style keyboard navigation
- **Aesthetics**: Beautiful interface that makes terminal work enjoyable
- **Customization**: Highly configurable to fit your workflow

### What does "dott" stand for?

While not explicitly stated, it's likely a play on "dot" (as in dotfiles) with an extra 't' for Terminal UI or just stylistic choice.

### Is dott free?

Yes! Dott is open-source software licensed under the MIT License. It's completely free to use, modify, and distribute.

## Installation & Setup

### What are the system requirements?

- **OS**: Linux, macOS, Windows (via WSL)
- **Rust**: 1.70 or later (only for building)
- **Terminal**: Any modern terminal with ANSI color support
- **Optional**: yazi, neovim, lazygit (for full functionality)

### Can I use dott without Rust installed?

No, you need Rust to build and install dott. However, once installed, the compiled binary doesn't require Rust to run.

### Does dott work on Windows?

Yes, through Windows Subsystem for Linux (WSL). Native Windows support is not currently available but may be added in the future.

### How do I update dott?

```bash
cargo install --git https://github.com/commended/dott --force
```

The `--force` flag will overwrite the existing installation.

### Can I install dott system-wide?

Yes, after building:
```bash
sudo cp target/release/dott /usr/local/bin/
```

Or use cargo install which installs to `~/.cargo/bin/`.

## Configuration

### Where is the configuration file?

```
~/.config/dott/config.toml
```

It's automatically created on first run if it doesn't exist.

### Can I have multiple configurations?

While dott only loads `config.toml`, you can maintain multiple configs and swap them:

```bash
# Switch configurations
cp ~/.config/dott/work.toml ~/.config/dott/config.toml

# Or use symlinks
ln -sf ~/.config/dott/work.toml ~/.config/dott/config.toml
```

### How do I reset to default configuration?

```bash
rm ~/.config/dott/config.toml
dott  # Will recreate with defaults
```

### Can I use environment variables in configuration?

Not directly in the TOML file, but you can use them in commands:

```toml
[[menu_items]]
name = "Open Project"
command = "bash"
args = ["-c", "cd $PROJECT_DIR && nvim"]
```

### Does configuration support comments?

Yes! TOML supports comments with `#`:

```toml
# This is a comment
[[menu_items]]
name = "Example"  # This is also a comment
command = "echo"
args = ["hello"]
```

## Usage

### What keyboard shortcuts are available?

- `j` or `↓`: Move down
- `k` or `↑`: Move up
- `Enter`: Select/execute
- `q` or `Esc`: Quit

That's it! Dott is intentionally simple.

### Can I use the mouse?

Currently, no. Dott is keyboard-only, following the terminal-first philosophy.

### How do I return to dott after running a command?

Press `q` when you see "Press 'q' to return to menu...". If the command is still running, exit it first (usually with `q` or `Ctrl+C`).

### Can I run dott commands from outside dott?

No, dott is an interactive TUI. However, you can script the underlying commands directly in your shell.

### Does dott support command history?

Not currently. Each launch of dott is independent.

### Can I search menu items?

Not yet. This is a potential feature for future releases.

## Features

### What's the difference between logo types?

- **Default**: Built-in dott ASCII art
- **Custom**: Your own ASCII art from a file
- **Image**: Real images (experimental, requires Kitty terminal)

### Why isn't my image logo showing?

Image logos require:
1. A terminal with Kitty graphics protocol (Kitty, WezTerm, some Konsole versions)
2. Correct path to valid image file (PNG, JPEG)
3. Image is shown before TUI launches, not within it

### What are the terminal colors for?

It's a visual feature that displays your terminal's 8 basic colors. Useful for:
- Verifying color scheme
- Aesthetic purposes
- Testing terminal compatibility

### Do I need the clock feature?

No, it's entirely optional. Enable it if you want a real-time clock display.

### Can I customize the colors of the interface?

The interface uses cyan accents and inherits your terminal's color scheme. To change colors, modify your terminal emulator's color theme.

## Customization

### How do I create custom ASCII art?

Use online tools like:
- http://patorjk.com/software/taag/
- https://textart.io/

Or create it manually in a text editor.

### Can I use Unicode characters in logos?

Yes, but stick to common Unicode for compatibility. Extended Unicode might not display correctly in all terminals.

### What image formats are supported for image logos?

PNG, JPEG, GIF, and most common image formats. The Kitty graphics protocol auto-detects the format.

### How do I organize many menu items?

- Group related items together
- Use consistent naming
- Consider multiple config files for different contexts
- Keep frequently-used items near the top

### Can I change the highlight color?

Not currently through configuration. The cyan highlight is hardcoded. This might be configurable in future releases.

## Troubleshooting

### Why isn't dott starting?

Common causes:
1. Not in PATH (add `~/.cargo/bin` to PATH)
2. Configuration file error (check TOML syntax)
3. Missing dependencies (install Rust 1.70+)

### My commands don't work in dott but work in shell. Why?

Usually a PATH issue. Commands in dott run with your shell's PATH. Solutions:
- Use absolute paths: `/usr/local/bin/mycommand`
- Ensure PATH is set in shell config
- Source your shell config before running dott

### The TUI looks broken. What's wrong?

Check:
1. Terminal size (should be at least 80 columns)
2. ANSI color support (most modern terminals have this)
3. Terminal emulator compatibility (try Alacritty, Kitty, or iTerm2)

### Can I run dott in tmux/screen?

Yes, dott works in terminal multiplexers. If you have issues with colors, set:
```bash
export TERM=screen-256color  # for screen
export TERM=tmux-256color    # for tmux
```

## Development & Contributing

### Is dott actively maintained?

Check the [GitHub repository](https://github.com/commended/dott) for recent activity, issues, and pull requests.

### Can I contribute to dott?

Yes! Contributions are welcome. See [Development Guide](Development.md) for details.

### Where do I report bugs?

Open an issue on [GitHub Issues](https://github.com/commended/dott/issues).

### What language is dott written in?

Rust! It uses the ratatui library for the TUI and crossterm for terminal handling.

### Can I fork dott for my own version?

Absolutely! Dott is MIT licensed. You're free to fork, modify, and distribute your version.

## Performance

### Is dott fast?

Yes! Written in Rust with minimal dependencies, dott starts almost instantly and uses very little resources.

### Does dott use much memory?

No, dott has a tiny memory footprint (typically a few MB).

### Can dott handle many menu items?

Yes, though for usability, 10-15 items is recommended. The TUI can handle more, but navigation becomes less efficient.

## Integration

### Can I integrate dott with my shell?

Yes! Add to `~/.bashrc` or `~/.zshrc`:
```bash
alias d='dott'
```

Or auto-launch on startup:
```bash
if [ -t 1 ] && [ -z "$DOTT_LAUNCHED" ]; then
    export DOTT_LAUNCHED=1
    dott
fi
```

### Does dott work with SSH?

Yes! You can:
- Install dott on remote servers
- Run it via SSH: `ssh -t user@host dott`
- Use it in your remote shell

### Can I use dott in scripts?

Dott is interactive-only. For scripting, use the underlying commands directly.

### Does dott integrate with git?

Not directly, but you can add git commands to your menu:
```toml
[[menu_items]]
name = "Git Status"
command = "lazygit"
args = []
```

## Comparison

### How is dott different from other launchers?

- **Terminal-native**: Runs in terminal, no GUI needed
- **Configuration**: Simple TOML file, no complex setup
- **Simplicity**: Focused on dotfiles and common commands
- **Speed**: Instant startup, keyboard-driven

### Is dott better than aliases?

They serve different purposes:
- **Aliases**: Fast for single commands you use constantly
- **Dott**: Better for commands with arguments, visual organization, less frequently used commands

Use both! Dott for menu-driven access, aliases for instant execution.

### Can dott replace my launcher (rofi, dmenu)?

Different use cases:
- **Rofi/dmenu**: System-wide application launchers
- **Dott**: Terminal-focused, command execution, dotfile management

## Future Plans

### What features are planned?

Common requests include:
- Multiple configuration profiles
- Theme support
- Tabs/categories for menu items
- Search functionality
- Custom keybindings
- Command history

Check GitHub issues for the latest feature discussions.

### Will there be a GUI version?

Unlikely. Dott is intentionally terminal-focused. A GUI would go against its design philosophy.

### Will Windows native support be added?

Possibly in the future, but it's not currently prioritized. WSL works well for Windows users.

## Getting More Help

### Where can I find more information?

- **Documentation**: Read all wiki pages (Home, Installation, Configuration, etc.)
- **GitHub**: [https://github.com/commended/dott](https://github.com/commended/dott)
- **Issues**: Search existing issues for similar problems
- **Source Code**: The code itself is well-documented

### How do I ask for help?

When asking for help, include:
1. Operating system and version
2. Terminal emulator
3. Dott version/commit
4. Configuration file content
5. Steps to reproduce the issue
6. Error messages
7. What you've already tried

### Can I request features?

Yes! Open a feature request issue on GitHub. Describe:
- What you want to achieve
- Why it would be useful
- How you envision it working

## See Also

- [Home](Home.md) - Overview and quick start
- [Installation](Installation.md) - Installation instructions
- [Configuration](Configuration.md) - Configuration guide
- [Troubleshooting](Troubleshooting.md) - Problem solving
- [Development](Development.md) - Contributing guide
