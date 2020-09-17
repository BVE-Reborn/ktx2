use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum ReadError {
    IoError(io::Error),
    ParseError(ParseError),
}

impl Error for ReadError {}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ReadError::IoError(e) => write!(f, "Input error: {}", e),
            ReadError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl From<io::Error> for ReadError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<ParseError> for ReadError {
    fn from(e: ParseError) -> Self {
        Self::ParseError(e)
    }
}

#[derive(Debug)]
pub enum ParseError {
    BadIdentifier([u8; 12]),
    BadFormat(u32),
    ZeroTypeSize,
    ZeroWidth,
    ZeroFaceCount,
    UnsupportedFeature(&'static str),
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            ParseError::BadIdentifier(id) => write!(f, "Identifier is wrong: {:?}", id),
            ParseError::BadFormat(i) => write!(f, "Unsoperted format: {:?}", i),
            ParseError::ZeroTypeSize => write!(f, "Type size is zero"),
            ParseError::ZeroWidth => write!(f, "Width is zero"),
            ParseError::ZeroFaceCount => write!(f, "Face count is zero"),
            ParseError::UnsupportedFeature(name) => write!(f, "Loader doesn't support: {}", name),
        }
    }
}
