use crate::error::InspectError;
use crate::format::ImageFormat;
use crate::report::{ImageReport, MetadataItem};

pub fn parse(bytes: &[u8]) -> Result<ImageReport, InspectError> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return Err(InspectError::InvalidFormat("missing SOI"));
    }

    let mut offset = 2;
    let mut width = None;
    let mut height = None;
    let mut metadata = Vec::new();
    let mut binary_info = Vec::new();

    while offset + 4 <= bytes.len() {
        if bytes[offset] != 0xFF {
            break;
        }

        let marker = bytes[offset + 1];
        offset += 2;

        if marker == 0xD9 {
            binary_info.push("EOI marker present".into());
            break;
        }

        let len = u16::from_be_bytes(bytes[offset..offset + 2].try_into().unwrap()) as usize;

        if marker == 0xC0 || marker == 0xC2 {
            height = Some(u16::from_be_bytes(bytes[offset + 3..offset + 5].try_into().unwrap()) as u32);
            width = Some(u16::from_be_bytes(bytes[offset + 5..offset + 7].try_into().unwrap()) as u32);

            metadata.push(MetadataItem {
                key: "Encoding".into(),
                value: if marker == 0xC2 {
                    "Progressive DCT".into()
                } else {
                    "Baseline DCT".into()
                },
            });
        }

        if marker == 0xE1 {
            metadata.push(MetadataItem {
                key: "EXIF".into(),
                value: "EXIF segment present".into(),
            });
        }

        offset += len;
    }

    Ok(ImageReport {
        format: ImageFormat::Jpeg,
        width,
        height,
        metadata,
        binary_info,
    })
}
