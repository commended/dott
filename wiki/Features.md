# Features

Dott is packed with features designed to make managing your dotfiles and running commands effortless. This document provides detailed information about each feature.

## Core Features

### 1. Beautiful Terminal User Interface

Dott provides an elegant, minimalist TUI built with [ratatui](https://github.com/ratatui-org/ratatui):

- **Clean design**: Cyan-accented interface with clear visual hierarchy
- **Centered layout**: Logo and menu centered for visual appeal
- **Responsive**: Adapts to different terminal sizes
- **ANSI color support**: Works in any modern terminal

### 2. Vim-Style Keyboard Navigation

Navigate efficiently with familiar vim keybindings:

- `j` / `k` for down/up navigation
- Also supports arrow keys for universal access
- Fast, keyboard-only operation
- No mouse required

### 3. Customizable Menu System

Create personalized menus for your workflow:

- **Unlimited menu items**: Add as many as you need
- **Command execution**: Run any shell command or script
- **Arguments support**: Pass arguments to commands
- **Built-in commands**: Special commands for common tasks
- **Live preview**: See what command will run before executing

### 4. Logo Customization

Three logo display options:

#### Default ASCII Logo
Beautiful built-in dott ASCII art:
```
    ;'*¨'`·- .,  '                   , ·. ,.-·~·.,   '  
    \`:·-,. ,   '` ·.  '            /  ·'´,.-·-.,   `,
     '\:/   ;\:'`:·,  '`·, '        /  .'´\:::::::'\   '\
      ...
```

#### Custom ASCII Logo
- Use your own ASCII art
- Load from any text file
- Perfect for personal branding
- See [Customization Guide](Customization.md) for details

#### Image Logo (Experimental)
- Display PNG/JPEG images
- Uses Kitty graphics protocol
- Requires compatible terminal
- Displayed before TUI launch

### 5. Terminal Color Display

Visualize your terminal color scheme:

#### Circles Display
Shows 8 colors in a row using circle symbols (●):
```
● ● ● ● ● ● ● ●
```

#### Squares Display
Shows colors in 2 rows using square symbols (■):
```
■ ■ ■ ■
■ ■ ■ ■
```

Colors displayed:
- Black, Red, Green, Yellow
- Blue, Magenta, Cyan, White

### 6. Real-Time Clock

Optional live clock display:

- **Live updates**: Updates in real-time
- **24-hour format**: HH:MM:SS display
- **Configurable position**: Currently supports bottom position
- **Minimal overhead**: Updates every 100ms

### 7. Shell Auto-Detection

Intelligent shell configuration management:

Automatically detects and opens the correct config file for:
- **bash** → `~/.bashrc`
- **zsh** → `~/.zshrc`
- **fish** → `~/.config/fish/config.fish`
- **ksh** → `~/.kshrc`
- **tcsh** → `~/.tcshrc`

Falls back to `~/.bashrc` if shell is unknown.

### 8. Path Expansion

Automatic tilde (`~`) expansion:

- Works in all file paths
- Expands to `$HOME` directory
- Simplifies configuration
- Platform-independent

Example:
```toml
args = ["~/.config"]  # Becomes /home/user/.config
```

### 9. Interactive Command Support

Full support for interactive commands:

- Terminal control released during command execution
- Supports interactive tools like:
  - Text editors (nvim, vim, nano)
  - File managers (yazi, ranger, nnn)
  - System monitors (htop, btop)
  - Git tools (lazygit, tig)
  - Any interactive CLI tool

### 10. Command Preview

See what will execute before you run it:

```
> View Dotfiles

Command: yazi ~/.config
```

Benefits:
- Verify correct command
- Learn command syntax
- Debug configuration issues
- Educational for new users

## Advanced Features

### Configuration Management

Built-in configuration editing:

- **Quick access**: Dedicated "Edit Dott Config" menu item
- **Direct editing**: Opens in neovim
- **Structured format**: TOML configuration
- **Auto-creation**: Creates default config on first run
- **Validation**: Gracefully handles config errors

### Error Handling

Robust error handling throughout:

- **Missing config**: Creates default automatically
- **Invalid config**: Falls back to defaults with warning
- **Missing files**: Shows helpful error messages
- **Command failures**: Gracefully handled, returns to menu

### Terminal State Management

Proper terminal handling:

- **Raw mode**: Enabled for TUI, disabled for commands
- **Alternate screen**: Preserves terminal history
- **Mouse capture**: Optional mouse support
- **Cursor management**: Proper cursor show/hide
- **Signal handling**: Clean exit on interrupts

### Performance

Optimized for speed:

- **Rust-powered**: Native performance
- **Minimal dependencies**: Fast startup
- **Efficient rendering**: Only updates when needed
- **Low resource usage**: Minimal CPU and memory

## Feature Details

### Menu Item Execution Flow

1. User selects menu item
2. TUI exits (alternate screen disabled)
3. Terminal returns to normal mode
4. Command executes with full terminal access
5. User presses 'q' when done
6. TUI restores (alternate screen enabled)
7. Menu reappears at same position

### Logo Loading Priority

1. Check `logo_type` setting
2. If "default": Use built-in ASCII art
3. If "custom": Load from `custom_logo_path`
4. If "image": Display image, show placeholder in TUI
5. If file missing: Fall back to default logo

### Configuration Loading

```rust
1. Check ~/.config/dott/config.toml
2. If exists: Load and parse TOML
3. If parse error: Show warning, use defaults
4. If missing: Create default config file
5. Return Config object
```

### Shell Detection Algorithm

```rust
1. Read $SHELL environment variable
2. Check shell name:
   - Contains "zsh" → ~/.zshrc
   - Contains "bash" → ~/.bashrc
   - Contains "fish" → ~/.config/fish/config.fish
   - Contains "ksh" → ~/.kshrc
   - Contains "tcsh" → ~/.tcshrc
   - Default → ~/.bashrc
3. Return path as string with ~ notation
```

## Experimental Features

### Image Logo Support

**Status**: Experimental  
**Requirements**: Kitty terminal or compatible  
**Protocol**: Kitty graphics protocol

How it works:
1. Image read from file
2. Base64 encoded
3. Sent via Kitty escape sequence
4. Displayed before TUI launch
5. Placeholder shown in TUI

**Compatible terminals**:
- Kitty ✓
- WezTerm ✓
- Konsole (some versions) ✓
- Other terminals: Not supported

**Limitations**:
- Cannot display in TUI itself
- Shown before TUI launches
- Requires compatible terminal
- May not work in multiplexers (tmux, screen)

## Planned Features

Features under consideration for future releases:

- Multiple configuration profiles
- Theme support
- Plugin system
- Remote command execution
- Command history
- Search functionality
- Tabs/categories for menu items
- Custom keybindings
- Status bar customization

## Feature Requests

Have an idea for a new feature? Open an issue on [GitHub](https://github.com/commended/dott/issues) with:

1. Feature description
2. Use case
3. Expected behavior
4. Mockups or examples (optional)

## See Also

- [Configuration](Configuration.md) - How to configure features
- [Customization](Customization.md) - Customize appearance and behavior
- [Usage](Usage.md) - How to use each feature
- [Examples](Examples.md) - Example configurations
