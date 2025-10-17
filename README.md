# PDF-page-extractor# PDF Page Extractor

A simple GUI application built with Rust that allows you to extract specific pages from PDF files.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)

## Features

- 🖥️ Simple and intuitive GUI interface
- 📄 Extract single pages or page ranges from PDFs
- 🎯 Support for complex page selections (e.g., "1-3,5,7-9")
- ✅ Input validation and error handling
- 🚀 Fast and lightweight

## Prerequisites

### For Linux/WSL Users

#### 1. Install Rust

Install Rust using rustup (the official Rust toolchain installer):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions. When prompted, choose option 1 (default installation).

After installation, configure your current shell:

```bash
source "$HOME/.cargo/env"
```

Verify the installation:

```bash
rustc --version
cargo --version
```

You should see version information for both commands.

#### 2. Install System Dependencies

The GUI components require some system libraries. Install them based on your distribution:

**Ubuntu/Debian/WSL with Ubuntu:**

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libgtk-3-dev libssl-dev
```

#### 3. WSL-Specific Setup

**WSL2 with [WSLg](https://github.com/microsoft/wslg)**

WSLg is included by default in recent versions. No additional setup needed! GUI apps will work out of the box.

To verify you have WSLg:
```bash
echo $DISPLAY
```
If you see output like `:0`, you're good to go.

## Installation

Clone the repository:

```bash
git clone https://github.com/archibald-carrion/PDF-page-extractor
cd pdf-page-extractor
```

## Building

Build the release version (recommended for better performance):

```bash
cargo build --release
```

The compiled binary will be in `target/release/pdf_extractor`

## Usage

Run the application:

```bash
cargo run --release
```

Or run the compiled binary directly:

```bash
./target/release/pdf_extractor
```

### How to Extract Pages

1. **Select Input PDF**: Click "Browse" next to "Input PDF" and select your PDF file
2. **Choose Output Location**: Click "Browse" next to "Output PDF" and choose where to save
3. **Specify Pages**: Enter the page numbers you want to extract
4. **Extract**: Click "Extract Pages" button

### Page Range Format

- **Single pages**: `1,3,5` - extracts pages 1, 3, and 5
- **Page ranges**: `1-5` - extracts pages 1 through 5
- **Combined**: `1-3,7,10-12` - extracts pages 1-3, 7, and 10-12

## Project Structure

```
pdf-page-extractor/
├── Cargo.toml          # Project dependencies and metadata
├── src/
│   └── main.rs         # Main application code
└──README.md           # This file
```

## Dependencies

- `eframe` - GUI framework built on egui
- `egui` - Immediate mode GUI library
- `lopdf` - PDF manipulation library
- `rfd` - Native file dialogs

## Troubleshooting

### "error: linker 'cc' not found"

Install build tools:
```bash
sudo apt install build-essential
```

### "Package gtk+-3.0 was not found"

Install GTK development libraries:
```bash
sudo apt install libgtk-3-dev
```

### GUI doesn't appear in WSL

Make sure you have an X server running (see WSL-Specific Setup above) or upgrade to WSL2 with WSLg support.

Test if GUI works:
```bash
sudo apt install x11-apps
xeyes
```

If `xeyes` doesn't work, your X server isn't configured correctly.

### "Failed to load PDF" error

- Verify the PDF file isn't corrupted
- Make sure you have read permissions for the file
- Some encrypted or protected PDFs may not work

### "Page X is out of range" error

The page number you specified doesn't exist in the PDF. PDF pages are numbered starting from 1.

## Building for Windows (from WSL)

To cross-compile for Windows from WSL:

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW
sudo apt install mingw-w64

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

The Windows executable will be in `target/x86_64-pc-windows-gnu/release/pdf_extractor.exe`

## Acknowledgments

- Built with [egui](https://github.com/emilk/egui) - an excellent immediate mode GUI library
- PDF handling powered by [lopdf](https://github.com/J-F-Liu/lopdf)
