# Rusty PDF 

A blazingly fast, modular command-line interface (CLI) for manipulating PDF files, written in Rust.

## Features

* **Fast:** Built on `lopdf` for efficient parsing and minimal memory overhead.
* **Merge:** Stitch multiple PDF documents into a single file with correct page tree restructuring.
* **Extract Images:** Deep-scan PDFs to extract embedded images (supports JPEG and FlateDecode/PNG).
* **Rotate:** Modify page metadata in-place to rotate pages by 90, 180, or 270 degrees.
* **Cross-Platform:** Runs natively on Linux and Windows.

## Installation

Go to the [Releases Page](../../releases) and download the executable for your operating system.

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
