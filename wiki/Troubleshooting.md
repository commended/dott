# Troubleshooting

This guide helps you solve common issues when using dott.

## Installation Issues

### "cargo: command not found"

**Problem**: Rust toolchain is not installed.

**Solution**:

Install Rust using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal or source your shell config:

```bash
source ~/.bashrc  # or ~/.zshrc
```

### "dott: command not found"

**Problem**: The dott binary is not in your PATH.

**Solution 1**: Use the full path

```bash
~/.cargo/bin/dott
```

**Solution 2**: Add to PATH

Add to your shell config (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then reload:

```bash
source ~/.bashrc  # or ~/.zshrc
```

**Solution 3**: Create alias

```bash
alias dott='~/.cargo/bin/dott'
```

### Build Fails with Compilation Errors

**Problem**: Rust version too old or dependencies issue.

**Solution**:

Update Rust:

```bash
rustup update
```

Clean and rebuild:

```bash
cd /path/to/dott
cargo clean
cargo build --release
```

Ensure Rust version is 1.70+:

```bash
rustc --version
```

## Configuration Issues

### Config File Not Found

**Problem**: dott can't find or create the config file.

**Solution**:

Manually create the directory and file:

```bash
mkdir -p ~/.config/dott
touch ~/.config/dott/config.toml
```

Add a minimal config:

```toml
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

[custom]

[custom.selected]
```

### "Failed to parse config" Error

**Problem**: TOML syntax error in config file.

**Solution**:

Check for common TOML mistakes:

1. **Missing quotes around strings**:
   ```toml
   # Wrong
   name = Neovim
   
   # Correct
   name = "Neovim"
   ```

2. **Incorrect array syntax**:
   ```toml
   # Wrong
   args = [~/.config]
   
   # Correct
   args = ["~/.config"]
   ```

3. **Missing double brackets**:
   ```toml
   # Wrong
   [entries]
   
   # Correct
   [[entries]]
   ```

4. **Validate your TOML**:
   Use an online TOML validator or:
   ```bash
   # Install toml-cli
   cargo install toml-cli
   
   # Validate
   toml get ~/.config/dott/config.toml .
   ```

### Hot Reload (u key) Not Working

**Problem**: Changes don't apply when pressing `u`.

**Solution**:

1. **Check for syntax errors**: Invalid TOML prevents reload
2. **Save the file**: Ensure changes are saved to disk
3. **File permissions**: Check file is readable
   ```bash
   ls -l ~/.config/dott/config.toml
   ```
4. **Restart dott**: If hot reload fails, restart the application

### Custom Logo Not Displaying

**Problem**: Custom logo file doesn't show.

**Solution**:

1. **Check file path**:
   ```toml
   custom_logo_path = "~/.config/dott/my-logo.txt"
   ```
   Tilde (~) should expand to home directory.

2. **Verify file exists**:
   ```bash
   cat ~/.config/dott/my-logo.txt
   ```

3. **Check file permissions**:
   ```bash
   chmod 644 ~/.config/dott/my-logo.txt
   ```

4. **Test with absolute path**:
   ```toml
   custom_logo_path = "/home/username/.config/dott/my-logo.txt"
   ```

### Image Logo Not Working

**Problem**: Image logo doesn't display.

**Solution**:

1. **Check terminal support**: Image logos require Kitty protocol support
   - Works: Kitty terminal
   - May work: Some Kitty-compatible terminals
   - Won't work: Most other terminals

2. **Verify image format**: Use PNG format

3. **Check image size**: Keep images reasonably sized (under 100KB)

4. **Test in Kitty**:
   ```bash
   kitty +kitten icat ~/.config/dott/logo.png
   ```

5. **Fallback**: Use custom ASCII logo instead

## Display Issues

### Colors Not Showing

**Problem**: Terminal colors appear incorrect or missing.

**Solution**:

1. **Check terminal support**: Ensure ANSI color support
   ```bash
   echo -e "\033[31mRed\033[0m \033[32mGreen\033[0m \033[34mBlue\033[0m"
   ```

2. **Try different terminal**:
   - Alacritty
   - Kitty
   - iTerm2 (macOS)
   - Windows Terminal
   - GNOME Terminal

3. **Check TERM variable**:
   ```bash
   echo $TERM
   ```
   Should be something like `xterm-256color` or `screen-256color`.

4. **Set TERM if needed**:
   ```bash
   export TERM=xterm-256color
   ```

### UI Layout Looks Wrong

**Problem**: Modules overlap or spacing is off.

**Solution**:

1. **Check terminal size**:
   ```bash
   tput cols
   tput lines
   ```
   Ensure minimum 80 columns.

2. **Resize terminal**: Make terminal window larger

3. **Adjust logo width**: Use narrower custom logo

4. **Use break modules**:
   ```toml
   [[structure.build]]
   module = "break"
   
   [custom]
   [custom.break]
   lines = 2
   ```

### Text Appears Garbled

**Problem**: Unicode or special characters render incorrectly.

**Solution**:

1. **Check locale**:
   ```bash
   locale
   ```
   Should include UTF-8.

2. **Set locale**:
   ```bash
   export LC_ALL=en_US.UTF-8
   export LANG=en_US.UTF-8
   ```

3. **Check font**: Use a font with good Unicode support
   - JetBrains Mono
   - Fira Code
   - Hack
   - Source Code Pro

### Selection Highlighting Not Visible

**Problem**: Can't see which entry is selected.

**Solution**:

1. **Add selected module**:
   ```toml
   [[structure.build]]
   module = "selected"
   
   [custom]
   [custom.selected]
   ```

2. **Check terminal theme**: Some themes have poor contrast

3. **Try different terminal color scheme**

## Runtime Issues

### Commands Don't Execute

**Problem**: Pressing Enter on an entry does nothing.

**Solution**:

1. **Check command exists**:
   ```bash
   which nvim
   which yazi
   ```

2. **Verify command in config**:
   ```toml
   [[entries]]
   name = "Neovim"
   command = "nvim"  # Must be executable name or path
   args = []
   ```

3. **Test command in terminal**:
   ```bash
   nvim
   ```

4. **Use full path if needed**:
   ```toml
   command = "/usr/bin/nvim"
   ```

### Application Crashes on Startup

**Problem**: dott exits immediately or crashes.

**Solution**:

1. **Check for error messages**:
   ```bash
   dott 2>&1 | tee ~/dott-error.log
   ```

2. **Test with minimal config**:
   ```bash
   mv ~/.config/dott/config.toml ~/.config/dott/config.toml.backup
   dott
   ```

3. **Rebuild from source**:
   ```bash
   cd /path/to/dott
   cargo clean
   cargo build --release
   ```

4. **Check system resources**:
   ```bash
   free -h
   df -h
   ```

### Key Presses Not Responding

**Problem**: Navigation keys don't work.

**Solution**:

1. **Check terminal input mode**: Some terminals may intercept keys

2. **Try alternative keys**:
   - Arrow keys: ↑/↓
   - Vim keys: j/k
   - Both should work

3. **Restart terminal**

4. **Test in different terminal emulator**

### System Info Modules Show No Data

**Problem**: system_info, memory, disk, or uptime shows nothing.

**Solution**:

1. **Declare modules in [custom]**:
   ```toml
   [custom]
   [custom.system_info]
   [custom.memory]
   [custom.disk_usage]
   path = "/"
   ```

2. **Check platform support**: Some modules may not work on all OS

3. **Verify system access**: Ensure permissions to read system info

4. **Check sysinfo crate compatibility**: May vary by platform

## Performance Issues

### Slow Startup

**Problem**: dott takes a long time to start.

**Solution**:

1. **Minimize modules**: Remove system info modules
   ```toml
   # Remove these if slow:
   # system_info, uptime, memory, disk
   ```

2. **Reduce entry count**: Split across groups instead of one large list

3. **Simplify logo**: Use default or smaller custom logo

4. **Check system load**:
   ```bash
   uptime
   top
   ```

### High CPU Usage

**Problem**: dott uses excessive CPU.

**Solution**:

1. **Update to latest version**:
   ```bash
   cargo install --git https://github.com/commended/dott --force
   ```

2. **Remove clock module temporarily**: Test if clock refresh causes issue

3. **Check for infinite loops**: Report issue if persistent

### Memory Usage High

**Problem**: dott uses too much memory.

**Solution**:

1. **Restart dott**: Memory may accumulate over time

2. **Reduce config complexity**: Fewer modules = less memory

3. **Report issue**: If memory usage is excessive (>100MB), this may be a bug

## Platform-Specific Issues

### macOS: "Developer cannot be verified"

**Problem**: Security warning when running dott.

**Solution**:

```bash
xattr -d com.apple.quarantine ~/.cargo/bin/dott
```

Or build from source instead of using a downloaded binary.

### Linux: Permission Denied

**Problem**: Can't execute dott binary.

**Solution**:

```bash
chmod +x ~/.cargo/bin/dott
```

### Windows: Path Issues

**Problem**: Windows paths don't work in config.

**Solution**:

Use forward slashes or escape backslashes:

```toml
# Option 1: Forward slashes
args = ["C:/Users/name/file.txt"]

# Option 2: Escaped backslashes
args = ["C:\\Users\\name\\file.txt"]
```

### SSH/Remote: Display Issues

**Problem**: UI looks wrong over SSH.

**Solution**:

1. **Set TERM on remote**:
   ```bash
   export TERM=xterm-256color
   ```

2. **Enable color in SSH client**

3. **Use simpler layout**: Avoid fancy Unicode characters

## Debug Mode

To get more information about errors:

1. **Check Rust backtrace**:
   ```bash
   RUST_BACKTRACE=1 dott
   ```

2. **Build with debug symbols**:
   ```bash
   cargo build  # instead of --release
   ./target/debug/dott
   ```

3. **Save error output**:
   ```bash
   dott 2>&1 | tee ~/dott-debug.log
   ```

## Still Having Issues?

If you can't resolve your issue:

1. **Check existing issues**: Visit GitHub issues page
2. **Create detailed bug report**:
   - Your OS and version
   - Terminal emulator and version
   - Rust version (`rustc --version`)
   - Your config.toml (remove sensitive info)
   - Error messages or unexpected behavior
   - Steps to reproduce

3. **Include diagnostic info**:
   ```bash
   echo "OS: $(uname -a)"
   echo "Terminal: $TERM"
   echo "Rust: $(rustc --version)"
   echo "Cargo: $(cargo --version)"
   ```

## Common Error Messages

### "No such file or directory"

Config or logo file path is wrong. Check paths and file existence.

### "Permission denied"

File permissions issue. Use `chmod` to fix.

### "Failed to execute command"

Command in entry doesn't exist or isn't in PATH.

### "Invalid TOML"

Syntax error in config.toml. Validate TOML syntax.

### "Module not found"

Custom module used without declaration in [custom] section.

## Quick Fixes Checklist

When something goes wrong:

- [ ] Save config file
- [ ] Check TOML syntax
- [ ] Verify file paths exist
- [ ] Reload config with `u` key
- [ ] Restart dott
- [ ] Check terminal supports required features
- [ ] Review recent config changes
- [ ] Test with minimal config
- [ ] Check error messages
- [ ] Update to latest version

## Getting Help

Remember: Most issues are configuration-related. Double-check your config.toml!

For support:
- Review this troubleshooting guide
- Check other wiki pages for detailed documentation
- Search existing GitHub issues
- Open a new issue with details if needed

Happy troubleshooting!
