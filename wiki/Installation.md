# Installation

This guide covers all methods of installing dott on your system.

## Quick Install (from GitHub)

The fastest way to install dott is directly from the GitHub repository:

```bash
cargo install --git https://github.com/commended/dott
```

This will download, compile, and install dott to your Cargo bin directory (typically `~/.cargo/bin`).

### Make sure Cargo bin is in your PATH

Add this to your shell configuration file (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Install from Source

If you want to build dott from source or contribute to development:

### 1. Clone the Repository

```bash
git clone https://github.com/commended/dott.git
cd dott
```

### 2. Build the Project

```bash
cargo build --release
```

The compiled binary will be located at `target/release/dott`.

### 3. Install Globally (Optional)

Copy the binary to a directory in your PATH:

```bash
sudo cp target/release/dott /usr/local/bin/
```

Or for a user-local installation:

```bash
mkdir -p ~/.local/bin
cp target/release/dott ~/.local/bin/
```

Make sure `~/.local/bin` is in your PATH:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Updating dott

### If installed via cargo install

```bash
cargo install --git https://github.com/commended/dott --force
```

The `--force` flag will overwrite the existing installation.

### If installed from source

```bash
cd dott
git pull
cargo build --release
sudo cp target/release/dott /usr/local/bin/
```

## Requirements

### Essential Requirements

- **Rust 1.70 or later** - Required for building dott
  ```bash
  # Install Rust using rustup
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Terminal with ANSI color support** - Most modern terminals support this

### Optional Requirements

- **yazi** - File manager for browsing dotfiles
  - GitHub: https://github.com/sxyazi/yazi
  - Install: Follow instructions on their repository

- **neovim** - Text editor for editing configuration files
  - Website: https://neovim.io/
  - Install: `sudo apt install neovim` (Ubuntu/Debian) or see their installation guide

- **Kitty terminal** - For image logo support (experimental feature)
  - Website: https://sw.kovidgoyal.net/kitty/
  - Required only if you want to use image logos

## Verification

After installation, verify that dott is working:

```bash
# Check if dott is in your PATH
which dott

# Check the version (if implemented)
dott --version

# Run dott
dott
```

You should see the dott TUI interface with the default ASCII logo.

## First Run

On first run, dott will automatically create a default configuration file at:

```
~/.config/dott/config.toml
```

You can edit this file to customize dott's behavior. See [Configuration](Configuration.md) for details.

## Troubleshooting

### "command not found: dott"

Make sure the Cargo bin directory is in your PATH. Add this to your shell configuration:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Then reload your shell configuration:

```bash
source ~/.bashrc  # or ~/.zshrc
```

### Build Errors

If you encounter build errors:

1. **Update Rust**: `rustup update`
2. **Check Rust version**: `rustc --version` (should be 1.70+)
3. **Clean and rebuild**: `cargo clean && cargo build --release`

### Missing Dependencies

On some Linux distributions, you may need additional development packages:

```bash
# Ubuntu/Debian
sudo apt install build-essential pkg-config

# Fedora/RHEL
sudo dnf install gcc pkg-config

# Arch Linux
sudo pacman -S base-devel
```

## Next Steps

- [Configure dott](Configuration.md) to customize the interface
- [Learn the features](Features.md) to get the most out of dott
- [See examples](Examples.md) for inspiration
