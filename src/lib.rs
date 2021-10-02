//! [KTX2] decoding. Start with a [`Reader`].
//!
//! [KTX2]: https://github.khronos.org/KTX-Specification/

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
            return Err(ParseError::BadMagic(
                input.as_ref()[0..KTX2_MAGIC.len()].try_into().unwrap(),
            ));
        }
        let header = input.as_ref()[0..48].try_into().unwrap();
        Header::from_bytes(header).validate()?;

        let result = Self { input };
        result.level_index()?; // Check index integrity
        result.read_data()?; // Check level integrity
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

    /// Access texture data
    pub fn data(&self) -> &[u8] {
        self.read_data().unwrap()
    }

    fn read_data(&self) -> ParseResult<&[u8]> {
        let data_len_bytes = self.data_len_bytes() as usize;

        let data_start_byte = self.first_level_offset_bytes() as usize;
        let data_end_byte = data_start_byte + data_len_bytes;
        Ok(&self
            .input
            .as_ref()
            .get(data_start_byte..data_end_byte)
            .ok_or(ParseError::UnexpectedEnd)?)
    }

    /// Container-level metadata
    pub fn header(&self) -> Header {
        let bytes = self.input.as_ref()[0..48].try_into().unwrap();
        Header::from_bytes(bytes)
    }

    /// Iterator over the texture's mip levels
    pub fn levels(&self) -> impl ExactSizeIterator<Item = Level> + '_ {
        let base_offset = self.first_level_offset_bytes();
        self.level_index()
            .unwrap()
            .enumerate()
            .map(move |(i, level)| self.level_from_level_index(i, level.offset - base_offset))
    }

    /// Start of texture data offset in bytes
    fn first_level_offset_bytes(&self) -> u64 {
        self.level_index()
            .unwrap()
            .map(|l| l.offset)
            .min()
            .expect("No levels got, but read some on constructing")
    }

    /// Last (by data offset) level in texture data.
    fn last_level(&self) -> LevelIndex {
        self.level_index()
            .unwrap()
            .max_by_key(|l| l.offset)
            .expect("No levels got, but read some on constructing")
    }

    /// Full length of texture data
    fn data_len_bytes(&self) -> u64 {
        let start_offset = self.first_level_offset_bytes();
        let last_level = self.last_level();
        last_level.offset + last_level.uncompressed_length_bytes - start_offset
    }

    /// Crates level from level info.
    fn level_from_level_index(&self, i: usize, offset: u64) -> Level {
        let header = self.header();
        Level {
            level: i as u32,
            layer_count: header.layer_count.max(1) * header.face_count,
            offset_bytes: offset,
            width: Self::level_size(header.pixel_width, i as u32),
            height: Self::level_size(header.pixel_height, i as u32),
            depth: Self::level_size(header.pixel_depth, i as u32),
        }
    }

    /// Size in pixels of `level`, with `base` size.
    fn level_size(base: u32, level: u32) -> u32 {
        (base >> level).max(1)
    }
}

/// Identifier, expected in start of input texture data.
const KTX2_MAGIC: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];

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
    fn from_bytes(data: HeadBytes) -> Self {
        Self {
            format: Format::new(u32::from_le_bytes(data[12..16].try_into().unwrap())),
            type_size: u32::from_le_bytes(data[16..20].try_into().unwrap()),
            pixel_width: u32::from_le_bytes(data[20..24].try_into().unwrap()),
            pixel_height: u32::from_le_bytes(data[24..28].try_into().unwrap()),
            pixel_depth: u32::from_le_bytes(data[28..32].try_into().unwrap()),
            layer_count: u32::from_le_bytes(data[32..36].try_into().unwrap()),
            face_count: u32::from_le_bytes(data[36..40].try_into().unwrap()),
            level_count: u32::from_le_bytes(data[40..44].try_into().unwrap()),
            supercompression_scheme: SupercompressionScheme::new(u32::from_le_bytes(
                data[44..48].try_into().unwrap(),
            )),
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

/// Array, that stores data of start of texture.
type HeadBytes<'a> = &'a [u8; 48];

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

/// Metadata describing a particular mipmap level
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Level {
    pub level: u32,
    pub layer_count: u32,
    pub offset_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
