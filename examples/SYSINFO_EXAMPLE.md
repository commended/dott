# Complete Configuration Example with SysInfo Module

This example shows a comprehensive configuration using the new sysinfo module
along with other custom modules.

```toml
[structure]
position = "center"

# Structure build order determines what appears and in what order
[[structure.build]]
module = "logo"

[[structure.build]]
module = "break"

# The new sysinfo module
[[structure.build]]
module = "sysinfo"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "break"

[[structure.build]]
module = "clock"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "help"

# Logo configuration
logo_type = "default"

# Menu entries
[[entries]]
name = "View Dotfiles"
command = "yazi"
args = ["~/.config"]

[[entries]]
name = "System Monitor"
command = "htop"
args = []

[[entries]]
name = "Edit Dott Config"
command = ""
args = []

[[entries]]
name = "Quit"
command = ""
args = []

# Custom modules configuration
[custom]

# Terminal colors module
[custom.terminal_colors]
shape = "circles"

# Clock module (HH:MM:SS format)
[custom.clock]

# Selected command display
[custom.selected]

# SysInfo module - NEW!
# At least one option must be true for the module to display
[custom.sysinfo]
os = true       # Show OS name with  icon
wm = true       # Show WM/DE with  icon  
cpu = true      # Show CPU with  icon
gpu = true      # Show GPU with 󰍛 icon
memory = true   # Show memory usage with  icon
uptime = true   # Show system uptime with  icon

# Break configuration (empty lines)
[custom.break]
lines = 2
```

## Expected Display

When running dott with this configuration, the sysinfo module will display something like:

```
 Linux
 GNOME
 Intel Core i7
󰍛 NVIDIA GeForce
 8.5GB/16.0GB 53%
 5h 32m
```

Each line shows:
- An icon in cyan color (using nerd fonts)
- The system information in white

## Customization Options

You can enable/disable individual fields:

```toml
[custom.sysinfo]
os = true       # Show only OS
wm = false      # Hide WM/DE
cpu = false     # Hide CPU
gpu = false     # Hide GPU
memory = true   # Show memory
uptime = true   # Show uptime
```

If all options are `false`, the module won't display at all.

## Requirements

- Nerd fonts installed and configured in your terminal for icons to display correctly
- The module will gracefully fall back to "Unknown" for information it cannot detect

## Tips

1. The module shows simplified information (no versions, no detailed specs)
2. Memory is always shown as used/total with percentage
3. Uptime is formatted as hours and minutes for readability
4. GPU detection works best on Linux systems with lspci available
5. WM/DE detection uses standard environment variables (XDG_CURRENT_DESKTOP, etc.)
