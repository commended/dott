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

Display a row of terminal colors to showcase your color scheme. The module can be configured with two different shapes:

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

Display a minimal clock showing the current time in 24-hour format. The clock position can be configured:

**Bottom** (above the help text):
```toml
[clock]
position = "bottom"
```

**Top** (below the logo, before modules start):
```toml
[clock]
position = "top"
```

**Note:** The clock ignores the normal module ordering and always displays in its configured position (top or bottom).

See `examples/config-with-modules.toml` and `examples/config-with-modules-alt.toml` for complete examples.


## Requirements

- Rust 1.70+ (for building)
- A terminal with ANSI color support
- yazi
- neovim

