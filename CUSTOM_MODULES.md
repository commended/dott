# Dott Custom Modules

This document lists all the custom modules available in dott. Custom modules are hardcoded UI components that can be enabled and configured through the config file.

## Overview

Custom modules are configured in the `[custom]` section of your config file. Their position in the UI is determined by the `structure.build` order, not by individual position settings.

**Important**: All custom modules must be explicitly declared in the `[custom]` section to be used, even if they have no configurable settings.

## Available Custom Modules

### 1. Logo
**Location in structure.build**: `module = "logo"`, `module = "logo:default"`, `module = "logo:custom"`, or `module = "logo:image"`
**Configuration**: Top-level config options (for backward compatibility) or in structure.build
**Description**: Displays the application logo at the top of the interface.

**Types**:
- `default`: The built-in ASCII art dott logo
- `custom`: Load a custom ASCII art logo from a file
- `image`: Display an image using Kitty graphics protocol (experimental)

**Configuration Options**:
```toml
# Method 1: Top-level configuration (backward compatible)
[structure]
position = "center"

[[structure.build]]
module = "logo"  # Uses the logo_type setting below

# Logo configuration
logo_type = "default"  # or "custom" or "image"
custom_logo_path = "/path/to/ascii/art.txt"  # For custom type
image_logo_path = "/path/to/image.png"  # For image type
```

```toml
# Method 2: Specify logo type directly in structure.build (recommended)
[structure]
position = "center"

[[structure.build]]
module = "logo:default"  # or "logo:custom" or "logo:image"

# Still need to specify paths for custom/image types
custom_logo_path = "/path/to/ascii/art.txt"  # For custom type
image_logo_path = "/path/to/image.png"  # For image type
```

### 2. Entries
**Location in structure.build**: `module = "entries"`
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
**Location in structure.build**: `module = "colors"`
**Configuration**: `[custom.terminal_colors]`
**Description**: Displays a visual representation of the terminal's color palette.

**Configuration Options**:
```toml
[custom]

[custom.terminal_colors]
shape = "circles"  # or "squares"
```

**Shapes**:
- `circles`: Displays 8 colored circles in a single row (● ● ● ● ● ● ● ●)
- `squares`: Displays 8 colored squares in 2 rows of 4 (■ ■ ■ ■ / ■ ■ ■ ■)

**Colors Displayed**:
Black, Red, Green, Yellow, Blue, Magenta, Cyan, White

### 4. Clock
**Location in structure.build**: `module = "clock"`
**Configuration**: `[custom.clock]`
**Description**: Displays the current time in HH:MM:SS format. Updates every 100ms.

**Configuration**:
```toml
[custom]

[custom.clock]
```

**Note**: The clock module must be declared in `[custom.clock]` even though it has no configurable settings. Its position is determined by where "clock" appears in your `structure.build` order.

### 5. Help
**Location in structure.build**: `module = "help"`
**Configuration**: Not configurable
**Description**: Displays keyboard shortcuts and help text.

**Displayed Keybindings**:
- ↑/k: Move selection up
- ↓/j: Move selection down
- Enter: Execute selected entry
- u: Reload configuration
- q/Esc: Quit application

### 6. Break
**Location in structure.build**: `module = "break"`
**Configuration**: `[custom.break]`
**Description**: Inserts empty lines in the UI. By default, each break adds 2 empty lines. This can be configured via `[custom.break]`.

**Configuration Options**:
```toml
[custom]

[custom.break]
lines = 2  # Number of empty lines to insert (default: 2)
```

**Example Usage**:
```toml
[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"      # Adds 2 empty lines (or custom amount)

[[structure.build]]
module = "entries2"

[[structure.build]]
module = "break"      # Adds another 2 empty lines (or custom amount)

[[structure.build]]
module = "help"

[custom]

[custom.break]
lines = 3  # Each break now adds 3 empty lines instead of 2
```

**Note**: You can use multiple `module = "break"` entries in your structure.build. Each will insert the configured number of empty lines.

### 7. Selected
**Location in structure.build**: `module = "selected"`
**Configuration**: `[custom.selected]`
**Description**: Displays the command that will be executed for the currently highlighted/selected menu entry. This module shows what command will run when you press Enter on the selected item.

**Configuration**:
```toml
[custom]

[custom.selected]
```

**Display Format**:
- For regular entries: Shows `Selected: <command> <args>`
- For special entries:
  - "Quit": Shows `Selected: Exit application`
  - "Edit Dott Config": Shows `Selected: Edit dott config in nvim`
  - "View Shell": Shows `Selected: View shell config in nvim`

**Example**:
```toml
[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "selected"  # Shows command for selected entry

[[structure.build]]
module = "help"

[[entries]]
name = "System Monitor"
command = "htop"
args = []

[custom]

[custom.selected]
```

When "System Monitor" is highlighted, the selected module will display: `Selected: htop`

**Note**: The selected module must be declared in `[custom.selected]` even though it has no configurable settings. Its position is determined by where "selected" appears in your `structure.build` order.

### 8. SysInfo
**Location in structure.build**: `module = "sysinfo"`
**Configuration**: `[custom.sysinfo]`
**Description**: Displays system information including OS, window manager/desktop environment, CPU, GPU, memory usage, and uptime. The module only displays if at least one option is enabled.

**Configuration Options**:
```toml
[custom]

[custom.sysinfo]
os = true       # Show OS name with  icon
wm = true       # Show WM/DE with  icon  
cpu = true      # Show CPU with  icon
gpu = true      # Show GPU with 󰍛 icon
memory = true   # Show memory usage (used/total %) with  icon
uptime = true   # Show system uptime with  icon
```

**Display Format**:
-  OS: Shows operating system name without version
-  WM/DE: Shows window manager or desktop environment
-  CPU: Shows CPU model without detailed specs or generation numbers
- 󰍛 GPU: Shows GPU vendor and basic model
-  Memory: Shows used/total in GB and percentage (e.g., "8.5GB/16.0GB 53%")
-  Uptime: Shows system uptime in hours and minutes (e.g., "5h 32m")

**Example**:
```toml
[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "sysinfo"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "help"

[[entries]]
name = "System Monitor"
command = "htop"
args = []

[custom]

[custom.sysinfo]
os = true
wm = true
cpu = true
gpu = true
memory = true
uptime = true

[custom.break]
lines = 2
```

**Note**: The sysinfo module must be declared in `[custom.sysinfo]` with at least one option set to `true` for the module to display anything. If all options are `false` or if no options are enabled, the module will not appear in the UI. Its position is determined by where "sysinfo" appears in your `structure.build` order.

### 9. Quit
**Location in structure.build**: `module = "quit"`
**Configuration**: Not configurable
**Description**: Exits the application immediately when encountered. This is different from the "Quit" entry which is a menu item the user can select. This module type would immediately quit when the structure is rendered, so it's typically not used in practice.

## Structure Configuration

The `structure` section determines how modules are positioned and ordered:

```toml
[structure]
position = "center"  # center, left, or right
font = "JetBrains Mono"  # Optional: specify a custom font (default is terminal font)

[[structure.build]]
module = "logo"

[[structure.build]]
module = "sysinfo"

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
```

### Position Options:
- `center`: Centers the UI elements horizontally (default)
- `left`: Aligns UI elements to the left
- `right`: Aligns UI elements to the right

### Font Option:
- `font`: Optional string to specify a custom font name (default: uses terminal's default font)
- Note: The font setting is stored in configuration but currently uses the terminal's default font for rendering

### Build Order:
The modules appear in the order they are declared in `structure.build`. You can:
- Reorder modules by changing their order in the array
- Skip modules by omitting them from the build order
- Only include the modules you want to use
- Use multiple entry groups (entries, entries2, entries3, entries4, entries5)
- Add breaks (empty lines) between sections

### Multiple Entry Groups:
You can define multiple entry groups to organize your menu items. Each group appears in the order specified in structure.build:

```toml
[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"      # First group

[[structure.build]]
module = "break"        # Empty line

[[structure.build]]
module = "entries2"     # Second group

[[structure.build]]
module = "help"

[[entries]]
name = "Edit Config"
command = "nvim"
args = ["~/.config"]

[[entries2]]
name = "System Tools"
command = "htop"
args = []
```

## Example Configurations

### Minimal Configuration
```toml
[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "help"

# Logo configuration
logo_type = "default"

[[entries]]
name = "Quit"
command = ""
args = []

# Custom modules must be declared
[custom]

[custom.terminal_colors]
shape = "circles"

[custom.clock]

[custom.selected]

[custom.sysinfo]
os = true
wm = true
cpu = true
gpu = true
memory = true
uptime = true

[custom.break]
lines = 2
```

### Multiple Entry Groups Configuration
```toml
[structure]
position = "center"

[[structure.build]]
module = "logo:default"  # Logo type specified in structure.build

[[structure.build]]
module = "clock"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"        # Add spacing between entry groups

[[structure.build]]
module = "entries2"

[[structure.build]]
module = "selected"     # Show command for selected entry

[[structure.build]]
module = "colors"

[[structure.build]]
module = "help"

# First group of entries
[[entries]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[entries]]
name = "Edit Config"
command = "nvim"
args = ["~/.config/nvim/init.lua"]

# Second group of entries
[[entries2]]
name = "System Monitor"
command = "htop"
args = []

[[entries2]]
name = "Quit"
command = ""
args = []

# Custom modules must be declared
[custom]

[custom.terminal_colors]
shape = "circles"

[custom.clock]

[custom.selected]

[custom.break]
lines = 2  # Each break adds 2 empty lines
```

### Full Configuration
```toml
[structure]
position = "center"
font = "JetBrains Mono"  # Optional: specify a custom font

[[structure.build]]
module = "logo"

[[structure.build]]
module = "sysinfo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "clock"

[[structure.build]]
module = "help"

# Logo configuration
logo_type = "default"

[[entries]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[entries]]
name = "Quit"
command = ""
args = []

# Custom modules must be declared
[custom]

[custom.terminal_colors]
shape = "circles"

[custom.clock]

[custom.selected]

[custom.sysinfo]
os = true
wm = true
cpu = true
gpu = true
memory = true
uptime = true

[custom.break]
lines = 2
```

### Reordered Configuration
```toml
[structure]
position = "center"

[[structure.build]]
module = "clock"        # Clock at top

[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "help"

# Logo configuration
logo_type = "default"

# ... rest of config
```

## Migration from Old Config Format

If you have an old config file, here are the changes:

1. **Update structure.build section**:
   Old format:
   ```toml
   [structure.build]
   1 = "logo"
   2 = "entries"
   3 = "colors"
   4 = "clock"
   5 = "help"
   ```
   
   New format:
   ```toml
   [[structure.build]]
   module = "logo"
   
   [[structure.build]]
   module = "entries"
   
   [[structure.build]]
   module = "colors"
   
   [[structure.build]]
   module = "clock"
   
   [[structure.build]]
   module = "help"
   ```

2. **Rename `menu_items` to `entries`**:
   - Change `[[menu_items]]` to `[[entries]]`

3. **Move custom modules under `[custom]` and declare all used modules**:
   - Change `[terminal_colors]` to `[custom.terminal_colors]`
   - Change `[clock]` to `[custom.clock]`
   - Always declare `[custom.clock]` even if it has no settings
   - Always declare `[custom.terminal_colors]` even if using defaults
   - Always declare `[custom.break]` if using breaks
   - Always declare `[custom.selected]` if using selected module
   - Always declare `[custom.sysinfo]` if using sysinfo module (at least one option must be true)

4. **Remove clock position setting**:
   - Remove `position = "bottom"` from clock config
   - Position is now determined by structure.build order

5. **Reorder config sections** (recommended):
   - Put `[structure]` and `[[structure.build]]` at the top
   - Put logo configuration below structure
   - Put entries after logo configuration
   - Put `[custom]` sections at the end
