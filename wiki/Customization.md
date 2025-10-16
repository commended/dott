# Customization

This guide covers advanced customization options to make dott truly your own.

## Table of Contents

- [Custom ASCII Logos](#custom-ascii-logos)
- [Image Logos](#image-logos)
- [Color Schemes](#color-schemes)
- [Menu Organization](#menu-organization)
- [Advanced Configuration](#advanced-configuration)
- [Integration with Other Tools](#integration-with-other-tools)

## Custom ASCII Logos

Create your own ASCII art logo to personalize dott.

### Creating ASCII Art

Tools for creating ASCII art:
- [ASCII Art Generator](http://patorjk.com/software/taag/)
- [Text to ASCII Art Generator (TAAG)](https://textart.io/)
- [FIGlet](http://www.figlet.org/)
- Manual creation in a text editor

### Using a Custom Logo

1. Create your ASCII art and save it to a file:

```bash
mkdir -p ~/.config/dott
cat > ~/.config/dott/my-logo.txt << 'EOF'
   ___           _   _                  
  / __\_   _ ___| |_(_) ___ _ __        
 / / | | | / __| __| |/ _ \ '_ \       
/ /__| |_| \__ \ |_| |  __/ | | |      
\____/\__,_|___/\__|_|\___|_| |_|      
EOF
```

2. Update your configuration:

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/my-logo.txt"
```

### ASCII Art Tips

1. **Keep it readable**: Not too complex or wide
2. **Test in terminal**: Verify it displays correctly
3. **Consider colors**: Dott displays logos in cyan
4. **Width matters**: Logos are centered; keep reasonable width
5. **Use standard ASCII**: Avoid extended Unicode for compatibility

### Example ASCII Logos

#### Minimal Logo
```
  __| _ \__ __| __ __| 
  _| (   |  |      |   
 ___|___/   |     _|   
```

#### Block Style
```
██████╗  ██████╗ ████████╗████████╗
██╔══██╗██╔═══██╗╚══██╔══╝╚══██╔══╝
██║  ██║██║   ██║   ██║      ██║   
██║  ██║██║   ██║   ██║      ██║   
██████╔╝╚██████╔╝   ██║      ██║   
╚═════╝  ╚═════╝    ╚═╝      ╚═╝   
```

#### Simple Text
```
     _       _   _   
  __| | ___ | |_| |_ 
 / _` |/ _ \| __| __|
| (_| | (_) | |_| |_ 
 \__,_|\___/ \__|\__|
```

## Image Logos

Use real images as logos (experimental feature).

### Requirements

- Terminal with Kitty graphics protocol support
- Supported terminals:
  - Kitty ✓
  - WezTerm ✓
  - Konsole (some versions) ✓

### Setting Up Image Logo

1. Prepare your image:
   - Supported formats: PNG, JPEG, GIF
   - Recommended size: 400x200 pixels
   - Optimize for terminal display

2. Save the image:
```bash
cp my-logo.png ~/.config/dott/logo.png
```

3. Configure dott:
```toml
logo_type = "image"
image_logo_path = "~/.config/dott/logo.png"
```

### Image Logo Tips

1. **Size appropriately**: Too large images may be cut off
2. **Contrast matters**: Ensure good visibility on terminal background
3. **Test first**: Verify it displays in your terminal
4. **Fallback**: Keep ASCII logo as backup
5. **Performance**: Large images may slow startup

### Converting Images

Use ImageMagick to optimize images:

```bash
# Resize to appropriate size
convert input.png -resize 400x200 output.png

# Reduce colors for terminal
convert input.png -colors 256 output.png

# Both operations
convert input.png -resize 400x200 -colors 256 ~/.config/dott/logo.png
```

## Color Schemes

Customize the terminal colors display.

### Circle Colors

Single row of colored circles:

```toml
[terminal_colors]
shape = "circles"
```

Display:
```
● ● ● ● ● ● ● ●
```

### Square Colors

Two rows of colored squares:

```toml
[terminal_colors]
shape = "squares"
```

Display:
```
■ ■ ■ ■
■ ■ ■ ■
```

### Disable Colors

Remove the section entirely:

```toml
# No terminal_colors section
```

## Menu Organization

Structure your menu items effectively.

### Grouping by Category

Organize menu items logically:

```toml
# Configuration files
[[menu_items]]
name = "Edit Neovim"
command = "nvim"
args = ["~/.config/nvim/init.lua"]

[[menu_items]]
name = "Edit Tmux"
command = "nvim"
args = ["~/.tmux.conf"]

[[menu_items]]
name = "Edit Shell"
command = "nvim"
args = ["~/.zshrc"]

# Utilities
[[menu_items]]
name = "File Manager"
command = "yazi"
args = []

[[menu_items]]
name = "System Monitor"
command = "htop"
args = []

# Git operations
[[menu_items]]
name = "Git Status"
command = "lazygit"
args = []

[[menu_items]]
name = "Git Log"
command = "git"
args = ["log", "--oneline", "--graph"]

# System
[[menu_items]]
name = "Edit Dott Config"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []
```

### Naming Conventions

Use clear, consistent names:

```toml
# Good: Clear and descriptive
name = "Edit Neovim Config"
name = "View Dotfiles"
name = "Update System"

# Avoid: Too vague or long
name = "Edit"
name = "Edit My Personal Neovim Configuration File"
```

### Menu Size

Keep menus manageable:
- **5-10 items**: Optimal for quick access
- **10-15 items**: Acceptable, may need scrolling
- **15+ items**: Consider splitting into separate configs

## Advanced Configuration

### Multiple Configuration Profiles

Create different configs for different workflows:

```bash
# Default config
~/.config/dott/config.toml

# Work profile
~/.config/dott/work.toml

# Personal profile  
~/.config/dott/personal.toml

# Development profile
~/.config/dott/dev.toml
```

**Note**: Dott currently only loads `config.toml`. To switch profiles:

```bash
# Switch to work profile
cp ~/.config/dott/work.toml ~/.config/dott/config.toml

# Or use symlinks
ln -sf ~/.config/dott/work.toml ~/.config/dott/config.toml
```

### Environment-Specific Configuration

Use environment variables in commands:

```toml
[[menu_items]]
name = "Edit Project"
command = "bash"
args = ["-c", "cd $PROJECT_DIR && nvim ."]
```

Set environment variables in your shell:

```bash
export PROJECT_DIR="~/projects/myapp"
```

### Conditional Commands

Execute different commands based on conditions:

```toml
[[menu_items]]
name = "Update Package Manager"
command = "bash"
args = ["-c", "command -v apt && sudo apt update || sudo pacman -Sy"]
```

### Script Integration

Call custom scripts from menu items:

```bash
# Create script
cat > ~/.local/bin/backup-dotfiles << 'EOF'
#!/bin/bash
rsync -av ~/.config ~/backups/config-$(date +%Y%m%d)
echo "Backup complete!"
EOF
chmod +x ~/.local/bin/backup-dotfiles
```

Add to dott:

```toml
[[menu_items]]
name = "Backup Dotfiles"
command = "backup-dotfiles"
args = []
```

## Integration with Other Tools

### Tmux Integration

Run dott in a dedicated tmux window:

```bash
# Create session with dott
tmux new-session -s main -n dott dott \; \
  new-window -n work \; \
  select-window -t 1
```

Or add to `.tmux.conf`:

```bash
bind d new-window -n dott "dott"
```

### Shell Aliases

Create convenient aliases:

```bash
# Add to ~/.bashrc or ~/.zshrc

# Quick launch
alias d='dott'

# Edit dott config
alias dconf='nvim ~/.config/dott/config.toml'

# Reload dott after config changes
alias dreload='killall dott; dott'
```

### Startup Integration

Auto-launch dott on terminal startup:

```bash
# Add to ~/.bashrc or ~/.zshrc

# Launch on interactive non-login shells
if [ -t 1 ] && [ -z "$DOTT_LAUNCHED" ]; then
    export DOTT_LAUNCHED=1
    dott
fi
```

### SSH Integration

Use dott on remote servers:

```bash
# Install dott on remote
ssh user@server "cargo install --git https://github.com/commended/dott"

# Run dott via SSH
ssh -t user@server "dott"
```

## Theming

While dott doesn't have built-in themes, you can customize appearance:

### Terminal Theme

Dott inherits your terminal's color scheme. Change terminal colors for different looks:

- **Dark themes**: Dracula, Gruvbox Dark, Nord
- **Light themes**: Solarized Light, Gruvbox Light
- **Custom**: Create your own terminal color scheme

### ASCII Art Themes

Create themed ASCII art:

```bash
# Cyberpunk theme
~/.config/dott/cyberpunk-logo.txt

# Minimalist theme
~/.config/dott/minimal-logo.txt

# Retro theme
~/.config/dott/retro-logo.txt
```

Switch themes by updating `custom_logo_path`.

## Example Customizations

### Developer Setup

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/dev-logo.txt"

[[menu_items]]
name = "IDE"
command = "nvim"
args = []

[[menu_items]]
name = "Git UI"
command = "lazygit"
args = []

[[menu_items]]
name = "Database"
command = "psql"
args = []

[[menu_items]]
name = "Docker"
command = "lazydocker"
args = []

[[menu_items]]
name = "API Testing"
command = "postman"
args = []

[terminal_colors]
shape = "squares"

[clock]
position = "bottom"
```

### System Admin Setup

```toml
logo_type = "default"

[[menu_items]]
name = "System Monitor"
command = "htop"
args = []

[[menu_items]]
name = "Disk Usage"
command = "ncdu"
args = ["/"]

[[menu_items]]
name = "Network"
command = "nethogs"
args = []

[[menu_items]]
name = "Logs"
command = "lnav"
args = ["/var/log"]

[[menu_items]]
name = "Services"
command = "systemctl"
args = ["status"]

[terminal_colors]
shape = "circles"
```

### Minimal Setup

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/minimal.txt"

[[menu_items]]
name = "Edit"
command = "nvim"
args = []

[[menu_items]]
name = "Files"
command = "yazi"
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []
```

## See Also

- [Configuration](Configuration.md) - Basic configuration options
- [Examples](Examples.md) - More configuration examples
- [Features](Features.md) - Feature descriptions
