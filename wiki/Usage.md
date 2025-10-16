# Usage

This guide covers how to use dott effectively, including keyboard shortcuts, navigation, and interaction with menu items.

## Starting Dott

Simply run the `dott` command in your terminal:

```bash
dott
```

The TUI interface will launch, displaying:
- Logo (ASCII art or image)
- Menu items
- Selected item command
- Optional terminal colors
- Optional clock
- Keyboard shortcuts help

## Interface Layout

```
┌─────────────────────────────────────────┐
│                                         │
│         [Logo - 10% from top]           │
│                                         │
├─────────────────────────────────────────┤
│                                         │
│          > View Dotfiles                │
│          > Edit Neovim Config           │
│          > Edit Shell Config            │
│          > System Monitor               │
│          > Git Status                   │
│          > Configure Dott               │
│          > Quit                         │
│                                         │
│    Command: yazi ~/.config              │
│                                         │
│    [Optional: Terminal Colors]          │
│                                         │
├─────────────────────────────────────────┤
│  ↑/k: Up | ↓/j: Down | Enter: Select   │
│            [Optional: Clock]             │
└─────────────────────────────────────────┘
```

## Keyboard Navigation

Dott supports both arrow keys and vim-style navigation:

### Basic Navigation

| Key | Action |
|-----|--------|
| `↓` or `j` | Move down to next menu item |
| `↑` or `k` | Move up to previous menu item |
| `Enter` | Select/execute current menu item |
| `q` or `Esc` | Quit dott |

### Navigation Details

- **Wrapping**: When at the bottom item, pressing down wraps to the top
- **Wrapping**: When at the top item, pressing up wraps to the bottom
- **Visual feedback**: Selected item is highlighted with cyan background
- **Live preview**: Command for selected item is shown below the menu

## Executing Menu Items

When you press `Enter` on a menu item:

1. **TUI exits temporarily** - The interface is hidden
2. **Command executes** - Your selected command runs
3. **Interactive support** - Commands that need input work normally
4. **Return prompt** - After command finishes, press `q` to return
5. **TUI restores** - The interface reappears

### Example Flow

```
1. User selects "System Monitor" (htop)
2. Presses Enter
3. Dott hides TUI, runs htop
4. User quits htop
5. Prompt: "Press 'q' to return to menu..."
6. User presses 'q'
7. Dott TUI reappears
```

## Built-in Commands

Some menu items have special built-in behavior:

### Edit Dott Config

```
Name: "Edit Dott Config"
```

- Opens `~/.config/dott/config.toml` in neovim
- Requires neovim to be installed
- Changes take effect on next dott launch

### View Shell

```
Name: "View Shell"
```

- Automatically detects your shell (bash, zsh, fish, etc.)
- Opens the appropriate config file in neovim
  - bash: `~/.bashrc`
  - zsh: `~/.zshrc`
  - fish: `~/.config/fish/config.fish`
  - ksh: `~/.kshrc`
  - tcsh: `~/.tcshrc`

### Quit

```
Name: "Quit"
```

- Exits dott completely
- Same as pressing `q` or `Esc`

## Working with Commands

### Simple Commands

Commands without arguments:

```bash
htop        # System monitor
lazygit     # Git TUI
ranger      # File manager
```

### Commands with Arguments

Commands with multiple arguments:

```bash
nvim ~/.bashrc                    # Edit file
yazi ~/.config                    # Browse directory
git log --oneline --graph         # Git with options
```

### Complex Commands

Running scripts or complex commands:

```bash
bash -c "echo 'Hello' && sleep 2"
sudo apt update && sudo apt upgrade
```

### Path Expansion

Dott automatically expands `~` to your home directory:

```bash
nvim ~/.config/nvim/init.lua    # Becomes /home/user/.config/nvim/init.lua
cd ~/projects                   # Becomes /home/user/projects
```

## Command Preview

At the bottom of the menu, dott shows the command that will be executed:

```
Command: nvim ~/.bashrc
```

This helps you:
- Verify the correct command before executing
- Learn what commands your menu items run
- Debug configuration issues

### Built-in Command Preview

Built-in commands show their equivalent command:

```
Command: nvim ~/.config/dott/config.toml    # Edit Dott Config
Command: nvim ~/.zshrc                       # View Shell (with zsh)
Command: exit                                # Quit
```

## Working with the Clock

If enabled, the clock displays at the bottom:

```
17:34:52
```

- **Live updates**: Clock updates every 100ms
- **Format**: 24-hour format (HH:MM:SS)
- **No interaction**: Clock is display-only

## Working with Terminal Colors

If enabled, terminal colors display below the menu:

### Circles Format
```
● ● ● ● ● ● ● ●
```

### Squares Format
```
■ ■ ■ ■
■ ■ ■ ■
```

The 8 standard terminal colors are shown:
- Black, Red, Green, Yellow, Blue, Magenta, Cyan, White

## Tips and Tricks

### Efficient Navigation

1. **Use vim keys**: `j` and `k` are faster than arrow keys
2. **Know the order**: Memorize your menu item positions
3. **Wrapping**: Use wrapping to quickly jump from bottom to top

### Quick Access

Create menu items for frequently used actions:

```toml
[[menu_items]]
name = "Quick Edit"
command = "nvim"
args = ["~/.config/nvim/init.lua"]
```

### Command Chaining

Use shell to chain multiple commands:

```toml
[[menu_items]]
name = "Update Everything"
command = "bash"
args = ["-c", "sudo apt update && sudo apt upgrade && cargo install-update -a"]
```

### Conditional Commands

Use shell conditionals in commands:

```toml
[[menu_items]]
name = "Git Pull"
command = "bash"
args = ["-c", "cd ~/projects && git pull || echo 'Not a git repository'"]
```

## Common Workflows

### Dotfile Management

```
1. Launch dott
2. Select "View Dotfiles" (yazi)
3. Navigate and edit files
4. Exit yazi
5. Press 'q' to return to dott
6. Press 'q' to exit dott
```

### Configuration Editing

```
1. Launch dott
2. Select "Edit Dott Config"
3. Make changes in neovim
4. Save and quit (:wq)
5. Press 'q' to return to dott
6. Press 'q' to exit dott
7. Launch dott again to see changes
```

### Quick Command Execution

```
1. Launch dott
2. Navigate to command (j/k)
3. Press Enter
4. Command executes
5. Press 'q' to return
6. Press 'q' to exit
```

## Troubleshooting Usage Issues

### Command Not Found

If a command doesn't work:
1. Test it in your regular shell first
2. Check if the program is installed
3. Verify the PATH includes the command location

### Command Doesn't Return

If a command hangs:
1. Press `Ctrl+C` to interrupt
2. Press `q` to return to dott
3. Check if the command needs user input
4. Add necessary input to the command args

### TUI Doesn't Restore

If the TUI doesn't restore after a command:
1. Press `q` to return to menu
2. If that doesn't work, press `Ctrl+C` and restart dott
3. Check terminal compatibility

## Advanced Usage

### Running Dott on Login

Add to your `~/.bashrc` or `~/.zshrc`:

```bash
# Run dott on interactive shell startup
if [ -t 1 ]; then
    dott
fi
```

### Aliasing Dott

Create quick aliases:

```bash
alias d='dott'
alias dott-config='nvim ~/.config/dott/config.toml'
```

### Integration with Tmux

Run dott in a tmux pane:

```bash
tmux new-session -s dott dott
```

## See Also

- [Configuration](Configuration.md) - Configure menu items and appearance
- [Features](Features.md) - Detailed feature descriptions
- [Troubleshooting](Troubleshooting.md) - Common issues and solutions
