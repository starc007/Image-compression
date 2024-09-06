# Image Compressor CLI

A command-line tool for compressing and resizing images in bulk. This tool can process entire directories, including nested folders, and maintains the original folder structure in the output.

## Features

- Compress and resize various image formats including JPEG, PNG, GIF, WebP, TIFF, TGA, BMP, ICO, and HDR
- Process entire directories, including nested folders
- Maintain original folder structure in the output
- Adjustable compression quality for JPEG images
- Automatic format detection and preservation

## Installation

### Prerequisites

- Rust programming language (https://www.rust-lang.org/tools/install)

### Building from source

1. Clone the repository:

   ```
   git clone https://github.com/starc007/Image-compression.git
   cd Image-compression
   ```

2. Build the project:
   ```
   cargo build --release
   ```

The executable will be created in the `target/release` directory.

## Usage

```
image_compressor_cli [OPTIONS]
```

### Options:

- `-i, --input <INPUT>`: Input directory path [default: .]
- `-o, --output <OUTPUT>`: Output directory path [default: compressed_images]
- `-q, --quality <QUALITY>`: Compression quality (0-100) [default: 85]

### Example:

```
./target/release/image_compressor_cli --input /path/to/input/folder --output /path/to/output/folder --quality 75
```

This command will compress all supported images in the input folder and its subfolders, saving the compressed images to the output folder while maintaining the original folder structure.

## How it works

1. The tool scans the input directory and all its subdirectories for supported image formats.
2. Each image is resized if its dimensions exceed the maximum allowed (currently set to 1280 pixels for the longest side).
3. JPEG images are compressed using the specified quality setting.
4. Other formats are resized (if necessary) and saved with their original format.
5. The compressed images are saved in the output directory, maintaining the original folder structure.

## Supported Formats

- JPEG (.jpg, .jpeg)
- PNG (.png)
- GIF (.gif)
- WebP (.webp)
- TIFF (.tiff)
- TGA (.tga)
- BMP (.bmp)
- ICO (.ico)
- HDR (.hdr)

## Limitations

- The tool does not currently support parallel processing, so it may be slow for very large numbers of images.
- Maximum image dimension is set to 1280 pixels. This can be changed in the source code if needed.
- Compression quality setting only affects JPEG images. Other formats are only resized if necessary.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This project uses the [image](https://github.com/image-rs/image) crate for image processing.
- Directory traversal is handled by the [walkdir](https://github.com/BurntSushi/walkdir) crate.
- Command-line argument parsing is done using the [clap](https://github.com/clap-rs/clap) crate.
