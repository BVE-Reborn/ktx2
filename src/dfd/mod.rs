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
//! # Structure
//!
//! A DFD contains one or more [`Block`]s. Each block has a [`BlockHeader`], which describes what type of block it is,
//! and a data blob. The most common block type in KTX2 is the [`Basic`] block which contains information about
//! texture-like data formats.
//!
//! [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html

mod generate;

pub use generate::BuildError;

use alloc::{vec, vec::Vec};
use core::num::NonZeroU8;

use crate::util::{bytes_to_u32, read_bytes, read_u16, shift_and_mask_lower};
use crate::{ColorModel, ColorPrimaries, ParseError, TransferFunction};

/// DFD block, containing a header and a data blob.
///
/// The header describes the type of block, and the data blob contains the block's contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    /// "Basic" DFD block. This is what most KTX2 files contain.
    Basic(Basic),
    /// DFD block type unknown to the ktx2 crate. Use the header
    /// to determine if this is a relevant block type for your application.
    Unknown { header: BlockHeader, data: Vec<u8> },
}

impl Block {
    /// Parses a single [`Block`] from the start of `bytes`, returning the block and the number
    /// of bytes consumed.
    pub(crate) fn parse(bytes: &[u8]) -> Result<(Self, usize), ParseError> {
        let (header, descriptor_block_size) = BlockHeader::from_bytes(
            &bytes
                .get(..BlockHeader::LENGTH)
                .ok_or(ParseError::UnexpectedEnd)?
                .try_into()
                .unwrap(),
        )?;

        if descriptor_block_size == 0 {
            return Err(ParseError::UnexpectedEnd);
        }

        let data = &bytes
            .get(BlockHeader::LENGTH..descriptor_block_size)
            .ok_or(ParseError::UnexpectedEnd)?;

        let block = match header {
            BlockHeader::BASIC => Block::Basic(Basic::parse(data)?),
            _ => Block::Unknown {
                header,
                data: data.to_vec(),
            },
        };

        Ok((block, descriptor_block_size))
    }

    /// Number of bytes the serialized form of this block will take up,
    /// including the [`BlockHeader`].
    pub fn serialized_length(&self) -> usize {
        let data_length = match self {
            Block::Basic(basic) => basic.serialized_length(),
            Block::Unknown { data, .. } => data.len(),
        };
        BlockHeader::LENGTH + data_length
    }

    /// Serializes this block to a given slice of bytes. The slice must be at least
    /// [`serialized_length`](Self::serialized_length) bytes long.
    pub fn to_bytes(&self, output: &mut [u8]) {
        assert!(
            output.len() >= self.serialized_length(),
            "Output buffer is too small to serialize Block: expected at least {} bytes, got {}",
            self.serialized_length(),
            output.len()
        );

        let descriptor_block_size = self.serialized_length() as u16;

        // Serialize the header
        let header = match self {
            Block::Basic(_) => BlockHeader::BASIC,
            Block::Unknown { header, .. } => *header,
        };
        output[..BlockHeader::LENGTH].copy_from_slice(&header.as_bytes(descriptor_block_size));

        // Serialize the data
        match self {
            Block::Basic(basic) => {
                basic.to_bytes(&mut output[BlockHeader::LENGTH..]);
            }
            Block::Unknown { data, .. } => {
                output[BlockHeader::LENGTH..][..data.len()].copy_from_slice(data);
            }
        }
    }

    /// Serializes this block to a vector of bytes.
    pub fn to_vec(&self) -> Vec<u8> {
        let mut output = vec![0u8; self.serialized_length()];
        self.to_bytes(&mut output);
        output
    }
}

/// DFD block header, containing what type and version of block.
///
/// Implementations can skip blocks with unrecognized headers, allowing unknown data to be ignored.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    /// 17-bit organization identifier. `0` is Khronos. PCI SIG IDs use bits 0–15 with bit 16
    /// clear; other IDs are assigned by Khronos starting at 65536.
    pub vendor_id: u32, //: 17;
    /// 15-bit vendor-defined identifier distinguishing between data representations.
    pub descriptor_type: u32, //: 15;
    /// Vendor-defined version, intended for backwards-compatible updates to a descriptor block.
    pub version_number: u16, //: 16;
}

impl BlockHeader {
    /// Number of bytes in a DFD block header.
    pub const LENGTH: usize = 8;

    /// The header for a [`Basic`] block.
    pub const BASIC: Self = Self {
        vendor_id: 0,
        descriptor_type: 0,
        version_number: 2,
    };

    /// Serializes the block header to bytes. `descriptor_block_size` is the
    /// total size of the containing [`Block`] (header + data).
    pub fn as_bytes(&self, descriptor_block_size: u16) -> [u8; Self::LENGTH] {
        let mut output = [0u8; Self::LENGTH];

        let first_word = (self.vendor_id & ((1 << 17) - 1)) | (self.descriptor_type << 17);
        output[0..4].copy_from_slice(&first_word.to_le_bytes());
        output[4..6].copy_from_slice(&self.version_number.to_le_bytes());
        output[6..8].copy_from_slice(&descriptor_block_size.to_le_bytes());

        output
    }

    /// Deserializes a block header from bytes, returning the header and the size
    /// of the containing [`Block`] (header + data).
    pub(crate) fn from_bytes(bytes: &[u8; Self::LENGTH]) -> Result<(Self, usize), ParseError> {
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

/// "Basic" DFD block, containing information about texture-like data.
///
/// This is the most common type of DFD block found in KTX2 files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Basic {
    /// The set of color (or other data) channels which may be encoded within the data,
    /// though there is no requirement that all of the possible channels from the colorModel
    /// be present.
    ///
    /// See the [DFD specification][dfd-spec] for more information on this than you'd ever need.
    ///
    /// None means unknown/unspecified.
    ///
    /// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#COLORMODEL
    pub color_model: Option<ColorModel>, //: 8;
    /// The color primaries used by the data.
    ///
    /// See the [DFD specification][dfd-spec] for more information than you can shake a stick at.
    ///
    /// None means unknown/unspecified.
    ///
    /// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#_emphasis_role_strong_emphasis_colorprimaries_emphasis_emphasis
    pub color_primaries: Option<ColorPrimaries>, //: 8;
    /// The function converting the encoded data to a linear color space.
    ///
    /// See the [DFD specification][dfd-spec] for more information.
    ///
    /// None means unknown/unspecified.
    ///
    /// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#_emphasis_role_strong_emphasis_transferfunction_emphasis_emphasis
    pub transfer_function: Option<TransferFunction>, //: 8;
    /// Boolean flags modifying properties of the data. In practice,
    /// this is only used to indicate if the alpha channel is premultiplied.
    ///
    /// See the [DFD specification][dfd-spec] for more information.
    ///
    /// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#_emphasis_role_strong_emphasis_flags_emphasis_emphasis
    pub flags: DataFormatFlags, //: 8;
    /// The dimensions of each "block" of texels in the image. For uncompressed formats, this is always 1x1x1x1.
    /// For compressed formats, this represents the dimensions of the compression block (e.g. 4x4x1x1 for BCn formats).
    ///
    /// The dfd stores this as one less than the actual dimension. See the [DFD specification][dfd-spec] for more information.
    ///
    /// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#_emphasis_role_strong_emphasis_texelblockdimension_0_3_emphasis_emphasis
    pub texel_block_dimensions: [NonZeroU8; 4], //: 8 x 4;
    /// The number of bytes in each plane of the data.
    ///
    /// See the [DFD specification][dfd-spec] for more information.
    ///
    /// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#_emphasis_role_strong_emphasis_bytesplane_0_7_emphasis_emphasis
    pub bytes_planes: [u8; 8], //: 8 x 8;
    /// Information about each "sample" within the data, describing how to interpret their bits of the data as color or other information.
    pub sample_information: Vec<SampleInformation>,
}

impl Basic {
    /// Number of bytes in the constant-size prefix of a Basic block, before the variable-length sample information.
    pub const FIXED_LENGTH: usize = 16;

    /// Creates a [`Basic`] DFD block for the given [`Format`](crate::Format),
    /// using the format's default transfer function, color primaries, color
    /// model, and straight (non-premultiplied) alpha.
    ///
    /// Returns the DFD block and the [`crate::Header::type_size`] that corresponds
    /// to the provided format.
    ///
    /// This is a convenience wrapper around [`from_format_with`](Self::from_format_with)
    /// that uses the standard defaults for every parameter. If you need to
    /// customize any of these, use `from_format_with` instead.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError::UnsupportedFormat`] if the format is not
    /// recognized by the DFD generation table.
    pub fn from_format(format: crate::Format) -> Result<(Self, u32), BuildError> {
        Self::from_format_with(format, false, None, None, None)
    }

    /// Creates a [`Basic`] DFD block for the given [`Format`](crate::Format),
    /// with optional overrides for transfer function, color primaries, color
    /// model, and alpha premultiplication.
    ///
    /// Returns the DFD block and the [`crate::Header::type_size`] that corresponds
    /// to the provided format and overrides.
    ///
    /// Parameters set to `None` use the format's natural defaults:
    ///
    /// - `alpha_premultiplied` — when `true`, sets the
    ///   [`ALPHA_PREMULTIPLIED`](DataFormatFlags::ALPHA_PREMULTIPLIED) flag,
    ///   indicating that color channel values have already been scaled by the
    ///   alpha channel. When `false` (the default), alpha is straight
    ///   (non-premultiplied).
    ///
    /// - `transfer_function` — overrides how encoded sample values are
    ///   converted to linear light. The default depends on the format: sRGB
    ///   format variants (e.g.
    ///   [`R8G8B8A8_SRGB`](crate::Format::R8G8B8A8_SRGB)) default to
    ///   [`TransferFunction::SRGB`]; all others default to
    ///   [`TransferFunction::Linear`]. When you override the default
    ///   transfer function, unlike with `srgb`-variant formats, the
    ///   "alpha" channel is not automatically marked as linear by DFD generation,
    ///   due to an ambiguity in the specification. See [this issue][ktx-spec-231].
    ///   However, conventionally, the alpha channel of these formats is still expected to be linear.
    ///
    /// - `color_primaries` — overrides the color primaries of the data.
    ///   Defaults to [`ColorPrimaries::BT709`] (the sRGB/Rec. 709 primaries
    ///   used by most consumer content).
    ///
    /// - `color_model` — overrides the color model. Defaults to
    ///   [`ColorModel::RGBSDA`] for most formats, [`ColorModel::YUVSDA`] for
    ///   4:2:2 subsampled formats, or the intrinsic model for compressed
    ///   formats. If this is changed from the default for the format,
    ///   the dfd will be formally invalid, but this may be useful for
    ///   some applications.
    ///
    /// # Format-specific restrictions
    ///
    /// Not all overrides are valid for all formats. The following restrictions
    /// are enforced and will return an error if violated:
    ///
    /// ## Depth-stencil formats (e.g. [`D16_UNORM_S8_UINT`](crate::Format::D16_UNORM_S8_UINT), [`D32_SFLOAT_S8_UINT`](crate::Format::D32_SFLOAT_S8_UINT))
    ///
    /// Depth-stencil formats have a fixed DFD layout. No overrides are
    /// permitted: `alpha_premultiplied` must be `false`, and
    /// `transfer_function`, `color_primaries`, and `color_model` must all be
    /// `None`.
    ///
    /// ## Compressed formats (e.g. [`BC7_UNORM_BLOCK`](crate::Format::BC7_UNORM_BLOCK), [`ASTC_4x4_SRGB_BLOCK`](crate::Format::ASTC_4x4_SRGB_BLOCK))
    ///
    /// Compressed formats must use their intrinsic color model (e.g.
    /// [`ColorModel::BC7`], [`ColorModel::ASTC`]). The `color_model` parameter
    /// must be `None`.
    ///
    /// ## sRGB variant rules
    ///
    /// Formats that exist in both UNORM and SRGB variants (e.g.
    /// [`R8G8B8A8_UNORM`](crate::Format::R8G8B8A8_UNORM) /
    /// [`R8G8B8A8_SRGB`](crate::Format::R8G8B8A8_SRGB),
    /// [`ASTC_4x4_UNORM_BLOCK`](crate::Format::ASTC_4x4_UNORM_BLOCK) /
    /// [`ASTC_4x4_SRGB_BLOCK`](crate::Format::ASTC_4x4_SRGB_BLOCK)) have
    /// strict transfer function requirements:
    ///
    /// - UNORM variant (e.g.
    ///   [`R8G8B8A8_UNORM`](crate::Format::R8G8B8A8_UNORM)): the transfer
    ///   function must **not** be set to [`TransferFunction::SRGB`]. If you
    ///   want sRGB encoding, use the SRGB variant of the format instead.
    ///
    /// - SRGB variant (e.g.
    ///   [`R8G8B8A8_SRGB`](crate::Format::R8G8B8A8_SRGB)): the transfer
    ///   function **must** be [`TransferFunction::SRGB`] (or `None` to use
    ///   the default). Overriding it to any other value is an error.
    ///
    /// Formats without an sRGB counterpart (e.g.
    /// [`R16_UNORM`](crate::Format::R16_UNORM),
    /// [`R32_SFLOAT`](crate::Format::R32_SFLOAT),
    /// [`ASTC_4x4_SFLOAT_BLOCK`](crate::Format::ASTC_4x4_SFLOAT_BLOCK))
    /// have no transfer function restrictions.
    ///
    /// # Errors
    ///
    /// Returns a [`BuildError`] describing which constraint was
    /// violated. See the variant documentation for details.
    ///
    /// [ktx-spec-231]: https://github.com/KhronosGroup/KTX-Specification/issues/231
    pub fn from_format_with(
        format: crate::Format,
        alpha_premultiplied: bool,
        transfer_function: Option<TransferFunction>,
        color_primaries: Option<ColorPrimaries>,
        color_model: Option<ColorModel>,
    ) -> Result<(Self, u32), BuildError> {
        let builder = generate::Builder::from_format(format).ok_or(BuildError::UnsupportedFormat)?;
        let type_size = builder.type_size();
        let dfd = builder.build(alpha_premultiplied, transfer_function, color_primaries, color_model)?;
        Ok((dfd, type_size))
    }

    /// Parses a [`Basic`] block from the start of `bytes`.
    pub fn parse(bytes: &[u8]) -> Result<Self, ParseError> {
        let mut offset = 0;

        let [model, primaries, transfer, flags] = read_bytes(bytes, &mut offset)?;
        let texel_block_dimensions = read_bytes(bytes, &mut offset)?.map(|dim| NonZeroU8::new(dim + 1).unwrap());
        let bytes_planes = read_bytes(bytes, &mut offset)?;

        let remaining_bytes = &bytes[Self::FIXED_LENGTH..];
        // TODO: If MSRV is bumped to 1.88, use as_chunks::<SampleInformation::LENGTH>().
        let iterator = remaining_bytes.chunks_exact(SampleInformation::LENGTH);

        if !iterator.remainder().is_empty() {
            return Err(ParseError::UnexpectedEnd);
        }

        let sample_information = iterator
            .map(|chunk| SampleInformation::from_bytes(chunk.try_into().unwrap()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            color_model: ColorModel::new(model),
            color_primaries: ColorPrimaries::new(primaries),
            transfer_function: TransferFunction::new(transfer),
            flags: DataFormatFlags::from_bits_truncate(flags),
            texel_block_dimensions,
            bytes_planes,
            sample_information,
        })
    }

    /// Number of bytes the serialized form of this block will take up.
    pub fn serialized_length(&self) -> usize {
        Self::FIXED_LENGTH + self.sample_information.len() * SampleInformation::LENGTH
    }

    /// Serializes this block to a given slice of bytes. The slice must be at least [`serialized_length`](Self::serialized_length) bytes long.
    pub fn to_bytes(&self, output: &mut [u8]) {
        assert!(
            output.len() >= self.serialized_length(),
            "Output buffer is too small to serialize Basic block: expected at least {} bytes, got {}",
            self.serialized_length(),
            output.len()
        );

        let color_model = self.color_model.map_or(0, |c| c.value());
        let color_primaries = self.color_primaries.map_or(0, |c| c.value());
        let transfer_function = self.transfer_function.map_or(0, |t| t.value());

        let texel_block_dimensions = self.texel_block_dimensions.map(|dim| dim.get() - 1);

        output[0] = color_model;
        output[1] = color_primaries;
        output[2] = transfer_function;
        output[3] = self.flags.bits();
        output[4..8].copy_from_slice(&texel_block_dimensions);
        output[8..16].copy_from_slice(&self.bytes_planes);
        for (i, sample) in self.sample_information.iter().enumerate() {
            let start = Self::FIXED_LENGTH + i * SampleInformation::LENGTH;
            output[start..][..SampleInformation::LENGTH].copy_from_slice(&sample.as_bytes());
        }
    }

    /// Serializes this block to a vector of bytes.
    pub fn to_vec(&self) -> Vec<u8> {
        let mut output = vec![0u8; self.serialized_length()];
        self.to_bytes(&mut output);
        output
    }
}

/// Information about each "sample" within an image.
///
/// A "sample" consisting of a single channel of data and with a
/// single corresponding "position" within the texel block.
///
/// See the [DFD specification][dfd-spec] for more extremely verbose information.
///
/// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#_anchor_id_sample_xreflabel_sample_sample_information
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SampleInformation {
    /// Offset from the beginning of the texel block in bits.
    pub bit_offset: u16, //: 16;
    /// The length of this sample in bits.
    pub bit_length: NonZeroU8, //: 8;
    /// The type of channel this sample represents. This varies by [`ColorModel`].
    pub channel_type: u8, //: 4;
    /// Qualifiers modifying the channel type.
    pub channel_type_qualifiers: ChannelTypeQualifiers, //: 4;
    /// The position in texels of this sample relative to the 0,0,0,0 texel in the 4D texel block.
    pub sample_positions: [u8; 4], //: 8 x 4;
    /// The sample value that maps to the format's logical minimum — typically `0` for unsigned
    /// formats, `-1` for signed formats, or `-0.5` for chroma channels in color difference models
    /// (e.g. Y′CbCr).
    ///
    /// Together with [`upper`](Self::upper), this defines how raw sample values are converted to
    /// their conceptual numeric interpretation. Values are not guaranteed to fall within this
    /// range — for example, HDR formats may define `1.0` as a nominal level well below the actual
    /// maximum. When samples should be interpreted directly as integers (unnormalized), set
    /// [`upper`](Self::upper) to `1` and `lower` to `0` (unsigned) or `-1` (signed).
    ///
    /// For integer formats, this is stored as a 32-bit integer (signed or unsigned matching the
    /// channel encoding). For floating-point formats, it is stored as a 32-bit float. For formats
    /// wider than 32 bits (e.g. 64-bit), integer values are expanded by preserving the sign bit
    /// and replicating the top non-sign bit, and float values are converted to the native
    /// representation (e.g. `f32` to `f64`).
    ///
    /// # Examples
    ///
    /// | Format | `lower` | `upper` | Effect |
    /// |--------|---------|---------|--------|
    /// | R8 unorm | `0` | `255` | Maps 0–255 to 0.0–1.0 |
    /// | R8 snorm | `-127` (`0xFFFFFF81`) | `127` | Maps -127–127 to -1.0–1.0 |
    /// | R8 uint | `0` | `1` | Integer value used directly |
    /// | R16 sfloat | `0xBF800000` (-1.0f) | `0x3F800000` (1.0f) | Float range -1.0–1.0 |
    /// | R64 uint | `0` | `1` | Expands to 64-bit `0` and `1` |
    /// | R64 uint norm | `0` | `0xFFFFFFFF` | Expands to `u64::MAX`, maps to 0.0–1.0 |
    /// | BT.709 Y′ (8-bit) | `16` | `235` | Maps 16–235 to 0.0–1.0 |
    ///
    /// For a very long and confusing explanation of this, please see the [DFD specification][dfd-spec]
    ///
    /// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#_emphasis_role_strong_emphasis_samplelower_emphasis_emphasis_and_emphasis_role_strong_emphasis_sampleupper_emphasis_emphasis
    pub lower: u32, //: 32;
    /// The sample value that maps to `1.0` (the white point), or `0.5` for chroma channels in
    /// color difference models (e.g. Y′CbCr).
    ///
    /// See [`lower`](Self::lower) for more details on interpretation and encoding and examples.
    pub upper: u32, //: 32;
}

impl SampleInformation {
    /// Number of bytes in a SampleInformation entry.
    pub const LENGTH: usize = 16;

    /// Serializes this sample information to bytes.
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

    /// Deserializes sample information from the given bytes.
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
    /// Qualifiers modifying the channel type of a [`SampleInformation`] entry.
    ///
    /// Multiple samples with the same channel and position are combined into a single logical
    /// ("virtual") sample. Samples without [`EXPONENT`](Self::EXPONENT) set contribute to the
    /// **base value** — the primary numeric content of the channel (e.g. a color intensity or
    /// mantissa). Samples with [`EXPONENT`](Self::EXPONENT) set act as **modifiers** that
    /// transform the base value — the interpretation depends on the combination of flags:
    ///
    /// | `EXPONENT` | `LINEAR` | `FLOAT` | Interpretation |
    /// |:----------:|:--------:|:-------:|----------------|
    /// | - | - | - | Base value, modified by [`TransferFunction`] |
    /// | - | L | - | Base value, always linear (ignores [`TransferFunction`]) |
    /// | - | - | F | Base value is a standard float (10/11/16/32/64-bit) |
    /// | E | - | - | Exponent: `base × 2^modifier` |
    /// | E | - | F | Multiplier: `base × modifier` |
    /// | E | L | - | Divisor: `base / modifier` |
    /// | E | L | F | Power: `base ^ modifier` |
    ///
    /// For a long and more confusing explanation of this, see the [DFD specification][dfd-spec].
    ///
    /// [dfd-spec]: https://registry.khronos.org/DataFormat/specs/1.4/dataformat.1.4.inline.html#_sample_emphasis_role_strong_emphasis_channeltype_emphasis_emphasis_emphasis_role_strong_emphasis_channelid_emphasis_emphasis_and_qualifiers
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct ChannelTypeQualifiers: u8 {
        /// The sample contains a linearly-encoded value, bypassing the format's
        /// [`TransferFunction`]. When combined with [`EXPONENT`](Self::EXPONENT), indicates a
        /// divisor modifier instead.
        const LINEAR        = (1 << 0);
        /// The sample is a modifier applied to the base value rather than part of the base value
        /// itself. The type of modification depends on the [`LINEAR`](Self::LINEAR) and
        /// [`FLOAT`](Self::FLOAT) flags — see the table on [`ChannelTypeQualifiers`].
        const EXPONENT      = (1 << 1);
        /// The sample holds a signed two's complement value. When not set, the sample is unsigned.
        const SIGNED        = (1 << 2);
        /// The sample holds floating-point data (10/11/16/32/64-bit IEEE 754). For custom float
        /// formats with a separate [`EXPONENT`](Self::EXPONENT) sample, this flag on the base
        /// value instead indicates an implicit leading `1` bit in the mantissa.
        /// When combined with [`EXPONENT`](Self::EXPONENT), indicates a multiplier modifier.
        const FLOAT         = (1 << 3);
    }
}

bitflags::bitflags! {
    /// Flags modifying the interpretation of color data in a [`Basic`] block.
    ///
    /// Controls whether color values have been pre-scaled by the alpha channel. Has no effect
    /// if the format does not contain an alpha channel.
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct DataFormatFlags: u8 {
        /// Color values are not premultiplied — they need to be scaled by alpha during blending.
        ///
        /// Not a flag itself, but an alias for when ALPHA_PREMULTIPLIED is not set.
        const STRAIGHT_ALPHA             = 0;
        /// Color values have already been scaled by the alpha channel.
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
        let basic = Basic {
            color_model: Some(ColorModel::LabSDA),
            color_primaries: Some(ColorPrimaries::ACES),
            transfer_function: Some(TransferFunction::ITU),
            flags: DataFormatFlags::STRAIGHT_ALPHA,
            texel_block_dimensions: to_nonzero([1, 2, 3, 4]),
            bytes_planes: [5, 6, 7, 8, 9, 10, 11, 12],
            sample_information: vec![],
        };

        let bytes = basic.to_vec();
        let decoded = Basic::parse(&bytes).unwrap();
        assert_eq!(basic, decoded);
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
