# ASCII Art Support

Dott includes infrastructure for ASCII art and image-to-ASCII conversion, inspired by the [ascii-view](https://github.com/gouwsxander/ascii-view) project.

## Current Features

### Custom ASCII Logos

You can use any ASCII art as your logo by:

1. Creating a text file with your ASCII art
2. Updating your config to point to it:

```toml
logo_type = "custom"
custom_logo_path = "/path/to/your/ascii-art.txt"
```

### Default Logo

The default "dott" logo is embedded in the application:

```
      _       _   _   
   __| | ___ | |_| |_ 
  / _` |/ _ \| __| __|
 | (_| | (_) | |_| |_ 
  \__,_|\___/ \__|\__|
```

## ASCII Art Resources

### Creating Your Own ASCII Art

- **FIGlet**: Generate text-based ASCII art
  ```bash
  sudo apt install figlet
  figlet "My Text" > ~/.config/dott/my-logo.txt
  ```

- **Online generators**:
  - [patorjk.com/software/taag](https://patorjk.com/software/taag/)
  - [asciiart.eu](https://www.asciiart.eu/)

### Image-to-ASCII Conversion

For future enhancement, dott includes a placeholder module (`src/ascii_image.rs`) for converting images to ASCII art, based on concepts from ascii-view:

- Load images (JPEG, PNG, BMP)
- Convert to colorized ASCII with ANSI codes
- Smart resizing with aspect ratio preservation
- Edge detection and enhancement

This functionality can be extended in future versions.

## Examples

See the `examples/` directory for:
- `custom-logo.txt` - Example custom ASCII logo
- `config.toml` - Configuration examples

## Tips for Good-Looking ASCII Art

1. Keep your logo between 5-15 lines tall for best results
2. Use monospace-friendly characters
3. Test your logo in your actual terminal to see how it renders
4. Consider terminal color schemes when designing colored ASCII art
5. Simpler designs often look better at terminal scale
