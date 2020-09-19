#[macro_use]
extern crate num_derive;

pub mod error;
pub mod format;

use crate::format::Format;

use crate::error::{ParseError, ReadError};
use byteorder::{ByteOrder, NativeEndian};
use std::convert::TryInto;
use std::io::SeekFrom;
use tokio::io::AsyncSeek;
use tokio::prelude::*;

pub struct Reader<T> {
    input: T,
    head: Header,
    levels_index: Vec<LevelInfo>,
}

impl<T: AsyncRead + AsyncSeek + Unpin> Reader<T> {
    pub async fn new(mut input: T) -> ReadResult<Self> {
        let head = Self::read_head(&mut input).await?;
        let levels_index = Self::read_level_index(&mut input, &head).await?;
        Ok(Self {
            input,
            head,
            levels_index,
        })
    }

    async fn read_head(input: &mut T) -> ReadResult<Header> {
        let mut head_bytes = [0; 48];
        input.read_exact(&mut head_bytes).await?;
        Self::test_identifier(&head_bytes)?;

        Ok(Header::from_bytes(&head_bytes)?)
    }

    async fn read_level_index(input: &mut T, head: &Header) -> ReadResult<Vec<LevelInfo>> {
        const LEVEL_INDEX_START_BYTE: u64 = 80;
        const LEVEL_INDEX_BYTE_LEN: u32 = 24;
        let level_count = head.level_count.max(1);
        let level_index_bytes_len = level_count * LEVEL_INDEX_BYTE_LEN;
        let mut level_index_bytes: Vec<u8> = (0..level_index_bytes_len).map(|_| 0u8).collect();

        input.seek(SeekFrom::Start(LEVEL_INDEX_START_BYTE)).await?;
        input.read_exact(&mut level_index_bytes).await?;
        let mut infos = Vec::with_capacity(level_count as usize);
        for level_index in 0..level_count {
            let start_byte = (level_index * LEVEL_INDEX_BYTE_LEN) as usize;
            let end_byte = start_byte + LEVEL_INDEX_BYTE_LEN as usize;
            infos.push(LevelInfo::from_bytes(
                &level_index_bytes[start_byte..end_byte],
            ))
        }
        Ok(infos)
    }

    pub async fn read_data(&mut self) -> ReadResult<Vec<u8>> {
        let data_start_byte = self.first_level_offset_bytes();
        self.input.seek(SeekFrom::Start(data_start_byte)).await?;
        let data_len_bytes = self.data_len_bytes();
        let mut buffer = Vec::new();
        buffer.resize(data_len_bytes as usize, 0);
        self.input.read_exact(&mut buffer).await?;
        Ok(buffer)
    }

    fn test_identifier(head_bytes: &HeadBytes) -> ReadResult<()> {
        let mut red_id = [0; 12];
        red_id.copy_from_slice(&head_bytes[0..12]);
        if red_id == KTX2_IDENTIFIER {
            return Ok(());
        }
        Err(ReadError::ParseError(ParseError::BadIdentifier(red_id)))
    }

    pub fn header(&self) -> &Header {
        &self.head
    }

    pub fn levels_index(&self) -> &Vec<LevelInfo> {
        &self.levels_index
    }

    pub fn regions_description(&self) -> Vec<RegionDescription> {
        let base_offset = self.first_level_offset_bytes();
        self.levels_index
            .iter()
            .enumerate()
            .map(|(i, level)| self.region_from_level_index(i, level.offset - base_offset))
            .collect()
    }

    fn first_level_offset_bytes(&self) -> u64 {
        self.levels_index
            .iter()
            .map(|l| l.offset)
            .min()
            .expect("No levels got, but read some on constructing")
    }

    pub fn last_level(&self) -> LevelInfo {
        *self
            .levels_index
            .iter()
            .max_by_key(|l| l.offset)
            .expect("No levels got, but read some on constructing")
    }

    pub fn data_len_bytes(&self) -> u64 {
        let start_offset = self.first_level_offset_bytes();
        let last_level = self.last_level();
        last_level.offset + last_level.uncompressed_length_bytes - start_offset
    }

    fn region_from_level_index(&self, i: usize, offset: u64) -> RegionDescription {
        RegionDescription {
            level: i as u32,
            layer_count: self.head.layer_count.max(1) * self.head.face_count,
            offset_bytes: offset,
            width: Self::level_size(self.head.base_width, i as u32),
            height: Self::level_size(self.head.base_height, i as u32),
            depth: Self::level_size(self.head.base_depth, i as u32),
        }
    }

    fn level_size(base: u32, level: u32) -> u32 {
        (base >> level).max(1)
    }
}

static KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];

pub type ReadResult<T> = Result<T, ReadError>;
pub type ParseResult<T> = Result<T, ParseError>;

pub struct TexData {
    pub header: Header,
    pub frames: Vec<RegionDescription>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Header {
    pub format: Format,
    pub type_size: u32,
    pub base_width: u32,
    pub base_height: u32,
    pub base_depth: u32,
    pub layer_count: u32,
    pub face_count: u32,
    pub level_count: u32,
    pub supercompression_scheme: u32,
}

impl Header {
    pub fn from_bytes(data: &HeadBytes) -> ParseResult<Self> {
        let format_id = NativeEndian::read_u32(&data[12..16]);
        let format = format_id.try_into()?;

        Ok(Self {
            format,
            type_size: NativeEndian::read_u32(&data[16..20]),
            base_width: Self::parse_base_width(&data[20..24])?,
            base_height: NativeEndian::read_u32(&data[24..28]),
            base_depth: NativeEndian::read_u32(&data[28..32]),
            layer_count: NativeEndian::read_u32(&data[32..36]),
            face_count: Self::parse_face_count(&data[36..40])?,
            level_count: NativeEndian::read_u32(&data[40..44]),
            supercompression_scheme: Self::parse_supercompression_scheme(&data[44..48])?,
        })
    }

    fn parse_base_width(data: &[u8]) -> ParseResult<u32> {
        let result = NativeEndian::read_u32(&data[0..4]);
        match result {
            0 => Err(ParseError::ZeroWidth),
            _ => Ok(result),
        }
    }

    fn parse_face_count(data: &[u8]) -> ParseResult<u32> {
        let result = NativeEndian::read_u32(&data[0..4]);
        match result {
            0 => Err(ParseError::ZeroFaceCount),
            _ => Ok(result),
        }
    }

    fn parse_supercompression_scheme(data: &[u8]) -> ParseResult<u32> {
        let result = NativeEndian::read_u32(&data[0..4]);
        match result {
            0 => Ok(0),
            _ => Err(ParseError::UnsupportedFeature("supercompression scheme")),
        }
    }
}

type HeadBytes = [u8; 48];

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LevelInfo {
    pub offset: u64,
    pub length_bytes: u64,
    pub uncompressed_length_bytes: u64,
}

impl LevelInfo {
    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            offset: NativeEndian::read_u64(&data[0..8]),
            length_bytes: NativeEndian::read_u64(&data[8..16]),
            uncompressed_length_bytes: NativeEndian::read_u64(&data[16..24]),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RegionDescription {
    pub level: u32,
    pub layer_count: u32,
    pub offset_bytes: u64,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
