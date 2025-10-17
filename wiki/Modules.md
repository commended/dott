# Modules Reference

This page documents all available modules in dott and how to use them.

## Module Types

dott has three categories of modules:

1. **Core Modules**: Essential UI components (logo, entries, help)
2. **Custom Modules**: Optional display elements requiring declaration
3. **Creative Modules**: System information and dynamic content

## Core Modules

These modules are available by default without declaration.

### Logo

Displays a logo at the top of your UI.

**Usage in structure:**
```toml
[[structure.build]]
module = "logo"
```

**Configuration:**
```toml
# Use default dott logo
logo_type = "default"

# Use custom ASCII art
logo_type = "custom"
custom_logo_path = "~/.config/dott/my-logo.txt"

# Use image (Kitty protocol)
logo_type = "image"
image_logo_path = "~/.config/dott/logo.png"
```

**Alternative syntax:**
```toml
[[structure.build]]
module = "logo:default"
# or
module = "logo:custom"
# or
module = "logo:image"
```

### Entries

Displays a group of command shortcuts.

**Usage:**
```toml
[[structure.build]]
module = "entries"   # Main group

[[structure.build]]
module = "entries2"  # Second group

[[structure.build]]
module = "entries3"  # Third group
```

**Configuration:**
```toml
[[entries]]
name = "Command Name"
command = "executable"
args = ["arg1", "arg2"]

[[entries2]]
name = "Another Command"
command = "another"
args = []
```

**Supported groups:** entries, entries2, entries3, entries4, entries5

### Help

Displays keyboard navigation help text.

**Usage:**
```toml
[[structure.build]]
module = "help"
```

**No configuration needed** - Shows default keybindings automatically.

### Quit

Special module for quit functionality.

**Usage:**
```toml
[[structure.build]]
module = "quit"
```

Provides a quit option. Usually handled within entries with an empty command:

```toml
[[entries]]
name = "Quit"
command = ""
args = []
```

## Custom Modules

These modules must be declared in `[custom]` to be used.

### Terminal Colors

Displays your terminal's color palette.

**Declaration:**
```toml
[custom]

[custom.terminal_colors]
shape = "circles"  # or "squares"
```

**Usage:**
```toml
[[structure.build]]
module = "colors"
```

**Options:**
- `circles`: Displays colors as circular shapes (●)
- `squares`: Displays colors as square blocks (■)

**Example output (circles):**
```
● ● ● ● ● ● ● ●
Black Red Green Yellow Blue Magenta Cyan White
```

### Clock

Displays the current time.

**Declaration:**
```toml
[custom]

[custom.clock]
```

**Usage:**
```toml
[[structure.build]]
module = "clock"
```

**Display format:**
Shows current time in HH:MM:SS format, updated in real-time.

### Selected Indicator

Shows which entry is currently selected.

**Declaration:**
```toml
[custom]

[custom.selected]
```

**Usage:**
```toml
[[structure.build]]
module = "selected"
```

**Display:**
Shows text like "Selected: Command Name" with highlighting.

### Break

Adds configurable empty lines between modules.

**Declaration:**
```toml
[custom]

[custom.break]
lines = 2  # Number of empty lines (default: 2)
```

**Usage:**
```toml
[[structure.build]]
module = "break"
```

**Use cases:**
- Separate logical sections
- Add breathing room to your layout
- Create visual hierarchy

**Multiple breaks:**
You can use break multiple times in your structure:

```toml
[[structure.build]]
module = "logo"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"

[[structure.build]]
module = "clock"
```

## Creative Modules

Dynamic modules that display system information.

### System Info

Displays operating system and kernel information.

**Declaration:**
```toml
[custom]

[custom.system_info]
```

**Usage:**
```toml
[[structure.build]]
module = "system_info"
```

**Example output:**
```
System: Linux 6.1.0
OS: Ubuntu 22.04
```

### Uptime

Shows how long the system has been running.

**Declaration:**
```toml
[custom]

[custom.uptime]
```

**Usage:**
```toml
[[structure.build]]
module = "uptime"
```

**Example output:**
```
Uptime: 5 days, 3 hours, 42 minutes
```

### Memory

Displays memory usage statistics.

**Declaration:**
```toml
[custom]

[custom.memory]
```

**Usage:**
```toml
[[structure.build]]
module = "memory"
```

**Example output:**
```
Memory: 8.2 GB / 16.0 GB (51%)
```

### Disk Usage

Shows disk space information for a specific path.

**Declaration:**
```toml
[custom]

[custom.disk_usage]
path = "/"  # Mount point to monitor
```

**Usage:**
```toml
[[structure.build]]
module = "disk"
```

**Example output:**
```
Disk (/): 120 GB / 256 GB (47%)
```

**Tips:**
- Use "/" for root filesystem
- Use "/home" for home partition
- Use specific mount points as needed

### Quote

Displays inspirational or custom quotes.

**Declaration:**
```toml
[custom]

[custom.quote]
quotes = [
    "First, solve the problem. Then, write the code.",
    "Code is like humor. When you have to explain it, it's bad.",
    "Simplicity is the ultimate sophistication."
]
```

**Usage:**
```toml
[[structure.build]]
module = "quote"
```

**Default quotes:**
If you don't specify quotes, dott includes these built-in quotes:
- "The only way to do great work is to love what you do. - Steve Jobs"
- "Innovation distinguishes between a leader and a follower. - Steve Jobs"
- "Stay hungry, stay foolish. - Steve Jobs"
- "Code is like humor. When you have to explain it, it's bad. - Cory House"
- "First, solve the problem. Then, write the code. - John Johnson"

**Random selection:**
A random quote is selected each time dott starts.

## Module Layout Examples

### Minimal Layout

```toml
[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "help"
```

### Balanced Layout

```toml
[[structure.build]]
module = "logo"

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
module = "colors"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "help"
```

### Information-Rich Layout

```toml
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
module = "entries"

[[structure.build]]
module = "clock"

[[structure.build]]
module = "help"
```

### Multi-Group Layout

```toml
[[structure.build]]
module = "logo"

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

[[structure.build]]
module = "selected"

[[structure.build]]
module = "help"
```

## Module Order Best Practices

1. **Logo first**: Place logo at the top for visual hierarchy
2. **System info early**: If using system info, place near the top
3. **Entries central**: Keep command entries in the middle section
4. **Help last**: Place help text at the bottom for reference
5. **Use breaks**: Add breaks between logical sections
6. **Selected near entries**: Place selected module near entry groups
7. **Clock flexible**: Can be placed anywhere, often near top or bottom

## Module Declaration Requirements

Remember these rules:

**Always declare in [custom]:**
- terminal_colors
- clock
- selected
- break
- system_info
- uptime
- memory
- disk_usage
- quote

**Never need declaration:**
- logo
- entries (all groups)
- help
- quit

## Troubleshooting Modules

### Module not appearing
- Ensure it's added to `[[structure.build]]`
- Check that custom modules are declared in `[custom]`
- Verify TOML syntax is correct

### Module shows error
- Check custom module configuration
- Ensure required fields are present (e.g., `path` for disk_usage)
- Reload config with `u` key

### Colors not showing
- Verify terminal supports ANSI colors
- Try different shape option (circles vs squares)
- Check terminal color scheme

### System modules show no data
- System modules require appropriate system access
- Some modules may not work on all platforms
- Check terminal output for error messages
