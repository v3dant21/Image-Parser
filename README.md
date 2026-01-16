# imginspect

imginspect is a Rust CLI tool that inspects and validates the binary structure of image files.

## Supported formats
- PNG
- JPEG
- DNG (TIFF-based)

## What it does
- Detects image format via magic bytes
- Parses structural metadata
- Reports dimensions and key format elements

## What it does NOT do
- Decode or render images
- Convert formats
- Process pixel data

This tool focuses on correctness, safety, and extensibility.
