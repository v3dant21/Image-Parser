mod parse;

use crate::error::InspectError;
use crate::report::ImageReport;

pub fn inspect_dng(bytes: &[u8]) -> Result<ImageReport, InspectError> {
    let mut report = parse::parse(bytes)?;

    // ---- Binary-level info (safe, minimal) ----
    report.binary_info.push(format!(
        "Binary size: {} bytes",
        bytes.len()
    ));

    // TIFF header info (first 8 bytes always meaningful)
    let header_preview = hex_preview(bytes, 8);
    report
        .binary_info
        .push(format!("TIFF header bytes: {}", header_preview));

    Ok(report)
}

/// Hex preview helper (local, small, no allocation explosion)
fn hex_preview(bytes: &[u8], max: usize) -> String {
    bytes.iter()
        .take(max)
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}
