//! Reading, validating and parsing of [`KTX v.2`] files.  
//! **Currently SUPER COMPRESSION is NOT supported.**
//!
//! [`KTX v.2`]: https://github.khronos.org/KTX-Specification/
mod error;
mod format;

pub use crate::error::ParseError;
pub use crate::format::Format;

use std::convert::TryInto;

/// Struct to read [`KTX v.2`] files.  
///
/// [`KTX v.2`]: https://github.khronos.org/KTX-Specification/
pub struct Reader<Data: AsRef<[u8]>> {
    input: Data,
    head: Header,
    levels_index: Vec<LevelIndex>,
}
impl<Data: AsRef<[u8]>> Reader<Data> {
    /// Create new instance of Reader.  
    /// Asyncroniosly reads and tries to parse data from `input`.
    /// # Errors
    /// If parsing fails, returns [`ParseError`].
    pub fn new(input: Data) -> ParseResult<Self> {
        let head = Self::read_head(input.as_ref())?;
        let levels_index = Self::read_level_index(input.as_ref(), &head)?;
        Ok(Self {
            input,
            head,
            levels_index,
        })
    }

    /// Reads and tries to parse header of texture.  
    fn read_head(input: &[u8]) -> ParseResult<Header> {
        let head_bytes: HeadBytes = input[0..48].try_into().unwrap();
        Self::test_identifier(head_bytes)?;
        Ok(Header::from_bytes(head_bytes)?)
    }

    /// Reads and tries to parse level index of texture.  
    ///
    /// [Level index](https://github.khronos.org/KTX-Specification/#_level_index) is a description of texture data layout.
    fn read_level_index(input: &[u8], head: &Header) -> ParseResult<Vec<LevelIndex>> {
        const LEVEL_INDEX_START_BYTE: usize = 80;
        const LEVEL_INDEX_BYTE_LEN: usize = 24;
        let level_count = head.level_count.max(1);

        let level_index_end_byte =
            LEVEL_INDEX_START_BYTE + level_count as usize * LEVEL_INDEX_BYTE_LEN;
        let level_index_bytes = &input[LEVEL_INDEX_START_BYTE..level_index_end_byte];

        let mut infos = Vec::with_capacity(level_count as usize);
        for level_index in 0..level_count {
            let start_byte = level_index as usize * LEVEL_INDEX_BYTE_LEN;
            let end_byte = start_byte + LEVEL_INDEX_BYTE_LEN as usize;
            infos.push(LevelIndex::from_bytes(
                &level_index_bytes[start_byte..end_byte],
            ))
        }
        Ok(infos)
    }

    pub fn read_data(&self) -> ParseResult<&[u8]> {
        let data_len_bytes = self.data_len_bytes() as usize;

        let data_start_byte = self.first_level_offset_bytes() as usize;
        let data_end_byte = data_start_byte + data_len_bytes;
        Ok(&self.input.as_ref()[data_start_byte..data_end_byte])
    }

    /// Tests first 12 bytes of input. If identifier is wrong,
    /// returns [`ParseError`]
    /// with [`ParseError::BadIdentifier`].
    fn test_identifier(head_bytes: HeadBytes<'_>) -> ParseResult<()> {
        let ident_bytes: &[u8; 12] = head_bytes[0..12].try_into().unwrap();
        if ident_bytes == &KTX2_IDENTIFIER {
            return Ok(());
        }
        Err(ParseError::BadIdentifier(*ident_bytes))
    }

    /// Returns [`Header`] of texture.
    pub fn header(&self) -> Header {
        self.head
    }

    /// Iterator over the texture's mip levels.
    pub fn levels(&self) -> impl ExactSizeIterator<Item = Level> + '_ {
        let base_offset = self.first_level_offset_bytes();
        self.levels_index
            .iter()
            .enumerate()
            .map(move |(i, level)| self.level_from_level_index(i, level.offset - base_offset))
    }

    /// Start of texture data oofset in bytes.
    fn first_level_offset_bytes(&self) -> u64 {
        self.levels_index
            .iter()
            .map(|l| l.offset)
            .min()
            .expect("No levels got, but read some on constructing")
    }

    /// Last (by data offset) level in texture data.
    fn last_level(&self) -> LevelIndex {
        *self
            .levels_index
            .iter()
            .max_by_key(|l| l.offset)
            .expect("No levels got, but read some on constructing")
    }

    /// Full length of texture data.
    pub fn data_len_bytes(&self) -> u64 {
        let start_offset = self.first_level_offset_bytes();
        let last_level = self.last_level();
        last_level.offset + last_level.uncompressed_length_bytes - start_offset
    }

    /// Crates level from level info.
    fn level_from_level_index(&self, i: usize, offset: u64) -> Level {
        Level {
            level: i as u32,
            layer_count: self.head.layer_count.max(1) * self.head.face_count,
            offset_bytes: offset,
            width: Self::level_size(self.head.pixel_width, i as u32),
            height: Self::level_size(self.head.pixel_height, i as u32),
            depth: Self::level_size(self.head.pixel_depth, i as u32),
        }
    }

    /// Size in pixels of `level`, with `base` size.
    fn level_size(base: u32, level: u32) -> u32 {
        (base >> level).max(1)
    }
}

/// Identifier, expected in start of input texture data.
static KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];

/// Result of parsing data operation.
pub type ParseResult<T> = Result<T, ParseError>;

/// Header of texture. Contains general information.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Header {
    pub format: Format,
    pub type_size: u32,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub pixel_depth: u32,
    pub layer_count: u32,
    pub face_count: u32,
    pub level_count: u32,
    pub supercompression_scheme: u32,
}

impl Header {
    /// Crates Header from bytes array.
    pub fn from_bytes(data: HeadBytes) -> ParseResult<Self> {
        let format_id = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let format = format_id.try_into()?;

        Ok(Self {
            format,
            type_size: u32::from_le_bytes(data[16..20].try_into().unwrap()),
            pixel_width: Self::parse_pixel_width(&data[20..24])?,
            pixel_height: u32::from_le_bytes(data[24..28].try_into().unwrap()),
            pixel_depth: u32::from_le_bytes(data[28..32].try_into().unwrap()),
            layer_count: u32::from_le_bytes(data[32..36].try_into().unwrap()),
            face_count: Self::parse_face_count(&data[36..40])?,
            level_count: u32::from_le_bytes(data[40..44].try_into().unwrap()),
            supercompression_scheme: Self::parse_supercompression_scheme(&data[44..48])?,
        })
    }

    fn parse_pixel_width(data: &[u8]) -> ParseResult<u32> {
        let result = u32::from_le_bytes(data[0..4].try_into().unwrap());
        match result {
            0 => Err(ParseError::ZeroWidth),
            _ => Ok(result),
        }
    }

    fn parse_face_count(data: &[u8]) -> ParseResult<u32> {
        let result = u32::from_le_bytes(data[0..4].try_into().unwrap());
        match result {
            0 => Err(ParseError::ZeroFaceCount),
            _ => Ok(result),
        }
    }

    fn parse_supercompression_scheme(data: &[u8]) -> ParseResult<u32> {
        let result = u32::from_le_bytes(data[0..4].try_into().unwrap());
        match result {
            0 => Ok(0),
            _ => Err(ParseError::UnsupportedFeature("supercompression scheme")),
        }
    }
}

/// Array, that stores data of start of texture.
type HeadBytes<'a> = &'a [u8; 48];

/// Struct, that contains size and offset information about levels.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct LevelIndex {
    pub offset: u64,
    pub length_bytes: u64,
    pub uncompressed_length_bytes: u64,
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
