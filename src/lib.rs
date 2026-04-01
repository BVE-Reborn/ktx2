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

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod dfd;
mod enums;
mod error;
mod util;

pub use crate::{
    enums::{ColorModel, ColorPrimaries, Format, SupercompressionScheme, TransferFunction},
    error::ParseError,
};

use alloc::vec::Vec;
use core::convert::TryInto;

/// Parses and validates a KTX2 texture container from an in-memory buffer.
///
/// All validation (magic bytes, bounds checks, DFD integrity, level index) is
/// performed eagerly in [`Reader::new`]. Subsequent accessors are infallible.
///
/// `Data` can be any type that derefs to `[u8]` — `&[u8]`, `Vec<u8>`,
/// `Arc<[u8]>`, etc.
pub struct Reader<Data: AsRef<[u8]>> {
    input: Data,
    header: Header,
    dfd_blocks: Vec<dfd::Block>,
}

impl<Data: AsRef<[u8]>> Reader<Data> {
    /// Parse and validate a KTX2 buffer.
    ///
    /// Validates the header magic, all section bounds, the DFD, and the level
    /// index. Returns [`ParseError`] on any structural problem.
    pub fn new(input: Data) -> Result<Self, ParseError> {
        let header_data = input
            .as_ref()
            .get(0..Header::LENGTH)
            .ok_or(ParseError::UnexpectedEnd)?
            .try_into()
            .unwrap();
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

        let mut result = Self {
            input,
            header,
            // 1 is the most likely length, as 99.99% of KTX2 files have exactly 1 DFD block.
            dfd_blocks: Vec::with_capacity(1),
        };
        result.parse_dfd()?;
        // Creating the iterator validates the integrity of the level index.
        let index = result.level_index()?;

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

    /// Eagerly parses all DFD blocks from the DFD section into `self.dfd_blocks`.
    /// Fails if the section contains a partial or malformed block.
    fn parse_dfd(&mut self) -> ParseResult<()> {
        let dfd_start = self.header.index.dfd_byte_offset as usize;
        let dfd_end = (self.header.index.dfd_byte_offset + self.header.index.dfd_byte_length) as usize;
        // Skip the 4-byte DFD total length field
        let mut data = &self.input.as_ref()[dfd_start + 4..dfd_end];

        // If we ever encounter a partial DFD, we want to throw an error,
        // not silently ignore the rest of the DFD data. We should end up consuming
        // all of the DFD data. If we end up with unconsumed DFD data, we let
        // dfd::Block::parse throw an error.
        while !data.is_empty() {
            let (block, consumed) = dfd::Block::parse(data)?;
            self.dfd_blocks.push(block);
            data = &data[consumed..];
        }

        Ok(())
    }

    /// Parses the level index table that immediately follows the 80-byte header.
    /// Each entry is 24 bytes. Used internally; prefer [`levels`](Self::levels) for data access.
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

    /// The raw KTX2 file bytes backing this reader.
    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }

    /// Container-level metadata (dimensions, format, compression, etc.).
    pub fn header(&self) -> Header {
        self.header
    }

    /// The color primaries used by this image (e.g. BT.709, BT.2020, etc.).
    ///
    /// Shorthand for [`dfd::Basic::color_primaries`]. Returns `None` if there is no basic DFD block.
    pub fn color_primaries(&self) -> Option<ColorPrimaries> {
        self.basic_dfd()?.color_primaries
    }

    /// The transfer function used by this image (e.g. Linear, sRGB, PQ, etc.).
    ///
    /// Shorthand for [`dfd::Basic::transfer_function`]. Returns `None` if there is no basic DFD block.
    pub fn transfer_function(&self) -> Option<TransferFunction> {
        self.basic_dfd()?.transfer_function
    }

    /// The color model used by this image (e.g. RGB, YUV, etc.). Note
    /// that compressed formats will have a dedicated color model (e.g. BC1, ASTC)
    /// rather than RGB, even if the uncompressed data would be RGB.
    ///
    /// Shorthand for [`dfd::Basic::color_model`]. Returns `None` if there is no basic DFD block.
    pub fn color_model(&self) -> Option<ColorModel> {
        self.basic_dfd()?.color_model
    }

    /// The alpha premuliplication state of the image. `true` if the RGB channels
    /// are premultiplied by alpha, `false` if not.
    ///
    /// Shorthand for [`dfd::Basic::flags`]'s [`dfd::DataFormatFlags::ALPHA_PREMULTIPLIED`] flag.
    /// Returns `None` if there is no basic DFD block.
    pub fn is_alpha_premultiplied(&self) -> Option<bool> {
        Some(
            self.basic_dfd()?
                .flags
                .contains(dfd::DataFormatFlags::ALPHA_PREMULTIPLIED),
        )
    }

    /// The program used to write this file, if specified by this file.
    ///
    /// Shorthand for retrieving the `KTXwriter` key from the key/value data.
    ///
    /// Returns None if:
    /// - The file doesn't contain a `KTXwriter` key.
    /// - The `KTXwriter` value is not valid UTF-8.
    pub fn writer(&self) -> Option<&str> {
        self.key_value_data()
            .find(|(key, _)| *key == "KTXwriter")
            .and_then(|(_, value)| core::str::from_utf8(value).ok())
    }

    /// Iterator over the texture's mip levels, ordered largest to smallest
    /// (level 0 first, level *N-1* last). Each [`Level`] contains the raw
    /// (possibly supercompressed) bytes for that level.
    pub fn levels(&self) -> impl ExactSizeIterator<Item = Level<'_>> + '_ {
        self.level_index().unwrap().map(move |level| Level {
            // Bounds-checking previously performed in `new`
            data: &self.input.as_ref()[level.byte_offset as usize..(level.byte_offset + level.byte_length) as usize],
            uncompressed_byte_length: level.uncompressed_byte_length,
        })
    }

    /// Supercompression Global Data (SGD) section. Currently only used by
    /// BasisLZ (scheme 1) for codebooks and image descriptors. Empty for
    /// other schemes.
    pub fn supercompression_global_data(&self) -> &[u8] {
        let header = self.header();
        let start = header.index.sgd_byte_offset as usize;
        // Bounds-checking previously performed in `new`
        let end = (header.index.sgd_byte_offset + header.index.sgd_byte_length) as usize;
        &self.input.as_ref()[start..end]
    }

    /// The Data Format Descriptor blocks. Most KTX2 files contain exactly
    /// one [`dfd::Block::Basic`] block. Use this to inspect color model,
    /// transfer function, primaries, and per-sample layout.
    pub fn dfd_blocks(&self) -> &[dfd::Block] {
        &self.dfd_blocks
    }

    /// The first [`dfd::Basic`] block, if present.
    ///
    /// Nearly all KTX2 files contain exactly one basic DFD block. Returns
    /// `None` only for files that exclusively use non-standard
    /// descriptor blocks.
    pub fn basic_dfd(&self) -> Option<&dfd::Basic> {
        self.dfd_blocks.iter().find_map(|block| match block {
            dfd::Block::Basic(basic) => Some(basic),
            _ => None,
        })
    }

    /// Iterator over key/value metadata pairs. Keys are UTF-8 strings;
    /// values are raw bytes (often NUL-terminated UTF-8, but not always).
    ///
    /// # Standard Keys
    ///
    /// The KTX specification defines a number of standard keys. Most commonly,
    /// the `KTXwriter` key is used to indicate the tool that wrote the file.
    ///
    /// For a full list of standard keys, see the [KTX specification](https://github.khronos.org/KTX-Specification/ktxspec.v2.html#_keyvalue_data).
    pub fn key_value_data(&self) -> KeyValueDataIterator<'_> {
        let header = self.header();

        let start = header.index.kvd_byte_offset as usize;
        // Bounds-checking previously performed in `new`
        let end = (header.index.kvd_byte_offset + header.index.kvd_byte_length) as usize;

        KeyValueDataIterator::new(&self.input.as_ref()[start..end])
    }
}

/// Iterator over KTX2 key/value metadata pairs.
///
/// Each item is `(key, value)` where `key` is a UTF-8 string and `value` is
/// raw bytes (often UTF-8, but not guaranteed). Malformed entries are silently
/// skipped. Prefer [`Reader::key_value_data`] over constructing this directly.
pub struct KeyValueDataIterator<'data> {
    data: &'data [u8],
}

impl<'data> KeyValueDataIterator<'data> {
    /// Create a new iterator from the raw key/value data section bytes.
    ///
    /// The slice should span from [`Index::kvd_byte_offset`] to
    /// `kvd_byte_offset + kvd_byte_length` relative to the start of the file.
    /// Prefer [`Reader::key_value_data`] which handles this for you.
    pub fn new(data: &'data [u8]) -> Self {
        Self { data }
    }
}

impl<'data> Iterator for KeyValueDataIterator<'data> {
    type Item = (&'data str, &'data [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        let mut offset = 0;

        loop {
            let length = util::bytes_to_u32(self.data, &mut offset).ok()?;

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

/// 12-byte file identifier: `«KTX 20»\r\n\x1A\n`. Must appear at offset 0.
pub const MAGIC: [u8; 12] = [0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A];

/// Result of parsing data operation.
type ParseResult<T> = Result<T, ParseError>;

/// Container-level metadata (dimensions, format, layout) from the 80-byte KTX2 file header.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Header {
    /// Vulkan `VkFormat` enum value. `None` means `VK_FORMAT_UNDEFINED`,
    /// used for supercompressed universal formats (e.g. Basis Universal)
    /// where the actual format is determined at transcode time.
    pub format: Option<Format>,
    /// Size in bytes of the data type used for GPU upload, for endian conversion
    /// on big-endian systems (all image data in KTX2 is little-endian).
    ///
    /// Must be `1` when `format` is `None` (VK_FORMAT_UNDEFINED) or for
    /// block-compressed formats (`_BLOCK` suffix). For packed formats
    /// (`_PACKxx`), equals the byte size of the packed unit `xx / 8`
    /// (e.g. `4` for `_PACK32`). For unpacked formats, equals the byte size
    /// of a single component (e.g. `2` for `R16G16B16_UNORM`). For combined
    /// depth/stencil: `2` for `D16_UNORM_S8_UINT`, `4` for all others.
    pub type_size: u32,
    /// Texture width in texels. Always non-zero.
    pub pixel_width: u32,
    /// Texture height in texels. `0` for 1D textures.
    pub pixel_height: u32,
    /// Texture depth in texels. `0` for non-3D textures.
    pub pixel_depth: u32,
    /// Number of array layers. `0` means a non-array texture (1 implicit
    /// layer). Use `layer_count.max(1)` when allocating storage.
    pub layer_count: u32,
    /// Number of cubemap faces. `6` for cubemaps, `1` otherwise.
    pub face_count: u32,
    /// Number of mip levels. `0` means the full mip chain should be
    /// generated from level 0 by the application if needed.
    /// Use `level_count.max(1)` when iterating stored levels.
    pub level_count: u32,
    /// Compression applied to mip level data. `None` means uncompressed.
    /// When set, each [`Level::data`] must be decompressed before use.
    pub supercompression_scheme: Option<SupercompressionScheme>,
    /// Raw byte offsets/lengths for the DFD, KVD, and SGD sections.
    /// For most use cases, prefer [`Reader::dfd_blocks`],
    /// [`Reader::key_value_data`], and
    /// [`Reader::supercompression_global_data`] instead.
    pub index: Index,
}

/// Byte offsets and lengths (from start of file) for the DFD, KVD, and SGD sections.
///
/// You typically don't need these directly — use [`Reader::dfd_blocks`],
/// [`Reader::key_value_data`], and [`Reader::supercompression_global_data`] instead.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Index {
    /// Byte offset of the Data Format Descriptor section.
    pub dfd_byte_offset: u32,
    /// Byte length of the Data Format Descriptor section.
    pub dfd_byte_length: u32,
    /// Byte offset of the Key/Value Data section.
    pub kvd_byte_offset: u32,
    /// Byte length of the Key/Value Data section. `0` if absent.
    pub kvd_byte_length: u32,
    /// Byte offset of the Supercompression Global Data section.
    pub sgd_byte_offset: u64,
    /// Byte length of the Supercompression Global Data section. `0` if absent.
    pub sgd_byte_length: u64,
}

impl Header {
    /// Size of the KTX2 header in bytes (12-byte magic + 68-byte fields).
    pub const LENGTH: usize = 80;

    /// Decode a header from exactly 80 bytes. Validates the magic bytes and
    /// rejects zero `pixel_width` or zero `face_count`.
    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> ParseResult<Self> {
        if !data.starts_with(&MAGIC) {
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

    /// Serialize this header back to 80 bytes (including magic).
    pub fn as_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0; Self::LENGTH];

        let format = self.format.map(|format| format.value()).unwrap_or(0);
        let supercompression_scheme = self.supercompression_scheme.map(|scheme| scheme.value()).unwrap_or(0);

        bytes[0..12].copy_from_slice(&MAGIC);
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

/// A single mip level's data, returned by [`Reader::levels`].
///
/// If [`Header::supercompression_scheme`] is `Some`, `data` is still
/// compressed — decompress it (e.g. via zstd/zlib) before interpreting
/// the texels according to [`Header::format`].
pub struct Level<'a> {
    /// Raw (possibly supercompressed) bytes for this mip level.
    ///
    /// After decompression, the data is laid out as:
    /// `layer → face → z-slice → row → texel/block`, where layers come
    /// from `layer_count` (1 if 0), faces from `face_count` (6 for
    /// cubemaps), and z-slices from `pixel_depth` (for 3D textures).
    pub data: &'a [u8],
    /// Size of `data` after decompression. Equals `data.len()` when no
    /// supercompression is applied. `0` for BasisLZ (transcode instead).
    pub uncompressed_byte_length: u64,
}

/// Offsets dictating the location of a [`Level`] within a file.
///
/// This is mainly useful for writing or low-level manipulation. Prefer [`Reader::levels`] for data access.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LevelIndex {
    /// Byte offset from the start of the file to this level's data.
    pub byte_offset: u64,
    /// Byte length of the (possibly supercompressed) level data.
    pub byte_length: u64,
    /// Byte length after decompression. `0` for BasisLZ.
    pub uncompressed_byte_length: u64,
}

impl LevelIndex {
    /// Size of one level index entry in bytes.
    pub const LENGTH: usize = 24;

    /// Decode a level index entry from 24 little-endian bytes.
    pub fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
        Self {
            byte_offset: u64::from_le_bytes(data[0..8].try_into().unwrap()),
            byte_length: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            uncompressed_byte_length: u64::from_le_bytes(data[16..24].try_into().unwrap()),
        }
    }

    /// Serialize this entry back to 24 little-endian bytes.
    pub fn as_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0; Self::LENGTH];

        bytes[0..8].copy_from_slice(&self.byte_offset.to_le_bytes()[..]);
        bytes[8..16].copy_from_slice(&self.byte_length.to_le_bytes()[..]);
        bytes[16..24].copy_from_slice(&self.uncompressed_byte_length.to_le_bytes()[..]);

        bytes
    }
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
