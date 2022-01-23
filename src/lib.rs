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

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

mod enums;
mod error;

use alloc::vec::Vec;

pub use crate::{
    enums::{Format, SupercompressionScheme},
    error::ParseError,
};

use core::convert::TryInto;

/// Decodes KTX2 texture data
pub struct Reader<Data: AsRef<[u8]>> {
    input: Data,
}

impl<Data: AsRef<[u8]>> Reader<Data> {
    /// Decode KTX2 data from `input`
    pub fn new(input: Data) -> Result<Self, ParseError> {
        if input.as_ref().len() < Header::LENGTH {
            return Err(ParseError::UnexpectedEnd);
        }
        if !input.as_ref().starts_with(&KTX2_MAGIC) {
            return Err(ParseError::BadMagic);
        }
        let header = input.as_ref()[0..Header::LENGTH].try_into().unwrap();
        Header::from_bytes(header).validate()?;

        let result = Self { input };
        result.level_index()?; // Check index integrity

        // Check level data integrity
        let trailing = result.level_index().unwrap().max_by_key(|l| l.offset).unwrap();
        if trailing.offset + trailing.length_bytes > result.input.as_ref().len() as u64 {
            return Err(ParseError::UnexpectedEnd);
        }

        Ok(result)
    }

    fn level_index(&self) -> ParseResult<impl ExactSizeIterator<Item = LevelIndex> + '_> {
        let level_count = self.header().level_count.max(1) as usize;

        let level_index_end_byte = Header::LENGTH + level_count * LevelIndex::LENGTH;
        let level_index_bytes = self
            .input
            .as_ref()
            .get(Header::LENGTH..level_index_end_byte)
            .ok_or(ParseError::UnexpectedEnd)?;
        Ok(level_index_bytes
            .chunks_exact(LevelIndex::LENGTH)
            .map(LevelIndex::from_bytes))
    }

    /// Access underlying raw bytes
    pub fn data(&self) -> &[u8] {
        self.input.as_ref()
    }

    /// Container-level metadata
    pub fn header(&self) -> Header {
        let bytes = self.input.as_ref()[0..Header::LENGTH].try_into().unwrap();
        Header::from_bytes(bytes)
    }

    /// Iterator over the texture's mip levels
    pub fn levels(&self) -> impl ExactSizeIterator<Item = &[u8]> + '_ {
        self.level_index()
            .unwrap()
            .map(move |level| &self.input.as_ref()[level.offset as usize..(level.offset + level.length_bytes) as usize])
    }

    pub fn supercompression_global_data(&self) -> &[u8] {
        let header = self.header();
        let start = header.sgd_byte_offset as usize;
        let end = (header.sgd_byte_offset + header.sgd_byte_length) as usize;
        self.input.as_ref()[start..end].try_into().unwrap()
    }

    pub fn data_format_descriptors(&self) -> Vec<BasicDataFormatDescriptor> {
        let header = self.header();
        let start = header.dfd_byte_offset as usize;
        let length: u32 = u32::from_le_bytes(self.input.as_ref()[start..start + 4].try_into().unwrap());
        assert_eq!(length, header.dfd_byte_length);
        assert_eq!(length, header.kvd_byte_offset - header.dfd_byte_offset);
        let end = (header.dfd_byte_offset + header.dfd_byte_length) as usize;
        let dfd_data: &[u8] = self.input.as_ref()[start..end].try_into().unwrap();
        let mut descriptors = Vec::new();
        let mut byte_offset = 4usize;
        while byte_offset < length as usize {
            let descriptor = BasicDataFormatDescriptor::from_bytes(&dfd_data[byte_offset..]);
            // NOTE: Sanity check to not get stuck in an infinite loop
            if descriptor.descriptor_block_size == 0 {
                break;
            }
            byte_offset += descriptor.descriptor_block_size as usize;
            descriptors.push(descriptor);
        }
        descriptors
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
    dfd_byte_offset: u32,
    dfd_byte_length: u32,
    kvd_byte_offset: u32,
    kvd_byte_length: u32,
    sgd_byte_offset: u64,
    sgd_byte_length: u64,
}

impl Header {
    const LENGTH: usize = 80;

    fn from_bytes(data: &[u8; Self::LENGTH]) -> Self {
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
            dfd_byte_offset: u32::from_le_bytes(data[48..52].try_into().unwrap()),
            dfd_byte_length: u32::from_le_bytes(data[52..56].try_into().unwrap()),
            kvd_byte_offset: u32::from_le_bytes(data[56..60].try_into().unwrap()),
            kvd_byte_length: u32::from_le_bytes(data[60..64].try_into().unwrap()),
            sgd_byte_offset: u64::from_le_bytes(data[64..72].try_into().unwrap()),
            sgd_byte_length: u64::from_le_bytes(data[72..80].try_into().unwrap()),
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
    const LENGTH: usize = 24;

    pub fn from_bytes(data: &[u8]) -> Self {
        Self {
            offset: u64::from_le_bytes(data[0..8].try_into().unwrap()),
            length_bytes: u64::from_le_bytes(data[8..16].try_into().unwrap()),
            uncompressed_length_bytes: u64::from_le_bytes(data[16..24].try_into().unwrap()),
        }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ChannelTypeQualifiers: u32 {
        const LINEAR        = (1 << 0);
        const EXPONENT      = (1 << 1);
        const SIGNED        = (1 << 2);
        const FLOAT         = (1 << 3);
        const NONE          = 0;
        const UNINITIALIZED = 0xFFFF;
    }
}

bitflags::bitflags! {
    #[derive(Default)]
    #[repr(transparent)]
    pub struct DataFormatFlags: u32 {
        const ALPHA_PREMULTIPLIED        = (1 << 0);
        const NONE          = 0;
        const UNINITIALIZED = 0xFFFF;
    }
}

// This can be obtained from std::mem::transmute::<f32, u32>(1.0f32) but we
// want to support nostd. It is used for identifying normalized sample types
// as in Unorm or Snorm
const F32_1_AS_U32: u32 = 1065353216;

#[derive(Debug, Default)]
pub struct BasicDataFormatDescriptor {
    pub vendor_id: u32,                      //: 17;
    pub descriptor_type: u32,                //: 15;
    pub version_number: u32,                 //: 16;
    pub descriptor_block_size: u32,          //: 16;
    pub color_model: ColorModel,             //: 8;
    pub color_primaries: ColorPrimaries,     //: 8;
    pub transfer_function: TransferFunction, //: 8;
    pub flags: DataFormatFlags,              //: 8;
    pub texel_block_dimensions: [u32; 4],    //: 8 x 4;
    pub bytes_planes: [u32; 8],              //: 8 x 8;
    pub samples: Vec<SampleInformation>,
}

impl BasicDataFormatDescriptor {
    const HEADER_LENGTH: usize = 24;
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut offset = 0;

        let v = bytes_to_u32(bytes, &mut offset);
        let vendor_id = shift_and_mask_lower(0, 17, v);
        assert_eq!(vendor_id, 0); // Basic Data Format Descriptor requirement
        let descriptor_type = shift_and_mask_lower(17, 15, v);
        assert_eq!(descriptor_type, 0); // Basic Data Format Descriptor requirement

        let v = bytes_to_u32(bytes, &mut offset);
        let version_number = shift_and_mask_lower(0, 16, v);
        assert_eq!(version_number, 2); // Basic Data Format Descriptor requirement
        let descriptor_block_size = shift_and_mask_lower(16, 16, v);

        let n_samples = (descriptor_block_size - Self::HEADER_LENGTH as u32) / SampleInformation::LENGTH as u32;

        let v = bytes_to_u32(bytes, &mut offset);
        let model = shift_and_mask_lower(0, 8, v);
        let primaries = shift_and_mask_lower(8, 8, v);
        let transfer = shift_and_mask_lower(16, 8, v);
        let flags = shift_and_mask_lower(24, 8, v);

        let v = bytes_to_u32(bytes, &mut offset);
        let texel_block_dimensions = [
            shift_and_mask_lower(0, 8, v) + 1,
            shift_and_mask_lower(8, 8, v) + 1,
            shift_and_mask_lower(16, 8, v) + 1,
            shift_and_mask_lower(24, 8, v) + 1,
        ];

        let v = bytes_to_u32(bytes, &mut offset);
        let mut bytes_planes = [0u32; 8];
        bytes_planes[0] = shift_and_mask_lower(0, 8, v);
        bytes_planes[1] = shift_and_mask_lower(8, 8, v);
        bytes_planes[2] = shift_and_mask_lower(16, 8, v);
        bytes_planes[3] = shift_and_mask_lower(24, 8, v);

        let v = bytes_to_u32(bytes, &mut offset);
        bytes_planes[4] = shift_and_mask_lower(0, 8, v);
        bytes_planes[5] = shift_and_mask_lower(8, 8, v);
        bytes_planes[6] = shift_and_mask_lower(16, 8, v);
        bytes_planes[7] = shift_and_mask_lower(24, 8, v);

        let mut samples = Vec::new();
        while (samples.len() as u32) < n_samples {
            samples.push(SampleInformation::from_bytes(&bytes[offset..offset + 16]));
            offset += 16;
        }
        assert_eq!(offset as u32, descriptor_block_size);

        Self {
            vendor_id,
            descriptor_type,
            version_number,
            descriptor_block_size,
            color_model: ColorModel::from(model),
            color_primaries: ColorPrimaries::from(primaries),
            transfer_function: TransferFunction::from(transfer),
            flags: DataFormatFlags::from_bits_truncate(flags),
            texel_block_dimensions,
            bytes_planes,
            samples,
        }
    }
}

#[derive(Debug)]
pub struct SampleInformation {
    pub bit_offset: u32,                                //: 16;
    pub bit_length: u32,                                //: 8;
    pub channel_type: u32,                              //: 4;
    pub channel_type_qualifiers: ChannelTypeQualifiers, //: 4;
    pub sample_positions: [u32; 4],                     //: 8 x 4;
    pub lower: u32,                                     //;
    pub upper: u32,                                     //;
}

impl SampleInformation {
    const LENGTH: usize = 16;

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut offset = 0;

        let v = bytes_to_u32(bytes, &mut offset);
        let bit_offset = shift_and_mask_lower(0, 16, v);
        let bit_length = shift_and_mask_lower(16, 8, v) + 1;
        let channel_type = shift_and_mask_lower(24, 4, v);
        let channel_type_qualifiers = ChannelTypeQualifiers::from_bits_truncate(shift_and_mask_lower(24, 4, v));

        let v = bytes_to_u32(bytes, &mut offset);
        let sample_positions = [
            shift_and_mask_lower(0, 8, v),
            shift_and_mask_lower(8, 8, v),
            shift_and_mask_lower(16, 8, v),
            shift_and_mask_lower(24, 8, v),
        ];
        let lower = bytes_to_u32(bytes, &mut offset);
        let upper = bytes_to_u32(bytes, &mut offset);

        assert_eq!(offset, 16);

        Self {
            bit_offset,
            bit_length,
            channel_type,
            channel_type_qualifiers,
            sample_positions,
            lower,
            upper,
        }
    }

    pub fn is_exponent(&self) -> bool {
        self.channel_type_qualifiers.contains(ChannelTypeQualifiers::EXPONENT)
    }

    pub fn is_float(&self) -> bool {
        self.channel_type_qualifiers.contains(ChannelTypeQualifiers::FLOAT)
    }

    pub fn is_linear(&self) -> bool {
        self.channel_type_qualifiers.contains(ChannelTypeQualifiers::LINEAR)
    }

    pub fn is_signed(&self) -> bool {
        self.channel_type_qualifiers.contains(ChannelTypeQualifiers::SIGNED)
    }

    pub fn is_norm(&self) -> bool {
        self.is_float() && self.upper == F32_1_AS_U32
    }
}

#[derive(Debug)]
pub enum ColorModel {
    Unspecified, // 0
    RGBSDA,      // 1
    YUVSDA,      // 2
    YIQSDA,      // 3
    LabSDA,      // 4
    CMYKA,       // 5
    XYZW,        // 6
    HSVAAng,     // 7
    HSLAAng,     // 8
    HSVAHex,     // 9
    HSLAHex,     // 10
    YCgCoA,      // 11
    YcCbcCrc,    // 12
    ICtCp,       // 13
    CIEXYZ,      // 14
    CIEXYY,      // 15
    BC1A,        // 128
    BC2,         // 129
    BC3,         // 130
    BC4,         // 131
    BC5,         // 132
    BC6H,        // 133
    BC7,         // 134
    ETC1,        // 160
    ETC2,        // 161
    ASTC,        // 162
    ETC1S,       // 163
    PVRTC,       // 164
    PVRTC2,      // 165
    UASTC,       // 166
    Unknown(u32),
}

impl From<u32> for ColorModel {
    fn from(color_model: u32) -> Self {
        match color_model {
            0 => Self::Unspecified,
            1 => Self::RGBSDA,
            2 => Self::YUVSDA,
            3 => Self::YIQSDA,
            4 => Self::LabSDA,
            5 => Self::CMYKA,
            6 => Self::XYZW,
            7 => Self::HSVAAng,
            8 => Self::HSLAAng,
            9 => Self::HSVAHex,
            10 => Self::HSLAHex,
            11 => Self::YCgCoA,
            12 => Self::YcCbcCrc,
            13 => Self::ICtCp,
            14 => Self::CIEXYZ,
            15 => Self::CIEXYY,
            128 => Self::BC1A,
            129 => Self::BC2,
            130 => Self::BC3,
            131 => Self::BC4,
            132 => Self::BC5,
            133 => Self::BC6H,
            134 => Self::BC7,
            160 => Self::ETC1,
            161 => Self::ETC2,
            162 => Self::ASTC,
            163 => Self::ETC1S,
            164 => Self::PVRTC,
            165 => Self::PVRTC2,
            166 => Self::UASTC,
            v => Self::Unknown(v),
        }
    }
}

impl Default for ColorModel {
    fn default() -> Self {
        Self::Unspecified
    }
}

#[derive(Debug)]
pub enum ColorPrimaries {
    Unspecified, // 0
    BT709,       // 1
    BT601EBU,    // 2
    BT601SMPTE,  // 3
    BT2020,      // 4
    CIEXYZ,      // 5
    ACES,        // 6
    ACESCC,      // 7
    NTSC1953,    // 8
    PAL525,      // 9
    DISPLAYP3,   // 10
    AdobeRGB,    // 11
    Unknown(u32),
}

impl Default for ColorPrimaries {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl From<u32> for ColorPrimaries {
    fn from(color_primaries: u32) -> Self {
        match color_primaries {
            0 => Self::Unspecified,
            1 => Self::BT709,
            2 => Self::BT601EBU,
            3 => Self::BT601SMPTE,
            4 => Self::BT2020,
            5 => Self::CIEXYZ,
            6 => Self::ACES,
            7 => Self::ACESCC,
            8 => Self::NTSC1953,
            9 => Self::PAL525,
            10 => Self::DISPLAYP3,
            11 => Self::AdobeRGB,
            v => Self::Unknown(v),
        }
    }
}

#[derive(Debug)]
pub enum TransferFunction {
    Unspecified, // 0
    Linear,      // 1
    SRGB,        // 2
    ITU,         // 3
    NTSC,        // 4
    SLOG,        // 5
    SLOG2,       // 6
    BT1886,      // 7
    HLGOETF,     // 8
    HLGEOTF,     // 9
    PQEOTF,      // 10
    PQOETF,      // 11
    DCIP3,       // 12
    PALOETF,     // 13
    PAL625EOTF,  // 14
    ST240,       // 15
    ACESCC,      // 16
    ACESCCT,     // 17
    AdobeRGB,    // 18
    Unknown(u32),
}

impl Default for TransferFunction {
    fn default() -> Self {
        Self::Unspecified
    }
}

impl From<u32> for TransferFunction {
    fn from(transfer_function: u32) -> Self {
        match transfer_function {
            0 => Self::Unspecified,
            1 => Self::Linear,
            2 => Self::SRGB,
            3 => Self::ITU,
            4 => Self::NTSC,
            5 => Self::SLOG,
            6 => Self::SLOG2,
            7 => Self::BT1886,
            8 => Self::HLGOETF,
            9 => Self::HLGEOTF,
            10 => Self::PQEOTF,
            11 => Self::PQOETF,
            12 => Self::DCIP3,
            13 => Self::PALOETF,
            14 => Self::PAL625EOTF,
            15 => Self::ST240,
            16 => Self::ACESCC,
            17 => Self::ACESCCT,
            18 => Self::AdobeRGB,
            v => Self::Unknown(v),
        }
    }
}

#[inline]
fn bytes_to_u32(bytes: &[u8], offset: &mut usize) -> u32 {
    let v = u32::from_le_bytes(bytes[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    v
}

#[inline]
fn shift_and_mask_lower(shift: u32, mask: u32, value: u32) -> u32 {
    (value >> shift) & ((1 << mask) - 1)
}
