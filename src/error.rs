use thiserror::Error;

#[derive(Error, Debug)]
pub enum InspectError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unknown or unsupported image format")]
    UnknownFormat,

    #[error("invalid image format: {0}")]
    InvalidFormat(&'static str),
}
