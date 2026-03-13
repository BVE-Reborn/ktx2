//! Parser for the [ktx2](https://github.khronos.org/KTX-Specification/ktxspec.v2.html) texture container format.
//!
//! ## Features
//! - [x] Async reading
//! - [x] Parsing
//! - [x] Validating
//! - [x] [Data format description](https://github.khronos.org/KTX-Specification/ktxspec.v2.html#_data_format_descriptor)
//! - [x] [Key/value data](https://github.khronos.org/KTX-Specification/ktxspec.v2.html#_keyvalue_data)
//!
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
//!
//! ## MSRV
//!
//! The minimum supported Rust version is 1.56. MSRV bumps are treated as breaking changes.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub mod dfd;
mod enums;
mod error;

pub use crate::{
    enums::{Format, SupercompressionScheme},
    error::ParseError,
};

use core::convert::TryInto;

/// Decodes KTX2 texture data
pub struct Reader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
}

impl<Data: AsRef<[u8]>> Reader<Data> {
    /// Decode KTX2 data from `input`
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < Header::LENGTH {
            return Err(ParseError::UnexpectedEnd);
        }
        let header_data = input.as_ref()[0..Header::LENGTH].try_into().unwrap();
        let header = Header::from_bytes(header_data)?;

        // Check DFD bounds
        let dfd_start = header
            .index
            .dfd_byte_offset
            .checked_add(4)
            .ok_or(ParseError::UnexpectedEnd)?;
        let dfd_end = header
            .index
            .dfd_byte_offset
            .checked_add(header.index.dfd_byte_length)
            .ok_or(ParseError::UnexpectedEnd)?;
        if dfd_end < dfd_start || dfd_end as usize >= input.as_ref().len() {
            return Err(ParseError::UnexpectedEnd);
        }

        // Check SGD bounds
        if header
            .index
            .sgd_byte_offset
            .checked_add(header.index.sgd_byte_length)
            .ok_or(ParseError::UnexpectedEnd)?
            >= input.as_ref().len() as u64
        {
            return Err(ParseError::UnexpectedEnd);
        }

        // Check KVD bounds
        if header
            .index
            .kvd_byte_offset
            .checked_add(header.index.kvd_byte_length)
            .ok_or(ParseError::UnexpectedEnd)? as usize
            >= input.as_ref().len()
        {
            return Err(ParseError::UnexpectedEnd);
        }

        let result = Self { input, header };
        let index = result.level_index()?; // Check index integrity

        // Check level data bounds
        for level in index {
            if level
                .byte_offset
                .checked_add(level.byte_length)
                .ok_or(ParseError::UnexpectedEnd)?
                > result.input.as_ref().len() as u64
            {
                return Err(ParseError::UnexpectedEnd);
            }
        }

        Ok(result)
    }

    fn level_index(&self) -> ParseResult<impl ExactSizeIterator<Item = LevelIndex> + '_> {
        let level_count = self.header().level_count.max(1) as usize;

        let level_index_end_byte = Header::LENGTH
            .checked_add(
                level_count
                    .checked_mul(LevelIndex::LENGTH)
                    .ok_or(ParseError::UnexpectedEnd)?,
            )
            .ok_or(ParseError::UnexpectedEnd)?;
        let level_index_bytes = self
            .input
            .as_ref()
            .get(Header::LENGTH..level_index_end_byte)
            .ok_or(ParseError::UnexpectedEnd)?;
        Ok(level_index_bytes.chunks_exact(LevelIndex::LENGTH).map(|data| {
            let level_data = data.try_into().unwrap();
            LevelIndex::from_bytes(&level_data)
        }))
    }

    /// Access underlying raw bytes
    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }

    /// Container-level metadata
    pub fn header(&self) -> Header {
        self.header
    }

    /// Iterator over the texture's mip levels
    pub fn levels(&self) -> impl ExactSizeIterator<Item = Level> + '_ {
        self.level_index().unwrap().map(move |level| Level {
            // Bounds-checking previously performed in `new`
            data: &self.input.as_ref()[level.byte_offset as usize..(level.byte_offset + level.byte_length) as usize],
            uncompressed_byte_length: level.uncompressed_byte_length,
        })
    }

    pub fn supercompression_global_data(&self) -> &[u8] {
        let header = self.header();
        let start = header.index.sgd_byte_offset as usize;
        // Bounds-checking previously performed in `new`
        let end = (header.index.sgd_byte_offset + header.index.sgd_byte_length) as usize;
        &self.input.as_ref()[start..end]
    }

    pub fn dfd_blocks(&self) -> impl Iterator<Item = dfd::Block> {
        let header = self.header();
        let start = header.index.dfd_byte_offset as usize;
        // Bounds-checking previously performed in `new`
        let end = (header.index.dfd_byte_offset + header.index.dfd_byte_length) as usize;
        DfdBlockIterator {
            // start + 4 to skip the data format descriptors total length
            data: &self.input.as_ref()[start + 4..end],
        }
    }

    /// Iterator over the key-value pairs
    pub fn key_value_data(&self) -> KeyValueDataIterator {
        let header = self.header();

        let start = header.index.kvd_byte_offset as usize;
        // Bounds-checking previously performed in `new`
        let end = (header.index.kvd_byte_offset + header.index.kvd_byte_length) as usize;

        KeyValueDataIterator::new(&self.input.as_ref()[start..end])
    }
}

pub(crate) struct DfdBlockIterator<'data> {
    data: &'data [u8],
}

impl<'data> Iterator for DfdBlockIterator<'data> {
    type Item = dfd::Block<'data>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() < dfd::BlockHeader::LENGTH {
            return None;
        }
        dfd::BlockHeader::parse(&self.data[..dfd::BlockHeader::LENGTH]).map_or(
            None,
            |(header, descriptor_block_size)| {
                if descriptor_block_size == 0 || self.data.len() < descriptor_block_size {
                    return None;
                }
                let data = &self.data[dfd::BlockHeader::LENGTH..descriptor_block_size];
                self.data = &self.data[descriptor_block_size..];
                Some(dfd::Block { header, data })
            },
        )
    }
}

/// An iterator that parses the key-value pairs in the KTX2 file.
pub struct KeyValueDataIterator<'data> {
    data: &'data [u8],
}

impl<'data> KeyValueDataIterator<'data> {
    /// Create a new iterator from the key-value data section of the KTX2 file.
    ///
    /// From the start of the file, this is a slice between [`Index::kvd_byte_offset`]
    /// and [`Index::kvd_byte_offset`] + [`Index::kvd_byte_length`].
    pub fn new(data: &'data [u8]) -> Self {
        Self { data }
    }
}

impl<'data> Iterator for KeyValueDataIterator<'data> {
    type Item = (&'data str, &'data [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        let mut offset = 0;

        loop {
            let length = bytes_to_u32(self.data, &mut offset).ok()?;

            let start_offset = offset;

            offset = offset.checked_add(length as usize)?;

            let end_offset = offset;

            // Ensure that we're 4-byte aligned
            if offset % 4 != 0 {
                offset += 4 - (offset % 4);
            }

            let key_and_value = match self.data.get(start_offset..end_offset) {
                Some(key_and_value) => key_and_value,
                None => continue,
            };

            // The key is terminated with a NUL character.
            let key_end_index = match key_and_value.iter().position(|&c| c == b'\0') {
                Some(index) => index,
                None => continue,
            };

            let key = &key_and_value[..key_end_index];
            let value = &key_and_value[key_end_index + 1..];

            let key = match core::str::from_utf8(key) {
                Ok(key) => key,
                Err(_) => continue,
            };

            self.data = self.data.get(offset..).unwrap_or_default();

            return Some((key, value));
        }
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
    pub index: Index,
}

/// An index giving the byte offsets from the start of the file and byte sizes of the various sections of the KTX2 file.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Index {
    pub dfd_byte_offset: u32,
    pub dfd_byte_length: u32,
    pub kvd_byte_offset: u32,
    pub kvd_byte_length: u32,
    pub sgd_byte_offset: u64,
    pub sgd_byte_length: u64,
}

impl Header {
    pub const LENGTH: usize = 80;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> ParseResult<Self> {
        if !data.starts_with(&KTX2_MAGIC) {
            return Err(ParseError::BadMagic);
        }

        let header = Self {
            format: Format::new(u32::from_le_bytes(data[12..16].try_into().unwrap())),
            type_size: u32::from_le_bytes(data[16..20].try_into().unwrap()),
            pixel_width: u32::from_le_bytes(data[20..24].try_into().unwrap()),
            pixel_height: u32::from_le_bytes(data[24..28].try_into().unwrap()),
            pixel_depth: u32::from_le_bytes(data[28..32].try_into().unwrap()),
            layer_count: u32::from_le_bytes(data[32..36].try_into().unwrap()),
            face_count: u32::from_le_bytes(data[36..40].try_into().unwrap()),
            level_count: u32::from_le_bytes(data[40..44].try_into().unwrap()),
            supercompression_scheme: SupercompressionScheme::new(u32::from_le_bytes(data[44..48].try_into().unwrap())),
            index: Index {
                dfd_byte_offset: u32::from_le_bytes(data[48..52].try_into().unwrap()),
                dfd_byte_length: u32::from_le_bytes(data[52..56].try_into().unwrap()),
                kvd_byte_offset: u32::from_le_bytes(data[56..60].try_into().unwrap()),
                kvd_byte_length: u32::from_le_bytes(data[60..64].try_into().unwrap()),
                sgd_byte_offset: u64::from_le_bytes(data[64..72].try_into().unwrap()),
                sgd_byte_length: u64::from_le_bytes(data[72..80].try_into().unwrap()),
            },
        };

        if header.pixel_width == 0 {
            return Err(ParseError::ZeroWidth);
        }
        if header.face_count == 0 {
            return Err(ParseError::ZeroFaceCount);
        }

        Ok(header)
    }

    pub fn as_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0; Self::LENGTH];

        let format = self.format.map(|format| format.value()).unwrap_or(0);
        let supercompression_scheme = self.supercompression_scheme.map(|scheme| scheme.value()).unwrap_or(0);

        bytes[0..12].copy_from_slice(&KTX2_MAGIC);
        bytes[12..16].copy_from_slice(&format.to_le_bytes()[..]);
        bytes[16..20].copy_from_slice(&self.type_size.to_le_bytes()[..]);
        bytes[20..24].copy_from_slice(&self.pixel_width.to_le_bytes()[..]);
        bytes[24..28].copy_from_slice(&self.pixel_height.to_le_bytes()[..]);
        bytes[28..32].copy_from_slice(&self.pixel_depth.to_le_bytes()[..]);
        bytes[32..36].copy_from_slice(&self.layer_count.to_le_bytes()[..]);
        bytes[36..40].copy_from_slice(&self.face_count.to_le_bytes()[..]);
        bytes[40..44].copy_from_slice(&self.level_count.to_le_bytes()[..]);
        bytes[44..48].copy_from_slice(&supercompression_scheme.to_le_bytes()[..]);
        bytes[48..52].copy_from_slice(&self.index.dfd_byte_offset.to_le_bytes()[..]);
        bytes[52..56].copy_from_slice(&self.index.dfd_byte_length.to_le_bytes()[..]);
        bytes[56..60].copy_from_slice(&self.index.kvd_byte_offset.to_le_bytes()[..]);
        bytes[60..64].copy_from_slice(&self.index.kvd_byte_length.to_le_bytes()[..]);
        bytes[64..72].copy_from_slice(&self.index.sgd_byte_offset.to_le_bytes()[..]);
        bytes[72..80].copy_from_slice(&self.index.sgd_byte_length.to_le_bytes()[..]);

        bytes
    }
}

pub struct Level<'a> {
    pub data: &'a [u8],
    pub uncompressed_byte_length: u64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LevelIndex {
    pub byte_offset: u64,
    pub byte_length: u64,
    pub uncompressed_byte_length: u64,
}

impl LevelIndex {
    pub const LENGTH: usize = 24;

    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        Self {
            byte_offset: u64::from_le_bytes(data[0..8].try_into().unwrap()),
            byte_length: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            uncompressed_byte_length: u64::from_le_bytes(data[16..24].try_into().unwrap()),
        }
    }

    pub fn as_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0; Self::LENGTH];

        bytes[0..8].copy_from_slice(&self.byte_offset.to_le_bytes()[..]);
        bytes[8..16].copy_from_slice(&self.byte_length.to_le_bytes()[..]);
        bytes[16..24].copy_from_slice(&self.uncompressed_byte_length.to_le_bytes()[..]);

        bytes
    }
}

fn read_bytes<const N: usize>(bytes: &[u8], offset: &mut usize) -> Result<[u8; N], ParseError> {
    let v = bytes
        .get(*offset..*offset + N)
        .ok_or(ParseError::UnexpectedEnd)?
        .try_into()
        .unwrap();
    *offset += N;
    Ok(v)
}

fn read_u16(bytes: &[u8], offset: &mut usize) -> Result<u16, ParseError> {
    let v = u16::from_le_bytes(read_bytes(bytes, offset)?);
    Ok(v)
}

fn bytes_to_u32(bytes: &[u8], offset: &mut usize) -> Result<u32, ParseError> {
    let v = u32::from_le_bytes(
        bytes
            .get(*offset..*offset + 4)
            .ok_or(ParseError::UnexpectedEnd)?
            .try_into()
            .unwrap(),
    );
    *offset += 4;
    Ok(v)
}

fn shift_and_mask_lower(shift: u32, mask: u32, value: u32) -> u32 {
    (value >> shift) & ((1 << mask) - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::octal_escapes)]
    fn test_malformed_key_value_data_handling() {
        let data = [
            &0_u32.to_le_bytes()[..],
            // Regular key-value pair
            &7_u32.to_le_bytes()[..],
            b"xyz\0123 ",
            // Malformed key-value pair with missing NUL byte
            &11_u32.to_le_bytes()[..],
            b"abcdefghi!! ",
            // Regular key-value pair again
            &7_u32.to_le_bytes()[..],
            b"abc\0987",
            &1000_u32.to_le_bytes()[..],
            &[1; 1000],
            &u32::MAX.to_le_bytes()[..],
        ];

        let mut iterator = KeyValueDataIterator { data: &data.concat() };

        assert_eq!(iterator.next(), Some(("xyz", &b"123"[..])));
        assert_eq!(iterator.next(), Some(("abc", &b"987"[..])));
        assert_eq!(iterator.next(), None);
    }
}
