use thiserror::Error;

/// Possible errors returned by `rubagen`
#[derive(Error, Debug)]
pub enum BarcodeError {
    #[error("\"{0}\" character not supported by Code 39")]
    InvalidCharacter(String),

    #[error("Could not read bmp image at {0}")]
    ReadError(String),
}