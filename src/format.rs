use crate::error::InspectError;
use crate::report::ImageReport;

#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Dng,
}

impl std::fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageFormat::Png => write!(f, "PNG"),
            ImageFormat::Jpeg => write!(f, "JPEG"),
            ImageFormat::Dng => write!(f, "DNG"),
        }
    }
}

pub fn inspect(bytes: &[u8]) -> Result<ImageReport, InspectError> {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        crate::png::inspect_png(bytes)
    } else if bytes.len() >= 3 && bytes[0] == 0xFF && bytes[1] == 0xD8 && bytes[2] == 0xFF {
        crate::jpeg::inspect_jpeg(bytes)
    } else if bytes.len() >= 4 && (
        bytes.starts_with(b"II*\0") || bytes.starts_with(b"MM\0*")
    ) {
        crate::dng::inspect_dng(bytes)
    } else {
        Err(InspectError::UnknownFormat)
    }
}
