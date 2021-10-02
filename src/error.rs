use std::error::Error;
use std::fmt;

/// Error, that happend when data doesn't satisfy expected parameters.
#[derive(Debug)]
pub enum ParseError {
    /// Unexpected texture identifier.
    BadIdentifier([u8; 12]),
    /// Specified format is not supported.
    BadFormat(u32),
    /// Width of texture is zero.
    ZeroWidth,
    /// Face count of texture is zero.
    ZeroFaceCount,
    /// Found unsupported feature.
    UnsupportedFeature(&'static str),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::BadIdentifier(id) => write!(f, "Identifier is wrong: {:?}", id),
            ParseError::BadFormat(i) => write!(f, "Unsoperted format: {:?}", i),
            ParseError::ZeroWidth => write!(f, "Width is zero"),
            ParseError::ZeroFaceCount => write!(f, "Face count is zero"),
            ParseError::UnsupportedFeature(name) => write!(f, "Loader doesn't support: {}", name),
        }
    }
}
