use crate::error::InspectError;
use crate::format::ImageFormat;
use crate::report::{ImageReport, MetadataItem};

pub fn parse(bytes: &[u8]) -> Result<ImageReport, InspectError> {
    if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Err(InspectError::InvalidFormat("bad PNG signature"));
    }

    let mut offset = 8;
    let mut width = None;
    let mut height = None;
    let mut idat_count = 0;

    let mut metadata = Vec::new();
    let mut binary_info = Vec::new();

    while offset + 8 <= bytes.len() {
        let len = u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap()) as usize;
        let kind = &bytes[offset + 4..offset + 8];
        let data_start = offset + 8;
        let data_end = data_start + len;

        if data_end + 4 > bytes.len() {
            break;
        }

        match kind {
            b"IHDR" => {
                width = Some(u32::from_be_bytes(bytes[data_start..data_start + 4].try_into().unwrap()));
                height = Some(u32::from_be_bytes(bytes[data_start + 4..data_start + 8].try_into().unwrap()));

                metadata.push(MetadataItem {
                    key: "Color Type".into(),
                    value: bytes[data_start + 9].to_string(),
                });
                metadata.push(MetadataItem {
                    key: "Bit Depth".into(),
                    value: bytes[data_start + 8].to_string(),
                });
            }
            b"tEXt" => {
                if let Some(pos) = bytes[data_start..data_end].iter().position(|&b| b == 0) {
                    let key = String::from_utf8_lossy(&bytes[data_start..data_start + pos]).to_string();
                    let value = String::from_utf8_lossy(&bytes[data_start + pos + 1..data_end]).to_string();
                    metadata.push(MetadataItem { key, value });
                }
            }
            b"IDAT" => idat_count += 1,
            b"IEND" => {
                binary_info.push("IEND chunk present".into());
                break;
            }
            _ => {}
        }

        offset = data_end + 4;
    }

    binary_info.push(format!("IDAT chunks: {}", idat_count));

    Ok(ImageReport {
        format: ImageFormat::Png,
        width,
        height,
        metadata,
        binary_info,
    })
}
