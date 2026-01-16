use crate::error::InspectError;
use crate::format::ImageFormat;
use crate::report::{ImageReport, MetadataItem};

pub fn parse(bytes: &[u8]) -> Result<ImageReport, InspectError> {
    let little = match &bytes[0..2] {
        b"II" => true,
        b"MM" => false,
        _ => return Err(InspectError::InvalidFormat("invalid TIFF endian")),
    };

    let read_u16 = |b: &[u8]| {
        if little {
            u16::from_le_bytes(b.try_into().unwrap())
        } else {
            u16::from_be_bytes(b.try_into().unwrap())
        }
    };

    let read_u32 = |b: &[u8]| {
        if little {
            u32::from_le_bytes(b.try_into().unwrap())
        } else {
            u32::from_be_bytes(b.try_into().unwrap())
        }
    };

    let ifd_offset = read_u32(&bytes[4..8]) as usize;
    let count = read_u16(&bytes[ifd_offset..ifd_offset + 2]) as usize;

    let mut width = None;
    let mut height = None;
    let mut metadata = Vec::new();

    let mut off = ifd_offset + 2;
    for _ in 0..count {
        let tag = read_u16(&bytes[off..off + 2]);
        let value = read_u32(&bytes[off + 8..off + 12]);

        match tag {
            256 => width = Some(value),
            257 => height = Some(value),
            271 => metadata.push(MetadataItem {
                key: "Camera Make".into(),
                value: value.to_string(),
            }),
            272 => metadata.push(MetadataItem {
                key: "Camera Model".into(),
                value: value.to_string(),
            }),
            _ => {}
        }

        off += 12;
    }

    Ok(ImageReport {
        format: ImageFormat::Dng,
        width,
        height,
        metadata,
        binary_info: vec!["TIFF/DNG container".into()],
    })
}
