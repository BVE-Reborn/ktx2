use core::fmt;
#[cfg(feature = "std")]
use std::error::Error;

/// Error, that happend when data doesn't satisfy expected parameters.
#[derive(Debug)]
pub enum ParseError {
    /// Unexpected texture identifier.
    BadIdentifier([u8; 12]),
    /// Width of texture is zero.
    ZeroWidth,
    /// Face count of texture is zero.
    ZeroFaceCount,
    /// Unexpected end of buffer
    UnexpectedEnd,
}

#[cfg(feature = "std")]
impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::BadIdentifier(id) => write!(f, "Identifier is wrong: {:?}", id),
            ParseError::ZeroWidth => write!(f, "Width is zero"),
            ParseError::ZeroFaceCount => write!(f, "Face count is zero"),
            ParseError::UnexpectedEnd => f.pad("unexpected end of buffer"),
        }
    }
}
