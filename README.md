# imginspect

`imginspect` is a Rust **CLI tool** for inspecting the **binary structure and metadata** of image files.

Supported formats:

* **PNG**
* **JPEG**
* **DNG (TIFF-based RAW)**

---

## What it does

* Detects image format using **magic bytes**
* Extracts **image dimensions**
* Extracts **format-specific metadata**
* Shows **binary-level information** (headers, markers, chunk layout)
* Works directly on raw bytes (no rendering, no conversion)

---

##

---

## Build

```bash
cargo build
```

---

## Usage

General form:

```bash
imginspect <COMMAND> <image-file>
```

When using Cargo:

```bash
cargo run -- <COMMAND> <image-file>
```

---

## Commands

### Inspect structure

```bash
cargo run -- inspect image.png
```

### Extract metadata (recommended)

```bash
cargo run -- metadata test.jpg
```

---

## Example Output (JPEG)

```text
Format: JPEG
Dimensions: 5712 x 4284

Metadata:
  Encoding: Baseline DCT

Binary Info:
  Binary size: 4881982 bytes
  SOI marker bytes: FF D8
  First segment bytes: FF E0 00 14
```

---

## Project Structure

```text
src/
├── main.rs      # CLI entry
├── args.rs      # CLI parsing
├── format.rs   # Format detection
├── report.rs   # Output model
├── png/         # PNG parsing
├── jpeg/        # JPEG parsing
└── dng/         # DNG/TIFF parsing
```

---

## Roadmap

* JSON output
* EXIF tag decoding
* PNG pixel extraction
* DNG RAW data extraction

---

## License

MIT
