mod parse;

use crate::error::InspectError;
use crate::report::ImageReport;

pub fn inspect_jpeg(bytes: &[u8]) -> Result<ImageReport, InspectError> {
    let mut report = parse::parse(bytes)?;

    // ---- Binary-level info ----
    report
        .binary_info
        .push(format!("Binary size: {} bytes", bytes.len()));

    // JPEG SOI marker (first 2 bytes)
    let soi_preview = hex_preview(bytes, 2);
    report
        .binary_info
        .push(format!("SOI marker bytes: {}", soi_preview));

    // First segment marker (usually APP0 / APP1)
    if bytes.len() >= 4 {
        let marker_preview = hex_preview(&bytes[2..6], 4);
        report
            .binary_info
            .push(format!("First segment bytes: {}", marker_preview));
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
