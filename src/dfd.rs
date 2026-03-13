//! Data Format Descriptor (DFD) for KTX2 textures.
//!
//! Each ktx2 file contains an abstract definition of the data format of the texture data,
//! called the [Data Format Descriptor (DFD)][dfd-spec]). The specification for the DFD is a separate
//! Khronos standard, and is not specific to KTX2.
//!
//! Most of the DFD's data is relatively esoteric, but it is required by the KTX2 specification,
//! and contains information like [`ColorModel`], [`ColorPrimaries`], and [`TransferFunction`] that
//! may be useful to applications.
//!
//! [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html

use core::num::NonZeroU8;

pub use crate::enums::{ColorModel, ColorPrimaries, TransferFunction};
use crate::{bytes_to_u32, read_bytes, read_u16, shift_and_mask_lower, ParseError};

pub struct Block<'data> {
    pub header: BlockHeader,
    pub data: &'data [u8],
}

#[derive(Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub vendor_id: u32,       //: 17;
    pub descriptor_type: u32, //: 15;
    pub version_number: u16,  //: 16;
}

impl BlockHeader {
    pub const LENGTH: usize = 8;

    pub const BASIC: Self = Self {
        vendor_id: 0,
        descriptor_type: 0,
        version_number: 2,
    };

    pub fn as_bytes(&self, descriptor_block_size: u16) -> [u8; Self::LENGTH] {
        let mut output = [0u8; Self::LENGTH];

        let first_word = (self.vendor_id & ((1 << 17) - 1)) | (self.descriptor_type << 17);
        output[0..4].copy_from_slice(&first_word.to_le_bytes());
        output[4..6].copy_from_slice(&self.version_number.to_le_bytes());
        output[6..8].copy_from_slice(&descriptor_block_size.to_le_bytes());

        output
    }

    pub(crate) fn parse(bytes: &[u8]) -> Result<(Self, usize), ParseError> {
        let mut offset = 0;

        let v = bytes_to_u32(bytes, &mut offset)?;
        let vendor_id = shift_and_mask_lower(0, 17, v);
        let descriptor_type = shift_and_mask_lower(17, 15, v);

        let version_number = read_u16(bytes, &mut offset)?;
        let descriptor_block_size = read_u16(bytes, &mut offset)?;

        Ok((
            Self {
                vendor_id,
                descriptor_type,
                version_number,
            },
            descriptor_block_size as usize,
        ))
    }
}

pub struct Basic<'data> {
    pub header: BasicHeader,
    sample_information: &'data [u8],
}

impl<'data> Basic<'data> {
    pub fn parse(bytes: &'data [u8]) -> Result<Self, ParseError> {
        let header_data = bytes
            .get(0..BasicHeader::LENGTH)
            .ok_or(ParseError::UnexpectedEnd)?
            .try_into()
            .unwrap();
        let header = BasicHeader::from_bytes(header_data)?;

        Ok(Self {
            header,
            sample_information: &bytes[BasicHeader::LENGTH..],
        })
    }

    pub fn sample_information(&self) -> impl Iterator<Item = SampleInformation> + 'data {
        SampleInformationIterator {
            data: self.sample_information,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BasicHeader {
    /// None means Unspecified
    pub color_model: Option<ColorModel>, //: 8;
    /// None means Unspecified
    pub color_primaries: Option<ColorPrimaries>, //: 8;
    /// None means Unspecified
    pub transfer_function: Option<TransferFunction>, //: 8;
    pub flags: DataFormatFlags,                 //: 8;
    pub texel_block_dimensions: [NonZeroU8; 4], //: 8 x 4;
    pub bytes_planes: [u8; 8],                  //: 8 x 8;
}

impl BasicHeader {
    pub const LENGTH: usize = 16;

    pub fn as_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];

        let color_model = self.color_model.map(|c| c.value()).unwrap_or(0);
        let color_primaries = self.color_primaries.map(|c| c.value()).unwrap_or(0);
        let transfer_function = self.transfer_function.map(|t| t.value()).unwrap_or(0);

        let texel_block_dimensions = self.texel_block_dimensions.map(|dim| dim.get() - 1);

        bytes[0] = color_model;
        bytes[1] = color_primaries;
        bytes[2] = transfer_function;
        bytes[3] = self.flags.bits();
        bytes[4..8].copy_from_slice(&texel_block_dimensions);
        bytes[8..16].copy_from_slice(&self.bytes_planes);

        bytes
    }

    pub fn from_bytes(bytes: &[u8; Self::LENGTH]) -> Result<Self, ParseError> {
        let mut offset = 0;

        let [model, primaries, transfer, flags] = read_bytes(bytes, &mut offset)?;
        let texel_block_dimensions = read_bytes(bytes, &mut offset)?.map(|dim| NonZeroU8::new(dim + 1).unwrap());
        let bytes_planes = read_bytes(bytes, &mut offset)?;

        Ok(Self {
            color_model: ColorModel::new(model),
            color_primaries: ColorPrimaries::new(primaries),
            transfer_function: TransferFunction::new(transfer),
            flags: DataFormatFlags::from_bits_truncate(flags),
            texel_block_dimensions,
            bytes_planes,
        })
    }
}

struct SampleInformationIterator<'data> {
    data: &'data [u8],
}

impl Iterator for SampleInformationIterator<'_> {
    type Item = SampleInformation;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.data.get(0..SampleInformation::LENGTH)?.try_into().unwrap();
        SampleInformation::from_bytes(&bytes).map_or(None, |sample_information| {
            self.data = &self.data[SampleInformation::LENGTH..];
            Some(sample_information)
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SampleInformation {
    pub bit_offset: u16,                                //: 16;
    pub bit_length: NonZeroU8,                          //: 8;
    pub channel_type: u8,                               //: 4;
    pub channel_type_qualifiers: ChannelTypeQualifiers, //: 4;
    pub sample_positions: [u8; 4],                      //: 8 x 4;
    pub lower: u32,                                     //: 32;
    pub upper: u32,                                     //: 32;
}

impl SampleInformation {
    pub const LENGTH: usize = 16;

    pub fn as_bytes(&self) -> [u8; Self::LENGTH] {
        let mut bytes = [0u8; Self::LENGTH];

        let channel_info = self.channel_type | (self.channel_type_qualifiers.bits() << 4);

        bytes[0..2].copy_from_slice(&self.bit_offset.to_le_bytes());
        bytes[2] = self.bit_length.get() - 1;
        bytes[3] = channel_info;
        bytes[4..8].copy_from_slice(&self.sample_positions);
        bytes[8..12].copy_from_slice(&self.lower.to_le_bytes());
        bytes[12..16].copy_from_slice(&self.upper.to_le_bytes());

        bytes
    }

    pub fn from_bytes(bytes: &[u8; Self::LENGTH]) -> Result<Self, ParseError> {
        let mut offset = 0;

        let v = bytes_to_u32(bytes, &mut offset)?;
        let bit_offset = shift_and_mask_lower(0, 16, v) as u16;
        let bit_length = (shift_and_mask_lower(16, 8, v) as u8)
            .checked_add(1)
            .and_then(NonZeroU8::new)
            .ok_or(ParseError::InvalidSampleBitLength)?;
        let channel_type = shift_and_mask_lower(24, 4, v) as u8;
        let channel_type_qualifiers = ChannelTypeQualifiers::from_bits_truncate(shift_and_mask_lower(28, 4, v) as u8);

        let sample_positions = read_bytes(bytes, &mut offset)?;
        let lower = bytes_to_u32(bytes, &mut offset)?;
        let upper = bytes_to_u32(bytes, &mut offset)?;

        Ok(Self {
            bit_offset,
            bit_length,
            channel_type,
            channel_type_qualifiers,
            sample_positions,
            lower,
            upper,
        })
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ChannelTypeQualifiers: u8 {
        const LINEAR        = (1 << 0);
        const EXPONENT      = (1 << 1);
        const SIGNED        = (1 << 2);
        const FLOAT         = (1 << 3);
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataFormatFlags: u8 {
        const STRAIGHT_ALPHA             = 0;
        const ALPHA_PREMULTIPLIED        = (1 << 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_nonzero<const N: usize>(input: [u8; N]) -> [NonZeroU8; N] {
        input.map(|n| NonZeroU8::new(n).unwrap())
    }

    #[test]
    fn basic_dfd_header_roundtrip() {
        let header = BasicHeader {
            color_model: Some(ColorModel::LabSDA),
            color_primaries: Some(ColorPrimaries::ACES),
            transfer_function: Some(TransferFunction::ITU),
            flags: DataFormatFlags::STRAIGHT_ALPHA,
            texel_block_dimensions: to_nonzero([1, 2, 3, 4]),
            bytes_planes: [5, 6, 7, 8, 9, 10, 11, 12],
        };

        let bytes = header.as_bytes();
        let decoded = BasicHeader::from_bytes(&bytes).unwrap();
        assert_eq!(header, decoded);
    }

    #[test]
    fn sample_information_roundtrip() {
        let info = SampleInformation {
            bit_offset: 234,
            bit_length: NonZeroU8::new(123).unwrap(),
            channel_type: 2,
            channel_type_qualifiers: ChannelTypeQualifiers::LINEAR,
            sample_positions: [1, 2, 3, 4],
            lower: 1234,
            upper: 4567,
        };

        let bytes = info.as_bytes();
        let decoded = SampleInformation::from_bytes(&bytes).unwrap();

        assert_eq!(info, decoded);
    }

    #[test]
    fn sample_info_invalid_bit_length() {
        let bytes = &[
            0u8, 0,   // bit_offset
            255, // bit_length
            1,   // channel_type | channel_type_qualifiers
            0, 0, 0, 0, // sample_positions
            0, 0, 0, 0, // lower
            255, 255, 255, 255, // upper
        ];

        assert!(matches!(
            SampleInformation::from_bytes(bytes),
            Err(ParseError::InvalidSampleBitLength)
        ));
    }
}
