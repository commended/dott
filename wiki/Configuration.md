# Configuration Guide

This guide covers all configuration options available in dott's `config.toml` file.

## Config File Location

The configuration file is located at:

```
~/.config/dott/config.toml
```

## Basic Structure

A dott config file has several main sections:

```toml
logo_type = "default"                    # Top-level logo setting
custom_logo_path = "~/path/to/logo.txt"  # Optional: path to custom logo
image_logo_path = "~/path/to/image.png"  # Optional: path to image logo

[structure]                              # Layout configuration
position = "center"                      # UI positioning
[[structure.build]]                      # Modules to display
module = "logo"

[[entries]]                              # Command entries
name = "Command Name"
command = "command"
args = []

[custom]                                 # Custom module settings
[custom.terminal_colors]
shape = "circles"
```

## Logo Configuration

### Logo Types

dott supports three types of logos:

#### 1. Default Logo
The built-in dott ASCII art logo:

```toml
logo_type = "default"
```

#### 2. Custom ASCII Logo
Use your own ASCII art from a text file:

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/my-logo.txt"
```

Your logo file can contain any ASCII art. Example `my-logo.txt`:

```
  ╔═══╗
  ║ ◆ ║  My Custom Logo
  ╚═══╝
```

#### 3. Image Logo (Kitty Protocol)
Display an image using the Kitty graphics protocol:

```toml
logo_type = "image"
image_logo_path = "~/.config/dott/logo.png"
```

**Note**: Image logos only work in terminals that support the Kitty graphics protocol (Kitty, and some others).

### Logo in Structure Build

You can also specify logo type directly in the structure:

```toml
[[structure.build]]
module = "logo:default"
# or
module = "logo:custom"
# or
module = "logo:image"
```

## Structure Configuration

The `[structure]` section controls the layout and positioning.

### Position

Control where the UI appears in your terminal:

```toml
[structure]
position = "center"  # Options: center, left, right
```

### Font (Optional)

Specify a font family (terminal-dependent):

```toml
[structure]
position = "center"
font = "JetBrains Mono"
```

### Build Array

The `build` array defines which modules appear and in what order:

```toml
[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"

[[structure.build]]
module = "clock"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "help"
```

## Entry Configuration

Entries are the commands you can execute from dott.

### Basic Entry

```toml
[[entries]]
name = "Display Name"
command = "executable"
args = ["arg1", "arg2"]
```

### Entry Fields

- **name**: The display name shown in the menu
- **command**: The executable to run (can be empty for "Quit" functionality)
- **args**: Array of command-line arguments (optional, defaults to empty)

### Example Entries

```toml
# File manager
[[entries]]
name = "Browse Dotfiles"
command = "yazi"
args = ["~/.config"]

# Editor with file
[[entries]]
name = "Edit Neovim Config"
command = "nvim"
args = ["~/.config/nvim/init.lua"]

# Simple command
[[entries]]
name = "System Monitor"
command = "btop"
args = []

# Quit entry (empty command)
[[entries]]
name = "Quit"
command = ""
args = []
```

## Multiple Entry Groups

You can organize commands into separate groups:

```toml
# First group
[[entries]]
name = "Neovim"
command = "nvim"
args = []

[[entries]]
name = "Edit Config"
command = "nvim"
args = ["~/.config/dott/config.toml"]

# Second group
[[entries2]]
name = "File Manager"
command = "yazi"
args = ["~"]

[[entries2]]
name = "Downloads"
command = "yazi"
args = ["~/Downloads"]

# Third group
[[entries3]]
name = "System Monitor"
command = "btop"
args = []

[[entries3]]
name = "Git"
command = "lazygit"
args = []
```

Then add them to your structure:

```toml
[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries2"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries3"
```

Supported groups: `entries`, `entries2`, `entries3`, `entries4`, `entries5`

## Custom Module Settings

Custom modules need to be declared in the `[custom]` section to be used.

### Terminal Colors

Display your terminal's color palette:

```toml
[custom]

[custom.terminal_colors]
shape = "circles"  # Options: circles, squares
```

Then add to structure:
```toml
[[structure.build]]
module = "colors"
```

### Clock

Display the current time:

```toml
[custom]

[custom.clock]
# Clock module has no additional settings
```

Then add to structure:
```toml
[[structure.build]]
module = "clock"
```

### Selected Indicator

Show which entry is currently selected:

```toml
[custom]

[custom.selected]
# Selected module has no additional settings
```

Then add to structure:
```toml
[[structure.build]]
module = "selected"
```

### Break Lines

Add configurable empty space between modules:

```toml
[custom]

[custom.break]
lines = 2  # Number of empty lines (default: 2)
```

Then add to structure:
```toml
[[structure.build]]
module = "break"
```

You can use multiple break modules in your structure.

### System Information

Display system details (OS, kernel, etc.):

```toml
[custom]

[custom.system_info]
# System info has no additional settings
```

Then add to structure:
```toml
[[structure.build]]
module = "system_info"
```

### Uptime

Show system uptime:

```toml
[custom]

[custom.uptime]
# Uptime has no additional settings
```

Then add to structure:
```toml
[[structure.build]]
module = "uptime"
```

### Memory Usage

Display memory statistics:

```toml
[custom]

[custom.memory]
# Memory module has no additional settings
```

Then add to structure:
```toml
[[structure.build]]
module = "memory"
```

### Disk Usage

Show disk space information:

```toml
[custom]

[custom.disk_usage]
path = "/"  # Disk path to monitor
```

Then add to structure:
```toml
[[structure.build]]
module = "disk"
```

### Quotes

Display inspirational or custom quotes:

```toml
[custom]

[custom.quote]
quotes = [
    "Your custom quote here",
    "Another inspiring quote",
    "Code is poetry"
]
```

If not specified, dott uses built-in quotes. Add to structure:
```toml
[[structure.build]]
module = "quote"
```

## Complete Example Configuration

Here's a full-featured config showcasing all options:

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/my-logo.txt"

[structure]
position = "center"
font = "JetBrains Mono"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "break"

[[structure.build]]
module = "system_info"

[[structure.build]]
module = "uptime"

[[structure.build]]
module = "break"

[[structure.build]]
module = "clock"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries2"

[[structure.build]]
module = "break"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "break"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "break"

[[structure.build]]
module = "memory"

[[structure.build]]
module = "disk"

[[structure.build]]
module = "break"

[[structure.build]]
module = "quote"

[[structure.build]]
module = "break"

[[structure.build]]
module = "help"

[[entries]]
name = "Neovim"
command = "nvim"
args = []

[[entries]]
name = "Edit Dott"
command = "nvim"
args = ["~/.config/dott/config.toml"]

[[entries2]]
name = "File Manager"
command = "yazi"
args = ["~"]

[[entries2]]
name = "System Monitor"
command = "btop"
args = []

[[entries2]]
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

[custom.system_info]

[custom.uptime]

[custom.memory]

[custom.disk_usage]
path = "/"

[custom.quote]
quotes = [
    "Make it work, make it right, make it fast.",
    "Simplicity is the ultimate sophistication."
]
```

## Validation and Testing

After editing your config:

1. Save the file
2. Press `u` in dott to hot-reload (if already running)
3. Or restart dott to apply changes

If there are syntax errors, dott will fall back to default configuration.

## Tips

- Start with the default config and modify incrementally
- Use hot-reload (`u` key) to test changes quickly
- Keep a backup of working configurations
- Comment out modules with `#` to temporarily disable them
- Use break modules to control spacing in your layout
- Group related commands together using multiple entry groups
