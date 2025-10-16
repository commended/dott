# Examples

This page provides complete configuration examples for different use cases and workflows.

## Basic Configuration

A simple, minimal configuration for getting started:

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

## Developer Workflow

Configuration optimized for software development:

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/dev-logo.txt"

# Editor
[[menu_items]]
name = "Open Neovim"
command = "nvim"
args = []

[[menu_items]]
name = "VS Code"
command = "code"
args = ["."]

# Version Control
[[menu_items]]
name = "Git Status"
command = "lazygit"
args = []

[[menu_items]]
name = "Git Log"
command = "git"
args = ["log", "--oneline", "--graph", "--all", "--decorate"]

# Project Management
[[menu_items]]
name = "Run Tests"
command = "bash"
args = ["-c", "npm test || cargo test || pytest"]

[[menu_items]]
name = "Build Project"
command = "bash"
args = ["-c", "npm run build || cargo build || make"]

# Containers & Services
[[menu_items]]
name = "Docker Dashboard"
command = "lazydocker"
args = []

[[menu_items]]
name = "View Logs"
command = "bash"
args = ["-c", "tail -f /var/log/app/*.log"]

# Database
[[menu_items]]
name = "PostgreSQL Client"
command = "psql"
args = ["-U", "postgres"]

# Configuration
[[menu_items]]
name = "Edit Dott Config"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []

[terminal_colors]
shape = "squares"

[clock]
position = "bottom"
```

## System Administrator

Configuration for system administration tasks:

```toml
logo_type = "default"

# Monitoring
[[menu_items]]
name = "System Monitor"
command = "htop"
args = []

[[menu_items]]
name = "Process Tree"
command = "btop"
args = []

[[menu_items]]
name = "Disk Usage"
command = "ncdu"
args = ["/"]

[[menu_items]]
name = "Network Monitor"
command = "nethogs"
args = []

# System Management
[[menu_items]]
name = "System Services"
command = "bash"
args = ["-c", "systemctl list-units --type=service | less"]

[[menu_items]]
name = "System Logs"
command = "journalctl"
args = ["-f"]

[[menu_items]]
name = "Failed Services"
command = "systemctl"
args = ["--failed"]

# Package Management
[[menu_items]]
name = "Update System"
command = "bash"
args = ["-c", "sudo apt update && sudo apt upgrade"]

[[menu_items]]
name = "Clean Packages"
command = "bash"
args = ["-c", "sudo apt autoremove && sudo apt clean"]

# Security
[[menu_items]]
name = "Firewall Status"
command = "sudo"
args = ["ufw", "status", "verbose"]

[[menu_items]]
name = "Check Open Ports"
command = "sudo"
args = ["netstat", "-tulpn"]

# Configuration
[[menu_items]]
name = "Edit Dott Config"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []

[terminal_colors]
shape = "circles"
```

## Content Writer / Blogger

Configuration for writing and content creation:

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/writer-logo.txt"

# Writing
[[menu_items]]
name = "Open Blog Posts"
command = "nvim"
args = ["~/blog/content/"]

[[menu_items]]
name = "Open Notes"
command = "nvim"
args = ["~/notes/"]

[[menu_items]]
name = "Obsidian Vault"
command = "obsidian"
args = []

# File Management
[[menu_items]]
name = "Browse Writing"
command = "yazi"
args = ["~/writing"]

# Publishing
[[menu_items]]
name = "Build Hugo Site"
command = "bash"
args = ["-c", "cd ~/blog && hugo"]

[[menu_items]]
name = "Deploy Site"
command = "bash"
args = ["-c", "cd ~/blog && ./deploy.sh"]

# Git
[[menu_items]]
name = "Git Status"
command = "lazygit"
args = []

# Utilities
[[menu_items]]
name = "Spell Check"
command = "bash"
args = ["-c", "aspell check ~/writing/draft.md"]

# Configuration
[[menu_items]]
name = "Edit Dott Config"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []

[clock]
position = "bottom"
```

## Minimalist Setup

Ultra-minimal configuration with just essentials:

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/minimal-logo.txt"

[[menu_items]]
name = "Editor"
command = "nvim"
args = []

[[menu_items]]
name = "Files"
command = "yazi"
args = []

[[menu_items]]
name = "Shell"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []
```

## Multi-Language Developer

Configuration for polyglot developers:

```toml
logo_type = "default"

# Editors
[[menu_items]]
name = "Neovim"
command = "nvim"
args = []

# Python
[[menu_items]]
name = "Python REPL"
command = "python3"
args = []

[[menu_items]]
name = "IPython"
command = "ipython"
args = []

[[menu_items]]
name = "Run Python Tests"
command = "pytest"
args = []

# JavaScript/Node
[[menu_items]]
name = "Node REPL"
command = "node"
args = []

[[menu_items]]
name = "NPM Dev Server"
command = "npm"
args = ["run", "dev"]

[[menu_items]]
name = "Run Jest Tests"
command = "npm"
args = ["test"]

# Rust
[[menu_items]]
name = "Cargo Build"
command = "cargo"
args = ["build"]

[[menu_items]]
name = "Cargo Test"
command = "cargo"
args = ["test"]

[[menu_items]]
name = "Cargo Run"
command = "cargo"
args = ["run"]

# Go
[[menu_items]]
name = "Go Run"
command = "go"
args = ["run", "."]

[[menu_items]]
name = "Go Test"
command = "go"
args = ["test", "./..."]

# Git
[[menu_items]]
name = "Git UI"
command = "lazygit"
args = []

# Configuration
[[menu_items]]
name = "Edit Dott Config"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []

[terminal_colors]
shape = "squares"

[clock]
position = "bottom"
```

## Dotfiles Manager

Specialized configuration for managing dotfiles:

```toml
logo_type = "default"

# Browse Dotfiles
[[menu_items]]
name = "View All Dotfiles"
command = "yazi"
args = ["~/.config"]

[[menu_items]]
name = "View Home Dotfiles"
command = "yazi"
args = ["~"]

# Edit Configurations
[[menu_items]]
name = "Edit Neovim"
command = "nvim"
args = ["~/.config/nvim/init.lua"]

[[menu_items]]
name = "Edit Tmux"
command = "nvim"
args = ["~/.tmux.conf"]

[[menu_items]]
name = "Edit Alacritty"
command = "nvim"
args = ["~/.config/alacritty/alacritty.yml"]

[[menu_items]]
name = "Edit Zsh"
command = "nvim"
args = ["~/.zshrc"]

[[menu_items]]
name = "Edit Git Config"
command = "nvim"
args = ["~/.gitconfig"]

# Dotfiles Repo Management
[[menu_items]]
name = "Dotfiles Git Status"
command = "bash"
args = ["-c", "cd ~/dotfiles && lazygit"]

[[menu_items]]
name = "Commit Dotfiles"
command = "bash"
args = ["-c", "cd ~/dotfiles && git add . && git commit"]

[[menu_items]]
name = "Push Dotfiles"
command = "bash"
args = ["-c", "cd ~/dotfiles && git push"]

# Backup & Restore
[[menu_items]]
name = "Backup Dotfiles"
command = "bash"
args = ["-c", "rsync -av ~/.config ~/backups/config-$(date +%Y%m%d)"]

[[menu_items]]
name = "List Backups"
command = "bash"
args = ["-c", "ls -lh ~/backups/"]

# Dott
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

[terminal_colors]
shape = "circles"
```

## Remote Server Management

Configuration for managing remote servers:

```toml
logo_type = "default"

# Server Monitoring
[[menu_items]]
name = "System Resources"
command = "htop"
args = []

[[menu_items]]
name = "Disk Space"
command = "df"
args = ["-h"]

[[menu_items]]
name = "Memory Usage"
command = "free"
args = ["-h"]

# Services
[[menu_items]]
name = "Nginx Status"
command = "systemctl"
args = ["status", "nginx"]

[[menu_items]]
name = "Nginx Restart"
command = "sudo"
args = ["systemctl", "restart", "nginx"]

[[menu_items]]
name = "PostgreSQL Status"
command = "systemctl"
args = ["status", "postgresql"]

# Logs
[[menu_items]]
name = "Nginx Logs"
command = "tail"
args = ["-f", "/var/log/nginx/access.log"]

[[menu_items]]
name = "Application Logs"
command = "tail"
args = ["-f", "/var/log/app/app.log"]

[[menu_items]]
name = "System Logs"
command = "journalctl"
args = ["-f"]

# Deployment
[[menu_items]]
name = "Deploy Application"
command = "bash"
args = ["-c", "cd /var/www/app && ./deploy.sh"]

[[menu_items]]
name = "Rollback Deployment"
command = "bash"
args = ["-c", "cd /var/www/app && ./rollback.sh"]

# Configuration
[[menu_items]]
name = "Edit Dott Config"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []

[terminal_colors]
shape = "circles"
```

## Data Science Workflow

Configuration for data science and analysis:

```toml
logo_type = "custom"
custom_logo_path = "~/.config/dott/data-logo.txt"

# Notebooks & IDEs
[[menu_items]]
name = "Jupyter Lab"
command = "jupyter"
args = ["lab"]

[[menu_items]]
name = "Jupyter Notebook"
command = "jupyter"
args = ["notebook"]

[[menu_items]]
name = "VS Code"
command = "code"
args = ["~/projects/data"]

# Python Environment
[[menu_items]]
name = "Python Shell"
command = "python3"
args = []

[[menu_items]]
name = "IPython"
command = "ipython"
args = []

# Data Tools
[[menu_items]]
name = "Browse Data"
command = "yazi"
args = ["~/data"]

[[menu_items]]
name = "CSV Viewer"
command = "visidata"
args = []

# Analysis
[[menu_items]]
name = "Run Analysis"
command = "python3"
args = ["~/scripts/analyze.py"]

[[menu_items]]
name = "Generate Report"
command = "python3"
args = ["~/scripts/generate_report.py"]

# Version Control
[[menu_items]]
name = "Git Status"
command = "lazygit"
args = []

# Configuration
[[menu_items]]
name = "Edit Dott Config"
command = ""
args = []

[[menu_items]]
name = "Quit"
command = ""
args = []

[terminal_colors]
shape = "squares"

[clock]
position = "bottom"
```

## Custom Logo Files

### Developer Logo (dev-logo.txt)
```
╔═══════════════════════════════╗
║    < DEVELOPMENT MODE />      ║
║                               ║
║   Code • Build • Deploy       ║
╚═══════════════════════════════╝
```

### Minimal Logo (minimal-logo.txt)
```
  •
```

### Writer Logo (writer-logo.txt)
```
   ✎  W R I T E R   M O D E
  ───────────────────────────
      Create • Edit • Publish
```

### Data Logo (data-logo.txt)
```
  ╔══════════════════════╗
  ║  DATA  SCIENCE  LAB  ║
  ║   Analyze • Visualize║
  ╚══════════════════════╝
```

## Tips for Creating Your Own

1. **Start simple**: Begin with basic configuration
2. **Add gradually**: Add items as you need them
3. **Test commands**: Verify each command works before adding
4. **Organize logically**: Group related items together
5. **Use comments**: TOML supports `#` for comments
6. **Keep menus short**: 10-15 items is ideal
7. **Customize logos**: Create ASCII art that fits your style

## See Also

- [Configuration](Configuration.md) - Configuration reference
- [Customization](Customization.md) - Advanced customization
- [Usage](Usage.md) - How to use dott
