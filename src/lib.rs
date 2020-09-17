#[macro_use]
extern crate num_derive;

pub mod error;
pub mod format;

use crate::format::Format;

use crate::error::{ParseError, ReadError};
use byteorder::{ByteOrder, NativeEndian};
use std::convert::TryInto;
use tokio::io::AsyncSeek;
use tokio::prelude::*;

pub struct Reader<T> {
    input: T,
    head: Header,
}

impl<T: AsyncRead + AsyncSeek + Unpin> Reader<T> {
    pub async fn new(mut input: T) -> ReadResult<Self> {
        let head = Self::read_head(&mut input).await?;
        Ok(Self { input, head })
    }

    pub async fn read_frame(&self, frame_index: u64) -> ReadResult<Frame> {
        todo!()
    }

    async fn read_head(input: &mut T) -> ReadResult<Header> {
        let mut head_bytes = [0; 48];
        input.read_exact(&mut head_bytes).await?;
        Self::test_identifier(&head_bytes)?;

        Ok(Header::from_bytes(&head_bytes)?)
    }

    fn test_identifier(head_bytes: &HeadBytes) -> ReadResult<()> {
        let mut red_id = [0; 12];
        red_id.copy_from_slice(&head_bytes[0..12]);
        if red_id == KTX2_IDENTIFIER {
            return Ok(());
        }
        Err(ReadError::ParseError(ParseError::BadIdentifier(red_id)))
    }

    pub fn get_header(&self) -> &Header {
        &self.head
    }
}

static KTX2_IDENTIFIER: [u8; 12] = [
    0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
];

pub type ReadResult<T> = Result<T, ReadError>;
pub type ParseResult<T> = Result<T, ParseError>;

pub struct TexData {
    pub header: Header,
    pub frames: Vec<Frame>,
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

pub struct FrameInfo {
    pub offset: u64,
    pub length: u64,
}

pub struct Frame {
    pub level: u32,
    pub layer: u32,
    pub face: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub data: Vec<u8>,
}
