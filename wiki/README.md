# Wiki Documentation

This folder contains comprehensive documentation for the dott project, ready to be moved to GitHub Wiki.

## Contents

1. **[Home.md](Home.md)** - Main landing page with overview and quick links
2. **[Installation.md](Installation.md)** - Installation instructions for all platforms
3. **[Configuration.md](Configuration.md)** - Complete configuration reference
4. **[Usage.md](Usage.md)** - How to use dott, keyboard shortcuts, and workflows
5. **[Features.md](Features.md)** - Detailed descriptions of all features
6. **[Customization.md](Customization.md)** - Advanced customization options
7. **[Examples.md](Examples.md)** - Real-world configuration examples
8. **[Troubleshooting.md](Troubleshooting.md)** - Common issues and solutions
9. **[FAQ.md](FAQ.md)** - Frequently asked questions
10. **[Development.md](Development.md)** - Guide for contributors and developers

## Statistics

- **Total Files**: 10 markdown files
- **Total Lines**: ~3,845 lines
- **Total Size**: ~96 KB

## How to Use

### Option 1: GitHub Wiki (Recommended)

To move these files to GitHub Wiki:

1. Clone the wiki repository:
   ```bash
   git clone https://github.com/commended/dott.wiki.git
   ```

2. Copy all markdown files from this folder to the wiki repo:
   ```bash
   cp wiki/*.md dott.wiki/
   ```

3. Commit and push:
   ```bash
   cd dott.wiki
   git add .
   git commit -m "Add comprehensive documentation"
   git push
   ```

### Option 2: View Locally

Read the markdown files in any markdown viewer:
- GitHub's web interface
- VS Code with markdown preview
- Grip: `grip wiki/Home.md`
- Any markdown editor

### Option 3: Generate HTML

Convert to HTML using pandoc or similar tools:
```bash
pandoc wiki/Home.md -o Home.html
```

## Documentation Coverage

### For Users
- ✅ Installation guide for all platforms
- ✅ Configuration reference with examples
- ✅ Usage instructions with keyboard shortcuts
- ✅ Feature descriptions
- ✅ Customization guide (logos, colors, clock)
- ✅ Real-world configuration examples
- ✅ Troubleshooting guide
- ✅ FAQ with common questions

### For Developers
- ✅ Project structure explanation
- ✅ Architecture overview
- ✅ Code style guidelines
- ✅ Contribution guidelines
- ✅ Development setup
- ✅ Building and testing instructions
- ✅ Release process

## Navigation Structure

The documentation is organized with cross-references:

```
Home.md (Landing page)
  ├── Installation.md
  ├── Configuration.md
  │   ├── Customization.md
  │   └── Examples.md
  ├── Usage.md
  ├── Features.md
  ├── Troubleshooting.md
  ├── FAQ.md
  └── Development.md
```

Each page includes "See Also" links to related pages for easy navigation.

## Maintenance

When updating documentation:

1. **Keep it current**: Update when features change
2. **Cross-reference**: Link to related pages
3. **Add examples**: Include practical examples
4. **Test instructions**: Verify commands work
5. **Screenshots**: Add for UI changes (if applicable)

## Contributing to Documentation

To improve the documentation:

1. Edit the markdown files in this folder
2. Follow the existing style and structure
3. Test all code examples
4. Update cross-references if needed
5. Submit a pull request

## License

This documentation is part of the dott project and follows the same MIT License.
