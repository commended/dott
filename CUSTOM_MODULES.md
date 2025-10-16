# Dott Custom Modules

This document lists all the custom modules available in dott. Custom modules are hardcoded UI components that can be enabled and configured through the config file.

## Overview

Custom modules are configured in the `[custom]` section of your config file. Their position in the UI is determined by the `structure.build` order, not by individual position settings.

## Available Custom Modules

### 1. Logo
**Location in structure.build**: `"logo"`
**Configuration**: Top-level config options
**Description**: Displays the application logo at the top of the interface.

**Types**:
- `default`: The built-in ASCII art dott logo
- `custom`: Load a custom ASCII art logo from a file
- `image`: Display an image using Kitty graphics protocol (experimental)

**Configuration Options**:
```toml
logo_type = "default"  # or "custom" or "image"
custom_logo_path = "/path/to/ascii/art.txt"  # For custom type
image_logo_path = "/path/to/image.png"  # For image type
```

### 2. Entries
**Location in structure.build**: `"entries"`
**Configuration**: `[[entries]]` array
**Description**: Terminal commands that can be executed from the menu. Each entry has a name, command, and optional arguments.

**Configuration Format**:
```toml
[[entries]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[entries]]
name = "Quit"
command = ""
args = []
```

**Special Built-in Entries**:
- "Edit Dott Config": Opens the dott config file in nvim
- "View Shell": Opens the detected shell config file in nvim
- "Quit": Exits the application

### 3. Terminal Colors
**Location in structure.build**: `"colors"`
**Configuration**: `[custom.terminal_colors]`
**Description**: Displays a visual representation of the terminal's color palette.

**Configuration Options**:
```toml
[custom.terminal_colors]
shape = "circles"  # or "squares"
```

**Shapes**:
- `circles`: Displays 8 colored circles in a single row (● ● ● ● ● ● ● ●)
- `squares`: Displays 8 colored squares in 2 rows of 4 (■ ■ ■ ■ / ■ ■ ■ ■)

**Colors Displayed**:
Black, Red, Green, Yellow, Blue, Magenta, Cyan, White

### 4. Clock
**Location in structure.build**: `"clock"`
**Configuration**: `[custom.clock]`
**Description**: Displays the current time in HH:MM:SS format. Updates every 100ms.

**Configuration**:
```toml
[custom.clock]
```

**Note**: The clock module no longer has a position setting. Its position is determined by where "clock" appears in your `structure.build` order.

### 5. Help
**Location in structure.build**: `"help"`
**Configuration**: Not configurable
**Description**: Displays keyboard shortcuts and help text.

**Displayed Keybindings**:
- ↑/k: Move selection up
- ↓/j: Move selection down
- Enter: Execute selected entry
- u: Reload configuration
- q/Esc: Quit application

## Structure Configuration

The `structure` section determines how modules are positioned and ordered:

```toml
[structure]
position = "center"  # center, left, or right

[structure.build]
1 = "logo"
2 = "entries"
3 = "colors"
4 = "clock"
5 = "help"
```

### Position Options:
- `center`: Centers the UI elements horizontally (default)
- `left`: Aligns UI elements to the left
- `right`: Aligns UI elements to the right

### Build Order:
The numbers (1, 2, 3, etc.) determine the vertical order in which modules appear. You can:
- Reorder modules by changing the numbers
- Skip modules by omitting them from the build order
- Only include the modules you want to use

## Example Configurations

### Minimal Configuration
```toml
[structure]
position = "center"

[structure.build]
1 = "logo"
2 = "entries"
5 = "help"

logo_type = "default"

[[entries]]
name = "Quit"
command = ""
args = []
```

### Full Configuration
```toml
[structure]
position = "center"

[structure.build]
1 = "logo"
2 = "entries"
3 = "colors"
4 = "clock"
5 = "help"

logo_type = "default"

[[entries]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[entries]]
name = "Quit"
command = ""
args = []

[custom]

[custom.terminal_colors]
shape = "circles"

[custom.clock]
```

### Reordered Configuration
```toml
[structure]
position = "center"

[structure.build]
1 = "clock"        # Clock at top
2 = "logo"
3 = "entries"
4 = "colors"
5 = "help"

logo_type = "default"

# ... rest of config
```

## Migration from Old Config Format

If you have an old config file, here are the changes:

1. **Add structure section**:
   ```toml
   [structure]
   position = "center"
   
   [structure.build]
   1 = "logo"
   2 = "entries"
   3 = "colors"
   4 = "clock"
   5 = "help"
   ```

2. **Rename `menu_items` to `entries`**:
   - Change `[[menu_items]]` to `[[entries]]`

3. **Move custom modules under `[custom]`**:
   - Change `[terminal_colors]` to `[custom.terminal_colors]`
   - Change `[clock]` to `[custom.clock]`

4. **Remove clock position setting**:
   - Remove `position = "bottom"` from clock config
   - Position is now determined by structure.build order
