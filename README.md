# dott

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

## Configuration

The configuration file is automatically created at `~/.config/dott/config.toml` on first run.

### Default Configuration

```toml
logo_type = "default"

[[menu_items]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[menu_items]]
name = "Edit Dott Config"
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

### Using an Image Logo (Experimental)

**Note:** This is an experimental feature that requires a terminal supporting the [Kitty graphics protocol](https://sw.kovidgoyal.net/kitty/graphics-protocol/).

Dott can display actual images as logos using Kitty's image protocol. This works in terminals that support this protocol, such as:
- [Kitty](https://sw.kovidgoyal.net/kitty/)
- [WezTerm](https://wezfurlong.org/wezterm/)
- [Konsole](https://konsole.kde.org/) (limited support)

To use an image as your logo:

1. Prepare your image file (PNG, JPEG, or other common formats)
2. Update your config:

```toml
logo_type = "image"
image_logo_path = "/home/username/.config/dott/logo.png"
```

**Limitations:**
- The image will be displayed briefly before the TUI launches
- Image rendering quality depends on your terminal emulator
- Not all terminals support this feature
- If your terminal doesn't support Kitty graphics protocol, you'll see a warning message

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

### Optional Modules

#### Terminal Colors Module

Display a row of terminal colors to showcase your color scheme. The colors are displayed below the menu items. The module can be configured with two different shapes:

**Circles** (1 row of 8 color entries):
```toml
[terminal_colors]
shape = "circles"
```

**Squares** (2 rows with 4 horizontal entries):
```toml
[terminal_colors]
shape = "squares"
```

#### 24-Hour Clock Module

Display a minimal clock showing the current time in 24-hour format. The clock is displayed directly under the keybindings (help text) at the bottom of the screen and updates every second.

To enable the clock module:
```toml
[clock]
position = "bottom"
```

**Note:** The `position` field is kept for backwards compatibility but is now ignored. The clock is always displayed under the keybindings at the bottom.

See `examples/config-with-modules.toml` and `examples/config-with-modules-alt.toml` for complete examples.


## Requirements

- Rust 1.70+ (for building)
- A terminal with ANSI color support
- yazi
- neovim

