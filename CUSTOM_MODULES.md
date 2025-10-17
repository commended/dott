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
# NOTE: logo_type and path fields must come BEFORE any [[...]] array sections
logo_type = "default"  # or "custom" or "image"
custom_logo_path = "/path/to/ascii/art.txt"  # For custom type
image_logo_path = "/path/to/image.png"  # For image type

[structure]
position = "center"

[[structure.build]]
module = "logo"  # Uses the logo_type setting above
```

```toml
# Method 2: Specify logo type directly in structure.build (recommended)
# NOTE: Path fields must come BEFORE any [[...]] array sections
custom_logo_path = "/path/to/ascii/art.txt"  # For custom type
image_logo_path = "/path/to/image.png"  # For image type

[structure]
position = "center"

[[structure.build]]
module = "logo:default"  # or "logo:custom" or "logo:image"
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

### 8. Quit
**Location in structure.build**: `module = "quit"`
**Configuration**: Not configurable
**Description**: Exits the application immediately when encountered. This is different from the "Quit" entry which is a menu item the user can select. This module type would immediately quit when the structure is rendered, so it's typically not used in practice.

### 9. System Info
**Location in structure.build**: `module = "system_info"` or `module = "systeminfo"`
**Configuration**: `[custom.system_info]`
**Description**: Displays system information including hostname, operating system, and kernel version.

**Configuration**:
```toml
[custom]

[custom.system_info]
```

**Display Format**:
Shows: `󰇄 hostname | os | kernel`

**Example**: `󰇄 laptop | linux | 6.5.0-28-generic`

**Note**: The system_info module must be declared in `[custom.system_info]` even though it has no configurable settings.

### 10. Quote
**Location in structure.build**: `module = "quote"`
**Configuration**: `[custom.quote]`
**Description**: Displays a random quote from a configured list. Great for inspiration and motivation!

**Configuration Options**:
```toml
[custom]

[custom.quote]
quotes = [
    "The only way to do great work is to love what you do. - Steve Jobs",
    "Code is like humor. When you have to explain it, it's bad. - Cory House",
    "First, solve the problem. Then, write the code. - John Johnson",
]
```

**Default Quotes**:
If no quotes are provided, a default set of programming and inspirational quotes is used.

**Features**:
- Randomly selects a quote each time
- Automatically wraps long quotes to multiple lines (max 80 characters per line)
- Displays in italic yellow text

### 11. Uptime
**Location in structure.build**: `module = "uptime"`
**Configuration**: `[custom.uptime]`
**Description**: Displays system uptime showing how long the system has been running.

**Configuration**:
```toml
[custom]

[custom.uptime]
```

**Display Format**:
- Shows days, hours, and minutes: ` Uptime: 2d 5h 30m`
- If less than 1 day: ` Uptime: 5h 30m`
- If less than 1 hour: ` Uptime: 30m`

**Note**: Currently only supported on Linux systems.

### 12. Disk Usage
**Location in structure.build**: `module = "disk_usage"`, `module = "diskusage"`, or `module = "disk"`
**Configuration**: `[custom.disk_usage]`
**Description**: Displays disk usage information for a specified mount point.

**Configuration Options**:
```toml
[custom]

[custom.disk_usage]
path = "/"  # Mount point to check (default: "/")
```

**Options**:
- `path`: The mount point to check disk usage for (default: "/")

**Display Format**:
Shows: ` Disk: used GB / total GB (percentage%)`

**Example**: ` Disk: 45 / 100 GB (45%)`

### 13. Memory
**Location in structure.build**: `module = "memory"` or `module = "mem"`
**Configuration**: `[custom.memory]`
**Description**: Displays current memory (RAM) usage.

**Configuration**:
```toml
[custom]

[custom.memory]
```

**Display Format**:
Shows: ` Memory: used MB / total MB (percentage%)`

**Example**: ` Memory: 8192 / 16384 MB (50%)`

**Note**: The memory module must be declared in `[custom.memory]` even though it has no configurable settings.

## Structure Configuration

The `structure` section determines how modules are positioned and ordered:

```toml
[structure]
position = "center"  # center, left, or right
font = "JetBrains Mono"  # Optional: specify a custom font (default is terminal font)

[[structure.build]]
module = "logo"

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
# Logo configuration
# NOTE: logo_type must come BEFORE any [[...]] array sections
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
# Logo configuration
# NOTE: logo_type must come BEFORE any [[...]] array sections
logo_type = "default"

[structure]
position = "center"
font = "JetBrains Mono"  # Optional: specify a custom font

[[structure.build]]
module = "logo"

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

[custom.break]
lines = 2
```

### Reordered Configuration
```toml
# Logo configuration
# NOTE: logo_type must come BEFORE any [[...]] array sections
logo_type = "default"

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

# ... rest of config
```

### Creative Modules Configuration
```toml
# Showcase all creative system monitoring modules
# Logo configuration
# NOTE: logo_type must come BEFORE any [[...]] array sections
logo_type = "default"

[structure]
position = "center"

[[structure.build]]
module = "logo"

[[structure.build]]
module = "system_info"

[[structure.build]]
module = "uptime"

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
module = "clock"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "help"

# Entries
[[entries]]
name = "System Monitor"
command = "htop"
args = []

[[entries]]
name = "Quit"
command = ""
args = []

# Custom modules configuration
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
    "The only way to do great work is to love what you do. - Steve Jobs",
    "Code is like humor. When you have to explain it, it's bad. - Cory House",
    "First, solve the problem. Then, write the code. - John Johnson",
]
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

4. **Remove clock position setting**:
   - Remove `position = "bottom"` from clock config
   - Position is now determined by structure.build order

5. **Reorder config sections** (recommended):
   - Put `[structure]` and `[[structure.build]]` at the top
   - Put logo configuration below structure
   - Put entries after logo configuration
   - Put `[custom]` sections at the end
