//! Parser for the [ktx2](https://github.khronos.org/KTX-Specification/) texture container format.
//!
//! ## Features
//! - [x] Async reading
//! - [x] Parsing
//! - [x] Validating
//! - [ ] [Data format description](https://github.khronos.org/KTX-Specification/#_data_format_descriptor)
//! - [ ] [Key/value data](https://github.khronos.org/KTX-Specification/#_keyvalue_data)
//
//! ## Example
//! ```rust
//! // Crate instance of reader. This validates the header
//! # let file = include_bytes!("../data/test_tex.ktx2");
//! let mut reader = ktx2::Reader::new(file).expect("Can't create reader"); // Crate instance of reader.
//!
//! // Get general texture information.
//! let header = reader.header();
//!
//! // Read iterator over slices of each mipmap level.
//! let levels = reader.levels().collect::<Vec<_>>();
//! # let _ = (header, levels);
//! ```

#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod enums;
mod error;

pub use crate::enums::{Format, SupercompressionScheme};
pub use crate::error::ParseError;

use core::convert::TryInto;

/// Decodes KTX2 texture data
pub struct Reader<Data: AsRef<[u8]>> {
    input: Data,
}

impl<Data: AsRef<[u8]>> Reader<Data> {
    /// Decode KTX2 data from `input`
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < 48 {
            return Err(ParseError::UnexpectedEnd);
        }
        if !input.as_ref().starts_with(&KTX2_MAGIC) {
            return Err(ParseError::BadMagic);
        }
        let header = input.as_ref()[0..48].try_into().unwrap();
        Header::from_bytes(header).validate()?;

        let result = Self { input };
        result.level_index()?; // Check index integrity

        // Check level data integrity
        let last = result.last_level();
        if last.offset + last.length_bytes > result.input.as_ref().len() as u64 {
            return Err(ParseError::UnexpectedEnd);
        }

        Ok(result)
    }

    fn level_index(&self) -> ParseResult<impl ExactSizeIterator<Item = LevelIndex> + '_> {
        const LEVEL_INDEX_START_BYTE: usize = 80;
        const LEVEL_INDEX_BYTE_LEN: usize = 24;
        let level_count = self.header().level_count.max(1) as usize;

        let level_index_end_byte = LEVEL_INDEX_START_BYTE + level_count * LEVEL_INDEX_BYTE_LEN;
        let level_index_bytes = self
            .input
            .as_ref()
            .get(LEVEL_INDEX_START_BYTE..level_index_end_byte)
            .ok_or(ParseError::UnexpectedEnd)?;
        Ok(level_index_bytes
            .chunks_exact(LEVEL_INDEX_BYTE_LEN)
            .map(LevelIndex::from_bytes))
    }

    /// Access underlying raw bytes
    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }

    /// Container-level metadata
    pub fn header(&self) -> Header {
        let bytes = self.input.as_ref()[0..48].try_into().unwrap();
        Header::from_bytes(bytes)
    }

    /// Iterator over the texture's mip levels
    pub fn levels(&self) -> impl ExactSizeIterator<Item = &[u8]> + '_ {
        self.level_index()
            .unwrap()
            .map(move |level| &self.input.as_ref()[level.offset as usize..(level.offset + level.length_bytes) as usize])
    }

    /// Last (by data offset) level in texture data.
    fn last_level(&self) -> LevelIndex {
        self.level_index()
            .unwrap()
            .max_by_key(|l| l.offset)
            .expect("No levels got, but read some on constructing")
    }
}

/// Identifier, expected in start of input texture data.
const KTX2_MAGIC: [u8; 12] = [0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A];

/// Result of parsing data operation.
type ParseResult<T> = Result<T, ParseError>;

/// Container-level metadata
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Header {
    pub format: Option<Format>,
    pub type_size: u32,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub pixel_depth: u32,
    pub layer_count: u32,
    pub face_count: u32,
    pub level_count: u32,
    pub supercompression_scheme: Option<SupercompressionScheme>,
}

impl Header {
    fn from_bytes(data: &[u8; 48]) -> Self {
        Self {
            format: Format::new(u32::from_le_bytes(data[12..16].try_into().unwrap())),
            type_size: u32::from_le_bytes(data[16..20].try_into().unwrap()),
            pixel_width: u32::from_le_bytes(data[20..24].try_into().unwrap()),
            pixel_height: u32::from_le_bytes(data[24..28].try_into().unwrap()),
            pixel_depth: u32::from_le_bytes(data[28..32].try_into().unwrap()),
            layer_count: u32::from_le_bytes(data[32..36].try_into().unwrap()),
            face_count: u32::from_le_bytes(data[36..40].try_into().unwrap()),
            level_count: u32::from_le_bytes(data[40..44].try_into().unwrap()),
            supercompression_scheme: SupercompressionScheme::new(u32::from_le_bytes(data[44..48].try_into().unwrap())),
        }
    }

    fn validate(&self) -> ParseResult<()> {
        if self.pixel_width == 0 {
            return Err(ParseError::ZeroWidth);
        }
        if self.face_count == 0 {
            return Err(ParseError::ZeroFaceCount);
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct LevelIndex {
    offset: u64,
    length_bytes: u64,
    uncompressed_length_bytes: u64,
}

impl LevelIndex {
    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            offset: u64::from_le_bytes(data[0..8].try_into().unwrap()),
            length_bytes: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            uncompressed_length_bytes: u64::from_le_bytes(data[16..24].try_into().unwrap()),
        }
    }
}
