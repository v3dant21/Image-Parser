mod parse;

use crate::error::InspectError;
use crate::report::ImageReport;

pub fn inspect_png(bytes: &[u8]) -> Result<ImageReport, InspectError> {
    let mut report = parse::parse(bytes)?;

    // ---- Binary-level info ----
    report
        .binary_info
        .push(format!("Binary size: {} bytes", bytes.len()));

    // PNG signature is always first 8 bytes
    let signature_preview = hex_preview(bytes, 8);
    report
        .binary_info
        .push(format!("PNG signature bytes: {}", signature_preview));

    // First chunk header (length + type = 8 bytes after signature)
    if bytes.len() >= 16 {
        let chunk_preview = hex_preview(&bytes[8..16], 8);
        report
            .binary_info
            .push(format!("First chunk header bytes: {}", chunk_preview));
    }

    Ok(report)
}

/// Hex preview helper
fn hex_preview(bytes: &[u8], max: usize) -> String {
    bytes.iter()
        .take(max)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
