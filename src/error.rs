use core::fmt;
#[cfg(feature = "std")]
use std::error::Error;

/// Error, that happend when data doesn't satisfy expected parameters.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseError {
    /// Unexpected magic numbers
    BadMagic,
    /// Zero pixel width
    ZeroWidth,
    /// Zero face count
    ZeroFaceCount,
    /// Unexpected end of buffer
    UnexpectedEnd,
}

#[cfg(feature = "std")]
impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::BadMagic => f.pad("unexpected magic numbers"),
            ParseError::ZeroWidth => f.pad("zero pixel width"),
            ParseError::ZeroFaceCount => f.pad("zero face count"),
            ParseError::UnexpectedEnd => f.pad("unexpected end of buffer"),
        }
    }
}
