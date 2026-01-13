# Rusty PDF 

A blazingly fast, modular command-line interface (CLI) for manipulating PDF files, written in Rust.

## Features

* **Fast:** Built on `lopdf` for efficient parsing and minimal memory overhead.
* **Merge:** Stitch multiple PDF documents into a single file with correct page tree restructuring.
* **Extract Images:** Deep-scan PDFs to extract embedded images (supports JPEG and FlateDecode/PNG).
* **Rotate:** Modify page metadata in-place to rotate pages by 90, 180, or 270 degrees.
* **Cross-Platform:** Runs natively on Linux and Windows.

## Installation

### Method 1: Download Binaries (Recommended)
Go to the [Releases Page](../../releases) and download the executable for your operating system.

**Linux Users:**
1. Download `rusty_pdf-linux`.
2. Open your terminal in the downloads folder.
3. Make it executable and run it:
```bash
chmod +x rusty_pdf-linux
./rusty_pdf-linux --help
```
4. **(Optional)** If you want to run rusty_pdf from anywhere without typing ./ or finding the folder every time, move it to your user's binary folder.
```bash
# Rename it to something short and move it
sudo mv rusty_pdf-linux /usr/local/bin/rusty_pdf

# Now you can just type this from any folder:
rusty_pdf --help
```

**Windows Users:**
1. Download `rusty_pdf.exe`.
2. Open COmmand Prompt or Powershell in the downloads folder.
3. Run:
```Powershell
.\rusty_pdf.exe --help
```

### Method 2: Build from Source
(If you have Rust installed)
```bash
git clone [https://github.com/lemongoreng/rusty_pdf](https://github.com/lemongoreng/rusty_pdf)
cd rusty_pdf
cargo install --path .
```

## Usage
1. Merge PDFs
```bash
rusty_pdf merge --output combined.pdf file1.pdf file2.pdf
```

2. Extract Images
```bash
rusty_pdf extract-images --input report.pdf --dir ./assets
```
3. Rotate Images
```bash
# Rotate 90 degrees
rusty_pdf rotate --input scanned.pdf --output fixed.pdf

# Rotate 180 degrees
rusty_pdf rotate --input scanned.pdf --output upside_down.pdf --angle 180
```

Ps: **Im still learning rust, dont @ me**
