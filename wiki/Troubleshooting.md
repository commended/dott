# Troubleshooting

This guide helps you resolve common issues with dott.

## Installation Issues

### "command not found: cargo"

**Problem**: Rust and Cargo are not installed.

**Solution**:
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell
source $HOME/.cargo/env

# Verify installation
cargo --version
```

### "command not found: dott"

**Problem**: dott binary is not in PATH.

**Solution**:
```bash
# Check if dott exists
ls ~/.cargo/bin/dott

# If it exists, add Cargo bin to PATH
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or for zsh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### Build Errors

**Problem**: Compilation fails during `cargo install` or `cargo build`.

**Solution**:
```bash
# Update Rust
rustup update

# Check Rust version (must be 1.70+)
rustc --version

# Clean and rebuild
cd dott
cargo clean
cargo build --release

# Install missing build dependencies
# Ubuntu/Debian
sudo apt install build-essential pkg-config

# Fedora/RHEL
sudo dnf install gcc pkg-config

# Arch Linux
sudo pacman -S base-devel
```

## Runtime Issues

### Configuration File Errors

**Problem**: "Error parsing config" message on startup.

**Solution**:
```bash
# Check if config file exists
ls ~/.config/dott/config.toml

# Validate TOML syntax
cat ~/.config/dott/config.toml

# Common issues:
# - Missing quotes around strings
# - Incorrect array syntax
# - Missing [[menu_items]] headers

# Reset to defaults
rm ~/.config/dott/config.toml
dott  # Will create new default config
```

### Configuration Not Loading

**Problem**: Changes to config.toml don't appear in dott.

**Solution**:
1. Ensure you saved the file after editing
2. Check file location: `~/.config/dott/config.toml`
3. Restart dott completely (quit and relaunch)
4. Check for TOML syntax errors

### Menu Items Not Working

**Problem**: Selecting menu item does nothing or shows error.

**Solution**:
```bash
# Test command in regular shell first
yazi ~/.config

# Check if command is installed
which yazi

# Install missing command
cargo install yazi-fm  # for yazi
sudo apt install neovim  # for nvim

# Check command path
# If command is not in PATH, use absolute path
[[menu_items]]
name = "My Tool"
command = "/usr/local/bin/mytool"
args = []
```

### Path Expansion Issues

**Problem**: Tilde (~) not expanding to home directory.

**Solution**:
- Dott automatically expands `~` in configuration
- If not working, check for typos:
  ```toml
  # Correct
  args = ["~/.config"]
  
  # Incorrect (extra space)
  args = ["~ /.config"]
  ```
- Use absolute paths as fallback:
  ```toml
  args = ["/home/username/.config"]
  ```

## Display Issues

### Terminal Not Displaying Colors

**Problem**: No colors shown, or strange characters appear.

**Solution**:
```bash
# Check if terminal supports ANSI colors
echo -e "\033[31mRed\033[0m \033[32mGreen\033[0m \033[34mBlue\033[0m"

# Set TERM environment variable
export TERM=xterm-256color

# Add to shell config
echo 'export TERM=xterm-256color' >> ~/.bashrc

# Try different terminal emulators
# - Alacritty (recommended)
# - Kitty
# - iTerm2 (macOS)
# - Windows Terminal (Windows)
```

### Logo Not Displaying Correctly

**Problem**: ASCII logo appears broken or misaligned.

**Solution**:
1. **Check terminal width**: Ensure terminal is wide enough
   ```bash
   # Check terminal size
   tput cols  # Should be at least 80
   ```

2. **Custom logo issues**:
   ```bash
   # Verify file exists
   cat ~/.config/dott/custom-logo.txt
   
   # Check for special characters
   # Use only standard ASCII characters
   ```

3. **Reset to default logo**:
   ```toml
   logo_type = "default"
   # Remove or comment out custom_logo_path
   ```

### Image Logo Not Displaying

**Problem**: Image logo doesn't appear.

**Solution**:
1. **Check terminal support**:
   ```bash
   # Kitty terminal
   echo $TERM  # Should contain "kitty"
   
   # Test Kitty graphics protocol
   kitty +kitten icat image.png
   ```

2. **Verify image file**:
   ```bash
   # Check if file exists
   ls -lh ~/.config/dott/logo.png
   
   # Verify it's a valid image
   file ~/.config/dott/logo.png
   ```

3. **Use ASCII fallback**:
   ```toml
   # Switch to ASCII logo
   logo_type = "default"
   ```

### TUI Not Restoring After Command

**Problem**: After running a command, TUI doesn't come back.

**Solution**:
1. Press `q` to return to menu
2. If frozen, press `Ctrl+C` to exit
3. Restart dott
4. Check if command is hanging:
   ```bash
   # Test command separately
   yazi ~/.config
   ```

## Command Execution Issues

### Command Hangs or Freezes

**Problem**: Selected command never returns or freezes.

**Solution**:
1. Press `Ctrl+C` to interrupt
2. Check if command expects input
3. Add input to command:
   ```toml
   [[menu_items]]
   name = "Update"
   command = "bash"
   args = ["-c", "yes | sudo apt upgrade"]
   ```

### Interactive Commands Not Working

**Problem**: Interactive tools (htop, nvim) don't work properly.

**Solution**:
- This should work automatically with dott
- If issues persist:
  ```bash
  # Check TERM variable
  echo $TERM
  
  # Try setting explicitly
  export TERM=xterm-256color
  ```

### Sudo Commands Asking for Password Repeatedly

**Problem**: Each sudo command prompts for password.

**Solution**:
1. **Use sudo with timestamp**:
   ```bash
   # Run sudo command first to cache credentials
   sudo -v
   ```

2. **Configure sudoers** (advanced):
   ```bash
   # Edit sudoers file
   sudo visudo
   
   # Add line (replace username):
   username ALL=(ALL) NOPASSWD: /usr/bin/apt update, /usr/bin/apt upgrade
   ```

### Command Not Found in Menu

**Problem**: Command works in shell but not in dott.

**Solution**:
```bash
# Find full path to command
which mycommand

# Use absolute path in config
[[menu_items]]
name = "My Command"
command = "/usr/local/bin/mycommand"
args = []

# Or ensure PATH is set
# Add to ~/.bashrc or ~/.zshrc:
export PATH="/usr/local/bin:$PATH"
```

## Performance Issues

### Slow Startup

**Problem**: Dott takes long to start.

**Solution**:
1. **Check config file size**:
   ```bash
   wc -l ~/.config/dott/config.toml
   ```

2. **Remove unnecessary menu items**
3. **Check for slow file operations**:
   - Large custom logo files
   - Inaccessible image files
   - Network drives in paths

### High CPU Usage

**Problem**: Dott uses excessive CPU.

**Solution**:
- This is unusual for dott
- Check for:
  - Infinite loops in custom scripts
  - Background processes started by menu items
  - Terminal emulator issues

## Shell-Specific Issues

### Shell Not Detected

**Problem**: "View Shell" menu item doesn't open correct config.

**Solution**:
```bash
# Check SHELL variable
echo $SHELL

# Set explicitly if needed
export SHELL=/bin/zsh

# Add to shell config
echo 'export SHELL=/bin/zsh' >> ~/.zshrc
```

### Zsh Completion Not Working

**Problem**: Tab completion doesn't work for dott.

**Solution**:
```bash
# Completion not yet implemented
# Run dott directly without completion
dott
```

## Platform-Specific Issues

### Linux Issues

**Problem**: Missing dependencies on fresh Linux install.

**Solution**:
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential pkg-config curl git

# Fedora/RHEL
sudo dnf install gcc pkg-config curl git

# Arch Linux
sudo pacman -S base-devel curl git
```

### macOS Issues

**Problem**: Build fails on macOS.

**Solution**:
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Homebrew if not present
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install dependencies
brew install rust
```

### Windows (WSL) Issues

**Problem**: Issues running dott in WSL.

**Solution**:
```bash
# Use WSL 2 (better terminal support)
wsl --set-version Ubuntu 2

# Install dependencies
sudo apt update
sudo apt install build-essential

# Use Windows Terminal for best experience
# Download from Microsoft Store
```

## Getting Help

If you can't resolve your issue:

1. **Check existing issues**: [GitHub Issues](https://github.com/commended/dott/issues)
2. **Search documentation**: Review all wiki pages
3. **Create a new issue**: Include:
   - Dott version
   - Operating system and version
   - Terminal emulator
   - Configuration file
   - Steps to reproduce
   - Error messages
   - Screenshots if relevant

### Information to Include

```bash
# System information
uname -a

# Rust version
rustc --version
cargo --version

# Terminal information
echo $TERM
echo $SHELL

# Dott configuration
cat ~/.config/dott/config.toml

# Test dott
dott 2>&1 | tee dott-output.log
```

## See Also

- [Installation](Installation.md) - Installation instructions
- [Configuration](Configuration.md) - Configuration guide
- [FAQ](FAQ.md) - Frequently asked questions
