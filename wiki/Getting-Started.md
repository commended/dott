# Getting Started

This guide will help you install and run dott for the first time.

## Installation

### Option 1: Install from Git (Recommended)

The easiest way to install dott is directly from the GitHub repository:

```bash
cargo install --git https://github.com/commended/dott
```

This will compile and install dott to `~/.cargo/bin/dott`.

### Option 2: Build from Source

If you want to build from source or contribute to development:

1. Clone the repository:
```bash
git clone https://github.com/commended/dott
cd dott
```

2. Build the project:
```bash
cargo build --release
```

3. Copy the binary to your PATH:
```bash
sudo cp target/release/dott /usr/local/bin/
```

Or add it to your local bin:
```bash
mkdir -p ~/.local/bin
cp target/release/dott ~/.local/bin/
```

Make sure `~/.local/bin` is in your PATH.

## First Run

After installation, simply run:

```bash
~/.cargo/bin/dott
```

Or if you added it to your PATH:

```bash
dott
```

On first run, dott will create a default configuration file at `~/.config/dott/config.toml`.

## Basic Navigation

Once dott is running, you can navigate using:

### Keyboard Controls

- **Arrow Keys** (`↑`/`↓`) - Move selection up and down
- **Vim Keys** (`j`/`k`) - Alternative navigation (down/up)
- **Enter** - Execute the selected command
- **`u`** - Reload configuration without restarting
- **`q`** or select "Quit" - Exit dott

### Understanding the Interface

The default interface shows:

1. **Logo** - The dott ASCII art logo at the top
2. **Entries** - Your list of commands/shortcuts
3. **Colors** - Terminal color palette display
4. **Clock** - Current time
5. **Selected** - Shows which item is currently selected
6. **Help** - Quick reference for keybindings

## Your First Configuration

The default config file is located at `~/.config/dott/config.toml`. Let's customize it!

### Edit Your Config

Select "Configure Dott" from the menu (if available), or manually open:

```bash
nvim ~/.config/dott/config.toml
# or
nano ~/.config/dott/config.toml
# or your preferred editor
```

### Adding Your First Entry

Add a new command to your entries:

```toml
[[entries]]
name = "My Terminal"
command = "bash"
args = []
```

### Hot Reload

After saving your changes, press `u` while dott is running to reload the configuration instantly!

## Common First Commands

Here are some useful entries to add:

### File Manager
```toml
[[entries]]
name = "Browse Files"
command = "yazi"
args = ["~"]
```

### System Monitor
```toml
[[entries]]
name = "System Monitor"
command = "btop"
args = []
```

### Edit Config
```toml
[[entries]]
name = "Edit Dott Config"
command = "nvim"
args = ["~/.config/dott/config.toml"]
```

### Shell Config
```toml
[[entries]]
name = "Edit Shell Config"
command = "nvim"
args = ["~/.bashrc"]  # or ~/.zshrc
```

### Git Interface
```toml
[[entries]]
name = "Git Status"
command = "lazygit"
args = []
```

## Updating dott

To update to the latest version:

```bash
cargo install --git https://github.com/commended/dott --force
```

The `--force` flag will overwrite the existing installation.

## Setting Up Autostart

### Add to Shell RC

To launch dott when you open a terminal, add to your shell config:

**For bash** (`~/.bashrc`):
```bash
# Launch dott
~/.cargo/bin/dott
```

**For zsh** (`~/.zshrc`):
```bash
# Launch dott
~/.cargo/bin/dott
```

**For fish** (`~/.config/fish/config.fish`):
```fish
# Launch dott
~/.cargo/bin/dott
```

### Create an Alias

For quick access, add an alias to your shell config:

```bash
alias dott='~/.cargo/bin/dott'
```

## Troubleshooting First Run

### "Command not found"

Make sure `~/.cargo/bin` is in your PATH. Add to your shell config:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then restart your terminal or source your config:

```bash
source ~/.bashrc  # or ~/.zshrc
```

### "Config file not found"

Dott will create the config automatically. If it doesn't, manually create:

```bash
mkdir -p ~/.config/dott
touch ~/.config/dott/config.toml
```

Then add the default configuration from the Configuration page.

### Terminal Colors Not Showing

Ensure your terminal supports ANSI colors. Most modern terminals do. Try:

- iTerm2 (macOS)
- Alacritty
- Kitty
- GNOME Terminal
- Windows Terminal

## Next Steps

Now that you have dott running:

1. Explore the Configuration page to learn about all available options
2. Check out the Modules page to discover what you can add to your layout
3. Visit the Customization page for advanced tips and tricks

Happy dotting!
