# dott

[![License: MIT](https://img.shields.io/badge/License-MIT-white.svg)](https://opensource.org/licenses/MIT)
![GitHub Repo stars](https://img.shields.io/github/stars/commended/dott)

a beautiful and fast tui written in rust


## Installation

```bash
cargo install --git https://github.com/commended/dott
```

### From Source

```bash
cargo build --release
sudo cp target/release/dott /usr/local/bin/
```

## Updating

```bash
cargo install --git https://github.com/commended/dott --force
```

## Usage

Simply run:

```bash
dott
```

The TUI features:
- **Highlighted selection**: Selected menu items are highlighted with a cyan background
- **Keyboard navigation**: Use arrow keys or vim-style `j`/`k` to navigate
- **Quick actions**: Press Enter to execute the selected menu item


## Requirements

- Rust 1.70+ (for building)
- A terminal with ANSI color support
- yazi
- neovim

