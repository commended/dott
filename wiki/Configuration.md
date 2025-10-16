# Configuration

Dott is highly configurable through a TOML configuration file. This guide covers all configuration options available.

## Configuration File Location

Dott's configuration file is located at:

```
~/.config/dott/config.toml
```

The configuration file is automatically created with default settings on first run.

## Configuration Structure

The configuration file uses TOML format and consists of several main sections:

1. Logo settings
2. Menu items
3. Terminal colors (optional)
4. Clock (optional)

## Logo Configuration

### Logo Type

Dott supports three types of logos:

```toml
logo_type = "default"  # Options: "default", "custom", "image"
```

#### Default Logo
The built-in dott ASCII art logo:

```toml
logo_type = "default"
```

#### Custom ASCII Logo
Use your own ASCII art from a file:

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/my-logo.txt"
```

Create a text file with your ASCII art and point to it with `custom_logo_path`.

#### Image Logo (Experimental)
Use an image file (requires Kitty terminal or compatible):

```toml
logo_type = "image"
image_logo_path = "~/.config/dott/logo.png"
```

**Note**: Image logos require a terminal with Kitty graphics protocol support (Kitty, WezTerm, some versions of Konsole).

## Menu Items Configuration

Menu items define the actions available in dott. Each menu item has three properties:

```toml
[[menu_items]]
name = "Display Name"      # Name shown in the menu
command = "command"        # Command to execute
args = ["arg1", "arg2"]   # Command arguments (optional)
```

### Example Menu Items

```toml
# Browse dotfiles with yazi
[[menu_items]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

# Edit neovim configuration
[[menu_items]]
name = "Edit Neovim Config"
command = "nvim"
args = ["~/.config/nvim/init.lua"]

# Run a script
[[menu_items]]
name = "Update System"
command = "bash"
args = ["-c", "sudo apt update && sudo apt upgrade"]

# Command without arguments
[[menu_items]]
name = "System Monitor"
command = "htop"
args = []

# Git management
[[menu_items]]
name = "Git Status"
command = "lazygit"
args = []
```

### Built-in Menu Items

Some menu items have special built-in behavior when the `command` field is empty:

```toml
# Built-in: Edit dott configuration
[[menu_items]]
name = "Edit Dott Config"
command = ""
args = []

# Built-in: View and edit shell configuration
# Automatically detects your shell (bash, zsh, fish, etc.)
[[menu_items]]
name = "View Shell"
command = ""
args = []

# Built-in: Quit dott
[[menu_items]]
name = "Quit"
command = ""
args = []
```

### Using Tilde (~) in Paths

Dott automatically expands `~` to your home directory in:
- Menu item arguments
- Logo paths
- Any file paths

```toml
[[menu_items]]
name = "Edit Hosts File"
command = "sudo"
args = ["nvim", "/etc/hosts"]  # Absolute path

[[menu_items]]
name = "Edit Bashrc"
command = "nvim"
args = ["~/.bashrc"]  # Tilde expanded to $HOME
```

## Terminal Colors Configuration

Display a terminal color palette below the menu:

```toml
[terminal_colors]
shape = "circles"  # Options: "circles", "squares"
```

### Circles Shape
Displays 8 colors in a single row using circle symbols (●):

```toml
[terminal_colors]
shape = "circles"
```

### Squares Shape
Displays 8 colors in 2 rows using square symbols (■):

```toml
[terminal_colors]
shape = "squares"
```

### Disable Terminal Colors

To disable the terminal colors display, simply remove or comment out the entire section:

```toml
# [terminal_colors]
# shape = "circles"
```

## Clock Configuration

Display a real-time clock in the interface:

```toml
[clock]
position = "bottom"  # Options: "top", "bottom"
```

### Clock Positions

```toml
# Display clock under the keybindings at the bottom
[clock]
position = "bottom"

# Display clock at the top (not yet implemented)
[clock]
position = "top"
```

### Disable Clock

To disable the clock, remove or comment out the entire section:

```toml
# [clock]
# position = "bottom"
```

## Complete Example Configuration

Here's a complete example with all available options:

```toml
# Logo configuration
logo_type = "default"

# Optional: Custom ASCII logo
# custom_logo_path = "~/.config/dott/custom-logo.txt"

# Optional: Image logo (experimental)
# image_logo_path = "~/.config/dott/logo.png"

# Menu items
[[menu_items]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[menu_items]]
name = "Edit Neovim Config"
command = "nvim"
args = ["~/.config/nvim/init.lua"]

[[menu_items]]
name = "Edit Shell Config"
command = "nvim"
args = ["~/.bashrc"]

[[menu_items]]
name = "System Monitor"
command = "htop"
args = []

[[menu_items]]
name = "Git Status"
command = "lazygit"
args = []

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

# Optional: Terminal colors
[terminal_colors]
shape = "circles"

# Optional: Clock
[clock]
position = "bottom"
```

## Reloading Configuration

To reload your configuration after making changes:

1. Quit dott (`q` or `Esc`)
2. Run dott again: `dott`

Changes take effect immediately on restart.

## Configuration Tips

1. **Order matters**: Menu items appear in the order they're defined
2. **Keep it simple**: Start with a few essential commands
3. **Test commands**: Make sure commands work in your shell before adding them
4. **Use full paths**: For critical scripts, use absolute paths instead of `~`
5. **Comment liberally**: TOML supports `#` for comments

## Default Configuration

If your configuration file is corrupted or you want to start fresh:

1. Delete the configuration file:
   ```bash
   rm ~/.config/dott/config.toml
   ```

2. Run dott again - it will recreate the default configuration:
   ```bash
   dott
   ```

## See Also

- [Customization Guide](Customization.md) - Advanced customization options
- [Examples](Examples.md) - Example configurations for different use cases
- [Troubleshooting](Troubleshooting.md) - Configuration issues and solutions
