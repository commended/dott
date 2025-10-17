# Customization Guide

This guide covers advanced customization techniques and creative ways to use dott.

## Creating Custom Logos

### ASCII Art Logos

You can create your own ASCII art logo and use it in dott.

**Step 1: Create your logo file**

```bash
nvim ~/.config/dott/my-logo.txt
```

**Step 2: Add your ASCII art**

Example logos:

```
    ╔══════════════════════════╗
    ║   Welcome to My Shell    ║
    ╚══════════════════════════╝
```

Or something more creative:

```
    ⚡ POWER USER ⚡
    ══════════════════
```

**Step 3: Configure dott**

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/my-logo.txt"
```

### ASCII Art Resources

Find ASCII art online:
- [ASCII Art Generator](https://www.asciiart.eu/)
- [Text to ASCII Art Generator (TAAG)](https://patorjk.com/software/taag/)
- [ASCII Generator](https://ascii-generator.site/)

### Tips for ASCII Logos

- Keep width under 80 characters for compatibility
- Use box-drawing characters (─ │ ┌ ┐ └ ┘) for clean lines
- Test in your terminal to ensure proper rendering
- Consider color support (ASCII art is monochrome in dott)

## Image Logos (Kitty Protocol)

If you use Kitty terminal or compatible terminals:

**Step 1: Prepare your image**

Convert images to appropriate size:

```bash
convert input.png -resize 40x20 ~/.config/dott/logo.png
```

**Step 2: Configure dott**

```toml
logo_type = "image"
image_logo_path = "~/.config/dott/logo.png"
```

**Supported terminals:**
- Kitty
- Some Kitty-compatible terminals

**Image requirements:**
- PNG format recommended
- Keep size reasonable (40x20 to 80x40 pixels)
- Will be displayed using terminal graphics protocol

## Advanced Layout Patterns

### Dashboard Style

Create an information-rich dashboard:

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
module = "clock"

[[structure.build]]
module = "break"

[[structure.build]]
module = "quote"

[[structure.build]]
module = "break"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "help"
```

### Minimalist Style

Keep it simple and focused:

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
module = "help"

[custom]

[custom.break]
lines = 3  # More breathing room
```

### Power User Style

Organize commands by category:

```toml
[[structure.build]]
module = "logo"

[[structure.build]]
module = "clock"

[[structure.build]]
module = "break"

# Development tools
[[structure.build]]
module = "entries"

[[structure.build]]
module = "break"

# System utilities
[[structure.build]]
module = "entries2"

[[structure.build]]
module = "break"

# File management
[[structure.build]]
module = "entries3"

[[structure.build]]
module = "break"

[[structure.build]]
module = "selected"

[[structure.build]]
module = "colors"

[[structure.build]]
module = "help"
```

## Command Organization Strategies

### By Task Type

Organize entries by what you're doing:

```toml
# Development
[[entries]]
name = "── Development ──"
command = ""
args = []

[[entries]]
name = "Neovim"
command = "nvim"
args = []

[[entries]]
name = "Git UI"
command = "lazygit"
args = []

# System Management
[[entries2]]
name = "── System ──"
command = ""
args = []

[[entries2]]
name = "System Monitor"
command = "btop"
args = []

[[entries2]]
name = "Disk Usage"
command = "ncdu"
args = ["/"]

# File Management
[[entries3]]
name = "── Files ──"
command = ""
args = []

[[entries3]]
name = "Home"
command = "yazi"
args = ["~"]

[[entries3]]
name = "Config"
command = "yazi"
args = ["~/.config"]
```

### By Frequency

Most used commands first:

```toml
[[entries]]
name = "Neovim (Most Used)"
command = "nvim"
args = []

[[entries]]
name = "File Manager"
command = "yazi"
args = ["~"]

[[entries]]
name = "Git"
command = "lazygit"
args = []

# Less frequent
[[entries2]]
name = "System Monitor"
command = "btop"
args = []

[[entries2]]
name = "Configure Dott"
command = "nvim"
args = ["~/.config/dott/config.toml"]
```

### By Project

Different configurations per project:

```bash
# Work config
~/.config/dott/work-config.toml

# Personal config
~/.config/dott/personal-config.toml

# Gaming setup
~/.config/dott/gaming-config.toml
```

Create a switcher script:

```bash
#!/bin/bash
# ~/.local/bin/dott-work
cp ~/.config/dott/work-config.toml ~/.config/dott/config.toml
dott
```

## Custom Quotes

Create themed quote collections:

### Motivational

```toml
[custom.quote]
quotes = [
    "The best time to plant a tree was 20 years ago. The second best time is now.",
    "Don't wait for opportunity. Create it.",
    "Success is not final, failure is not fatal.",
    "The only way to do great work is to love what you do."
]
```

### Programming Wisdom

```toml
[custom.quote]
quotes = [
    "First, solve the problem. Then, write the code. - John Johnson",
    "Code is like humor. When you have to explain it, it's bad. - Cory House",
    "Any fool can write code that a computer can understand. Good programmers write code that humans can understand. - Martin Fowler",
    "Premature optimization is the root of all evil. - Donald Knuth",
    "Make it work, make it right, make it fast. - Kent Beck"
]
```

### Personal Reminders

```toml
[custom.quote]
quotes = [
    "Remember to take breaks!",
    "Have you committed your work?",
    "Time for a coffee?",
    "Stand up and stretch!",
    "Don't forget the meeting at 2pm"
]
```

### Empty (No Quotes)

```toml
[custom.quote]
quotes = []
```

## Color Schemes

Choose your terminal colors to match dott's display:

### Popular Terminal Themes

**Dracula Theme:**
- Looks great with `shape = "circles"`
- Purple and cyan accents complement the default logo

**Nord Theme:**
- Clean and minimal
- Works well with `shape = "squares"`
- Matches minimalist layouts

**Gruvbox:**
- Warm, retro aesthetic
- Both shapes work well
- Great for a cozy terminal feel

**Solarized:**
- Classic developer theme
- Circles show the palette beautifully
- Professional appearance

## Dynamic Configurations

### Time-Based Switching

Create different configs for different times:

```bash
#!/bin/bash
# ~/.local/bin/dott-auto

hour=$(date +%H)

if [ $hour -ge 6 -a $hour -lt 12 ]; then
    cp ~/.config/dott/morning.toml ~/.config/dott/config.toml
elif [ $hour -ge 12 -a $hour -lt 18 ]; then
    cp ~/.config/dott/afternoon.toml ~/.config/dott/config.toml
else
    cp ~/.config/dott/night.toml ~/.config/dott/config.toml
fi

dott
```

### Context-Aware Configs

Switch based on current directory:

```bash
#!/bin/bash
# ~/.local/bin/dott-smart

if [[ $(pwd) == *"work"* ]]; then
    cp ~/.config/dott/work.toml ~/.config/dott/config.toml
elif [[ $(pwd) == *"personal"* ]]; then
    cp ~/.config/dott/personal.toml ~/.config/dott/config.toml
else
    cp ~/.config/dott/default.toml ~/.config/dott/config.toml
fi

dott
```

## Advanced Command Tricks

### Command with Multiple Arguments

```toml
[[entries]]
name = "Search in Code"
command = "rg"
args = ["pattern", "--type", "rust", "./src"]
```

### Script Execution

```toml
[[entries]]
name = "Run Build Script"
command = "bash"
args = ["~/.local/bin/build.sh"]
```

### Opening Multiple Files

```toml
[[entries]]
name = "Edit Multiple Configs"
command = "nvim"
args = ["-O", "~/.bashrc", "~/.vimrc", "~/.config/dott/config.toml"]
```

### SSH Connections

```toml
[[entries]]
name = "Connect to Server"
command = "ssh"
args = ["user@example.com"]
```

### Docker Commands

```toml
[[entries]]
name = "Docker Stats"
command = "docker"
args = ["stats"]

[[entries]]
name = "Docker PS"
command = "docker"
args = ["ps", "-a"]
```

## Integration Ideas

### With tmux

Launch dott in a specific tmux pane:

```bash
tmux split-window -h 'dott'
```

### With i3/sway

Bind to a key in your window manager:

```
bindsym $mod+d exec kitty -e dott
```

### As Shell Greeting

Add to shell RC to show on terminal startup:

```bash
# Show dott on new terminal
if [ -t 0 ]; then
    dott
fi
```

### Custom Wrapper

Create a wrapper with pre/post actions:

```bash
#!/bin/bash
# ~/.local/bin/my-dott

# Pre-actions
echo "Loading dott..."
cd ~

# Run dott
dott

# Post-actions
echo "Thanks for using dott!"
```

## Performance Tips

### Fast Loading

Minimize modules for faster startup:

```toml
# Minimal, fast config
[[structure.build]]
module = "logo"

[[structure.build]]
module = "entries"

[[structure.build]]
module = "help"
```

### Optimize for Large Entry Lists

Split into groups to maintain readability:

```toml
# Instead of 20 entries in one group
# Use multiple groups of 5-7 entries each
```

## Backup Your Configurations

Always keep backups of your working configs:

```bash
# Create backup directory
mkdir -p ~/.config/dott/backups

# Backup current config
cp ~/.config/dott/config.toml ~/.config/dott/backups/config-$(date +%Y%m%d).toml

# Restore from backup
cp ~/.config/dott/backups/config-20240101.toml ~/.config/dott/config.toml
```

## Sharing Configurations

Share your config with the community:

1. Remove any personal information (paths, usernames)
2. Add comments explaining unique features
3. Share as a gist or in the repository discussions
4. Include a screenshot of your setup

## Next Level Customization

Explore these advanced ideas:

- **Multi-monitor setups**: Different configs per terminal
- **Profile system**: Quick-switch between configs
- **Integration scripts**: Launch multiple tools from one entry
- **Custom logos per project**: Automatic logo switching
- **Themed setups**: Coordinate all terminal tools' colors

Remember: The best configuration is one that fits YOUR workflow!
