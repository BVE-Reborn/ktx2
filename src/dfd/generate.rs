use alloc::vec::Vec;
use core::num::NonZeroU8;

use crate::dfd::{Basic, ChannelTypeQualifiers, DataFormatFlags, SampleInformation};
use crate::{ColorModel, ColorPrimaries, Format, TransferFunction};

// RGBSDA channel IDs (from the Khronos Data Format Specification).
const CHANNEL_R: u8 = 0;
const CHANNEL_G: u8 = 1;
const CHANNEL_B: u8 = 2;
const CHANNEL_STENCIL: u8 = 13;
const CHANNEL_DEPTH: u8 = 14;
const CHANNEL_ALPHA: u8 = 15;

// YUVSDA channel IDs (same numeric namespace, different color model).
const CHANNEL_Y: u8 = 0;
const CHANNEL_U: u8 = 1;
const CHANNEL_V: u8 = 2;

// BCn compressed channel IDs.
// Channel 0 is COLOR for BC1A–BC3, BC4, BC6H, BC7;
const BC_COLOR: u8 = 0;
// Single bit alpha channel for BC1.
const BC1A_ALPHA: u8 = 1;
const BC5_RED: u8 = 0;
const BC5_GREEN: u8 = 1;
const BC_ALPHA: u8 = 15;

// ETC2/EAC compressed channel IDs.
const ETC2_RED: u8 = 0;
const ETC2_GREEN: u8 = 1;
const ETC2_COLOR: u8 = 2;
const ETC2_ALPHA: u8 = 15;

// ASTC compressed channel IDs.
const ASTC_DATA: u8 = 0;

// PVRTC compressed channel IDs.
const PVRTC_COLOR: u8 = 0;

// E5B9G9R9 shared-exponent format constants.
const RGB9E5_MANTISSA_BITS: u8 = 9;
const RGB9E5_EXPONENT_BITS: u8 = 5;
const RGB9E5_EXPONENT_OFFSET: u16 = 27;
const RGB9E5_EXPONENT_BIAS: u32 = 15;
const RGB9E5_EXPONENT_MAX: u32 = (1 << RGB9E5_EXPONENT_BITS) - 1;
// The upper bound the KTX reference validator expects for the 9-bit mantissa,
// derived from the shared-exponent encoding where the full-range mantissa
// maps through the exponent bias.
const RGB9E5_MANTISSA_UPPER: u32 = 8448;

// DFD sample positions are in 1/256 of the texel block dimension.
const SAMPLE_POS_ORIGIN: [u8; 4] = [0, 0, 0, 0];
// 0.5 in 1/256 texel-block coordinates; for a 2x1 block, y=128 means the
// center of the single pixel row.
const HALF_TEXEL: u8 = 128;

/// The numeric interpretation of sample data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Datatype {
    /// Unsigned normalized integer, mapped to `[0.0, 1.0]`.
    Unorm,
    /// Signed normalized integer, mapped to `[-1.0, 1.0]`.
    Snorm,
    /// Unsigned integer, not normalized.
    Uint,
    /// Signed integer, not normalized.
    Sint,
    /// Signed floating-point.
    Sfloat,
    /// Unsigned floating-point.
    Ufloat,
    /// Signed fixed-point with 5 fractional bits.
    Sfixed5,
}

/// The order of chroma samples in a 4:2:2 subsampled format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromaSubsamplingSampleOrder {
    /// GBGR memory order (e.g. `G8B8G8R8_422_UNORM`).
    Gbgr,
    /// BGRG memory order (e.g. `B8G8R8G8_422_UNORM`).
    Bgrg,
}

/// The transfer function inherent to a format, if any.
///
/// Formats with an sRGB counterpart have strict transfer function requirements:
/// the UNORM variant must NOT use sRGB transfer (use the SRGB variant instead),
/// and the SRGB variant MUST use sRGB transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FormatInherentTransferFunction {
    /// No sRGB relationship. Transfer function defaults to linear and may be
    /// overridden freely.
    Linear,
    /// This is the UNORM variant of a format that also has a dedicated SRGB
    /// variant. The transfer function must NOT be sRGB — use the SRGB variant
    /// of the format instead.
    LinearWithSrgbCounterpart,
    /// This IS the sRGB variant. The transfer function MUST be sRGB.
    Srgb,
}

/// Error type for DFD generation and validation.
#[derive(Debug)]
#[non_exhaustive]
pub enum BuildError {
    /// The [`Format`] is not recognized by the DFD generation table.
    UnsupportedFormat,
    /// Premultiplied alpha was requested for a depth-stencil format, which has
    /// no alpha channel.
    DepthStencilPremultipliedAlpha,
    /// A transfer function override was specified for a depth-stencil format.
    /// Depth-stencil formats always use linear transfer.
    DepthStencilTransferFunction,
    /// A color primaries override was specified for a depth-stencil format.
    /// Depth-stencil formats always use BT.709 primaries.
    DepthStencilColorPrimaries,
    /// A color model override was specified for a depth-stencil format.
    /// Depth-stencil formats always use RGBSDA.
    DepthStencilColorModel,
    /// A color model override was specified for a compressed format. Compressed
    /// formats must use their intrinsic color model.
    CompressedColorModel,
    /// The transfer function was set to sRGB, but this format has a dedicated
    /// sRGB variant. Use the sRGB variant of the format instead.
    SrgbTransferNotAllowed,
    /// The transfer function was overridden to a non-sRGB value, but this
    /// format is an sRGB variant and must use sRGB transfer.
    SrgbTransferRequired,
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str = match self {
            Self::UnsupportedFormat => "format is not recognized by the DFD generation table",
            Self::DepthStencilPremultipliedAlpha => {
                "premultiplied alpha is not supported for depth-stencil formats (no alpha channel)"
            }
            Self::DepthStencilTransferFunction => {
                "transfer function override is not supported for depth-stencil formats (always linear)"
            }
            Self::DepthStencilColorPrimaries => {
                "color primaries override is not supported for depth-stencil formats (always BT.709)"
            }
            Self::DepthStencilColorModel => {
                "color model override is not supported for depth-stencil formats (always RGBSDA)"
            }
            Self::CompressedColorModel => {
                "color model override is not supported for compressed formats (must use intrinsic model)"
            }
            Self::SrgbTransferNotAllowed => {
                "sRGB transfer function is not allowed for this format; use the dedicated sRGB variant instead"
            }
            Self::SrgbTransferRequired => "this format is an sRGB variant and must use sRGB transfer function",
        };

        f.pad(str)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BuildError {}

/// Describes how to build a DFD for a given format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Builder {
    /// Standard uncompressed or simple format.
    Standard {
        /// How sample values are interpreted numerically.
        datatype: Datatype,
        /// Whether the format uses sRGB transfer.
        srgb: FormatInherentTransferFunction,
        /// Number of bytes per texel.
        bytes_per_texel: u8,
        /// Size in bytes of the underlying data type for endian conversion.
        ///
        /// - **Component formats** (R8G8B8A8, R16G16, etc.): individual
        ///   component size in bytes.
        /// - **nPACK16 formats** (R10X6G10X6_2PACK16, etc.): 16-bit word
        ///   size (2), not the total texel size.
        /// - **Single-word pack formats** (R4G4_PACK8, R5G6B5_PACK16,
        ///   A8B8G8R8_PACK32): bytes_per_texel (the entire pack unit).
        type_size: u8,
        /// Bit width of each channel, in ascending bit-offset order.
        bit_count: &'static [u8],
        /// Bit offset of each channel within the texel, in ascending order.
        bit_offset: &'static [u8],
        /// RGBSDA channel ID for each sample, in ascending bit-offset order.
        channel_ids: &'static [u8],
    },
    /// Combined depth-stencil format with mixed datatypes per channel.
    DepthStencil {
        /// Bit width of the depth channel (16, 24, or 32).
        depth_bits: u8,
        /// Numeric interpretation of the depth channel.
        depth_datatype: Datatype,
    },
    /// Shared-exponent format (`E5B9G9R9_UFLOAT_PACK32`).
    Rgb9e5,
    /// Compressed block format (BCn, ETC2, EAC, ASTC, PVRTC).
    Compressed {
        /// Compressed-format color model (e.g. `BC1A`, `ETC2`, `ASTC`).
        color_model: ColorModel,
        /// Whether the format uses sRGB transfer.
        srgb: FormatInherentTransferFunction,
        /// Texel block dimensions `[width, height, depth]`.
        block_dimensions: [u8; 3],
        /// Bytes per compressed block.
        bytes_per_block: u8,
        /// Numeric interpretation of the compressed data.
        datatype: Datatype,
        /// Channel types for each sample. One entry = single sample covering the
        /// whole block. Two entries = two 64-bit halves (first at bit offset 0,
        /// second at bit offset 64).
        channel_types: &'static [u8],
    },
    /// 4:2:2 horizontally subsampled format.
    Subsampled422 {
        /// The order of chroma samples in memory.
        sample_order: ChromaSubsamplingSampleOrder,
        /// Significant bit width per channel (8, 10, 12, or 16).
        bit_width: u8,
    },
}

impl Builder {
    /// Returns the `type_size` value for the KTX2 header.
    ///
    /// This is the size in bytes of the underlying data type, used for
    /// endian conversion on big-endian targets.
    pub fn type_size(&self) -> u32 {
        let type8 = match *self {
            Builder::Standard { type_size, .. } => type_size,
            Builder::Compressed { .. } => 1,
            // All three combined depth-stencil formats are single-plane
            // packed in KTX2; type_size = the depth component's native size.
            Builder::DepthStencil { depth_bits, .. } => match depth_bits {
                16 => 2,
                24 => 4, // X8_D24 packed into 32 bits
                32 => 4,
                _ => unreachable!("unsupported depth bit width: {depth_bits}"),
            },
            Builder::Rgb9e5 => 4,                                            // PACK32
            Builder::Subsampled422 { bit_width, .. } => (bit_width + 7) / 8, // Round up to whole bytes.
        };
        type8 as u32
    }

    /// Returns the [`Builder`] for a given [`Format`], or `None` if the
    /// format is unknown.
    ///
    /// All `Standard` entries have `bit_offset`, `bit_count`, and `channel_ids`
    /// in ascending bit-offset order, matching the DFD spec's sample ordering
    /// requirement.
    ///
    /// NOTE: If new variants are added to [`Format`] in `enums.rs`, a
    /// corresponding match arm must be added here.
    pub fn from_format(format: Format) -> Option<Builder> {
        use ColorModel as Cm;
        use Datatype as Dt;
        use Format as F;
        use FormatInherentTransferFunction::{Linear, LinearWithSrgbCounterpart as Counterpart, Srgb};

        const R: u8 = CHANNEL_R;
        const G: u8 = CHANNEL_G;
        const B: u8 = CHANNEL_B;
        const A: u8 = CHANNEL_ALPHA;
        const D: u8 = CHANNEL_DEPTH;
        const S: u8 = CHANNEL_STENCIL;

        fn s(
            datatype: Datatype,
            srgb: FormatInherentTransferFunction,
            bytes_per_texel: u8,
            type_size: u8,
            bit_count: &'static [u8],
            bit_offset: &'static [u8],
            channel_ids: &'static [u8],
        ) -> Builder {
            Builder::Standard {
                datatype,
                srgb,
                bytes_per_texel,
                type_size,
                bit_count,
                bit_offset,
                channel_ids,
            }
        }

        fn ds(depth_bits: u8, depth_datatype: Datatype) -> Builder {
            Builder::DepthStencil {
                depth_bits,
                depth_datatype,
            }
        }

        fn c422(sample_order: ChromaSubsamplingSampleOrder, bit_width: u8) -> Builder {
            Builder::Subsampled422 {
                sample_order,
                bit_width,
            }
        }

        #[allow(clippy::too_many_arguments)]
        fn c(
            color_model: ColorModel,
            srgb: FormatInherentTransferFunction,
            block_dimensions: [u8; 3],
            bytes_per_block: u8,
            datatype: Datatype,
            channel_types: &'static [u8],
        ) -> Builder {
            Builder::Compressed {
                color_model,
                srgb,
                block_dimensions,
                bytes_per_block,
                datatype,
                channel_types,
            }
        }

        // All Standard entries below list (bit_count, bit_offset, channel_ids)
        // in ascending bit-offset order so that build_basic() can emit samples
        // directly without a post-hoc sort.
        //
        // NOTE: If new arms are added here, the corresponding variant must exist
        // in [`Format`] in `enums.rs`.
        #[rustfmt::skip]
        let res = match format {
            // ---- Packed 8-bit (MSB→LSB: R4, G4) ----
            F::R4G4_UNORM_PACK8                   => s(Dt::Unorm,   Linear,      1,  1, &[4, 4],           &[0, 4],            &[G, R]),

            // ---- Packed 16-bit ----
            // MSB→LSB: R4, G4, B4, A4 → ascending: A@0, B@4, G@8, R@12
            F::R4G4B4A4_UNORM_PACK16              => s(Dt::Unorm,   Linear,      2,  2, &[4, 4, 4, 4],     &[0, 4, 8, 12],     &[A, B, G, R]),
            F::B4G4R4A4_UNORM_PACK16              => s(Dt::Unorm,   Linear,      2,  2, &[4, 4, 4, 4],     &[0, 4, 8, 12],     &[A, R, G, B]),
            F::R5G6B5_UNORM_PACK16                => s(Dt::Unorm,   Linear,      2,  2, &[5, 6, 5],        &[0, 5, 11],        &[B, G, R]),
            F::B5G6R5_UNORM_PACK16                => s(Dt::Unorm,   Linear,      2,  2, &[5, 6, 5],        &[0, 5, 11],        &[R, G, B]),
            F::R5G5B5A1_UNORM_PACK16              => s(Dt::Unorm,   Linear,      2,  2, &[1, 5, 5, 5],     &[0, 1, 6, 11],     &[A, B, G, R]),
            F::B5G5R5A1_UNORM_PACK16              => s(Dt::Unorm,   Linear,      2,  2, &[1, 5, 5, 5],     &[0, 1, 6, 11],     &[A, R, G, B]),
            F::A1R5G5B5_UNORM_PACK16              => s(Dt::Unorm,   Linear,      2,  2, &[5, 5, 5, 1],     &[0, 5, 10, 15],    &[B, G, R, A]),

            // ---- R8 ----
            F::R8_UNORM                           => s(Dt::Unorm,   Counterpart, 1,  1, &[8],              &[0],               &[R]),
            F::R8_SNORM                           => s(Dt::Snorm,   Linear,      1,  1, &[8],              &[0],               &[R]),
            F::R8_UINT                            => s(Dt::Uint,    Linear,      1,  1, &[8],              &[0],               &[R]),
            F::R8_SINT                            => s(Dt::Sint,    Linear,      1,  1, &[8],              &[0],               &[R]),
            F::R8_SRGB                            => s(Dt::Unorm,   Srgb,        1,  1, &[8],              &[0],               &[R]),

            // ---- R8G8 ----
            F::R8G8_UNORM                         => s(Dt::Unorm,   Counterpart, 2,  1, &[8, 8],           &[0, 8],            &[R, G]),
            F::R8G8_SNORM                         => s(Dt::Snorm,   Linear,      2,  1, &[8, 8],           &[0, 8],            &[R, G]),
            F::R8G8_UINT                          => s(Dt::Uint,    Linear,      2,  1, &[8, 8],           &[0, 8],            &[R, G]),
            F::R8G8_SINT                          => s(Dt::Sint,    Linear,      2,  1, &[8, 8],           &[0, 8],            &[R, G]),
            F::R8G8_SRGB                          => s(Dt::Unorm,   Srgb,        2,  1, &[8, 8],           &[0, 8],            &[R, G]),

            // ---- R8G8B8 ----
            F::R8G8B8_UNORM                       => s(Dt::Unorm,   Counterpart, 3,  1, &[8, 8, 8],        &[0, 8, 16],        &[R, G, B]),
            F::R8G8B8_SNORM                       => s(Dt::Snorm,   Linear,      3,  1, &[8, 8, 8],        &[0, 8, 16],        &[R, G, B]),
            F::R8G8B8_UINT                        => s(Dt::Uint,    Linear,      3,  1, &[8, 8, 8],        &[0, 8, 16],        &[R, G, B]),
            F::R8G8B8_SINT                        => s(Dt::Sint,    Linear,      3,  1, &[8, 8, 8],        &[0, 8, 16],        &[R, G, B]),
            F::R8G8B8_SRGB                        => s(Dt::Unorm,   Srgb,        3,  1, &[8, 8, 8],        &[0, 8, 16],        &[R, G, B]),

            // ---- B8G8R8 ----
            F::B8G8R8_UNORM                       => s(Dt::Unorm,   Counterpart, 3,  1, &[8, 8, 8],        &[0, 8, 16],        &[B, G, R]),
            F::B8G8R8_SNORM                       => s(Dt::Snorm,   Linear,      3,  1, &[8, 8, 8],        &[0, 8, 16],        &[B, G, R]),
            F::B8G8R8_UINT                        => s(Dt::Uint,    Linear,      3,  1, &[8, 8, 8],        &[0, 8, 16],        &[B, G, R]),
            F::B8G8R8_SINT                        => s(Dt::Sint,    Linear,      3,  1, &[8, 8, 8],        &[0, 8, 16],        &[B, G, R]),
            F::B8G8R8_SRGB                        => s(Dt::Unorm,   Srgb,        3,  1, &[8, 8, 8],        &[0, 8, 16],        &[B, G, R]),

            // ---- R8G8B8A8 ----
            F::R8G8B8A8_UNORM                     => s(Dt::Unorm,   Counterpart, 4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),
            F::R8G8B8A8_SNORM                     => s(Dt::Snorm,   Linear,      4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),
            F::R8G8B8A8_UINT                      => s(Dt::Uint,    Linear,      4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),
            F::R8G8B8A8_SINT                      => s(Dt::Sint,    Linear,      4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),
            F::R8G8B8A8_SRGB                      => s(Dt::Unorm,   Srgb,        4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),

            // ---- B8G8R8A8 ----
            F::B8G8R8A8_UNORM                     => s(Dt::Unorm,   Counterpart, 4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[B, G, R, A]),
            F::B8G8R8A8_SNORM                     => s(Dt::Snorm,   Linear,      4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[B, G, R, A]),
            F::B8G8R8A8_UINT                      => s(Dt::Uint,    Linear,      4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[B, G, R, A]),
            F::B8G8R8A8_SINT                      => s(Dt::Sint,    Linear,      4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[B, G, R, A]),
            F::B8G8R8A8_SRGB                      => s(Dt::Unorm,   Srgb,        4,  1, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[B, G, R, A]),

            // ---- A8B8G8R8 packed 32-bit (MSB→LSB: A8, B8, G8, R8) ----
            F::A8B8G8R8_UNORM_PACK32              => s(Dt::Unorm,   Counterpart, 4,  4, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),
            F::A8B8G8R8_SNORM_PACK32              => s(Dt::Snorm,   Linear,      4,  4, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),
            F::A8B8G8R8_UINT_PACK32               => s(Dt::Uint,    Linear,      4,  4, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),
            F::A8B8G8R8_SINT_PACK32               => s(Dt::Sint,    Linear,      4,  4, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),
            F::A8B8G8R8_SRGB_PACK32               => s(Dt::Unorm,   Srgb,        4,  4, &[8, 8, 8, 8],     &[0, 8, 16, 24],    &[R, G, B, A]),

            // ---- A2R10G10B10 packed 32-bit (MSB→LSB: A2, R10, G10, B10) ----
            F::A2R10G10B10_UNORM_PACK32           => s(Dt::Unorm,   Linear,      4,  4, &[10, 10, 10, 2],  &[0, 10, 20, 30],   &[B, G, R, A]),
            F::A2R10G10B10_SNORM_PACK32           => s(Dt::Snorm,   Linear,      4,  4, &[10, 10, 10, 2],  &[0, 10, 20, 30],   &[B, G, R, A]),
            F::A2R10G10B10_UINT_PACK32            => s(Dt::Uint,    Linear,      4,  4, &[10, 10, 10, 2],  &[0, 10, 20, 30],   &[B, G, R, A]),
            F::A2R10G10B10_SINT_PACK32            => s(Dt::Sint,    Linear,      4,  4, &[10, 10, 10, 2],  &[0, 10, 20, 30],   &[B, G, R, A]),

            // ---- A2B10G10R10 packed 32-bit (MSB→LSB: A2, B10, G10, R10) ----
            F::A2B10G10R10_UNORM_PACK32           => s(Dt::Unorm,   Linear,      4,  4, &[10, 10, 10, 2],  &[0, 10, 20, 30],   &[R, G, B, A]),
            F::A2B10G10R10_SNORM_PACK32           => s(Dt::Snorm,   Linear,      4,  4, &[10, 10, 10, 2],  &[0, 10, 20, 30],   &[R, G, B, A]),
            F::A2B10G10R10_UINT_PACK32            => s(Dt::Uint,    Linear,      4,  4, &[10, 10, 10, 2],  &[0, 10, 20, 30],   &[R, G, B, A]),
            F::A2B10G10R10_SINT_PACK32            => s(Dt::Sint,    Linear,      4,  4, &[10, 10, 10, 2],  &[0, 10, 20, 30],   &[R, G, B, A]),

            // ---- R16 ----
            F::R16_UNORM                          => s(Dt::Unorm,   Linear,      2,  2, &[16],             &[0],               &[R]),
            F::R16_SNORM                          => s(Dt::Snorm,   Linear,      2,  2, &[16],             &[0],               &[R]),
            F::R16_UINT                           => s(Dt::Uint,    Linear,      2,  2, &[16],             &[0],               &[R]),
            F::R16_SINT                           => s(Dt::Sint,    Linear,      2,  2, &[16],             &[0],               &[R]),
            F::R16_SFLOAT                         => s(Dt::Sfloat,  Linear,      2,  2, &[16],             &[0],               &[R]),

            // ---- R16G16 ----
            F::R16G16_UNORM                       => s(Dt::Unorm,   Linear,      4,  2, &[16, 16],         &[0, 16],           &[R, G]),
            F::R16G16_SNORM                       => s(Dt::Snorm,   Linear,      4,  2, &[16, 16],         &[0, 16],           &[R, G]),
            F::R16G16_UINT                        => s(Dt::Uint,    Linear,      4,  2, &[16, 16],         &[0, 16],           &[R, G]),
            F::R16G16_SINT                        => s(Dt::Sint,    Linear,      4,  2, &[16, 16],         &[0, 16],           &[R, G]),
            F::R16G16_SFLOAT                      => s(Dt::Sfloat,  Linear,      4,  2, &[16, 16],         &[0, 16],           &[R, G]),

            // ---- R16G16B16 ----
            F::R16G16B16_UNORM                    => s(Dt::Unorm,   Linear,      6,  2, &[16, 16, 16],     &[0, 16, 32],       &[R, G, B]),
            F::R16G16B16_SNORM                    => s(Dt::Snorm,   Linear,      6,  2, &[16, 16, 16],     &[0, 16, 32],       &[R, G, B]),
            F::R16G16B16_UINT                     => s(Dt::Uint,    Linear,      6,  2, &[16, 16, 16],     &[0, 16, 32],       &[R, G, B]),
            F::R16G16B16_SINT                     => s(Dt::Sint,    Linear,      6,  2, &[16, 16, 16],     &[0, 16, 32],       &[R, G, B]),
            F::R16G16B16_SFLOAT                   => s(Dt::Sfloat,  Linear,      6,  2, &[16, 16, 16],     &[0, 16, 32],       &[R, G, B]),

            // ---- R16G16B16A16 ----
            F::R16G16B16A16_UNORM                 => s(Dt::Unorm,   Linear,      8,  2, &[16, 16, 16, 16], &[0, 16, 32, 48],   &[R, G, B, A]),
            F::R16G16B16A16_SNORM                 => s(Dt::Snorm,   Linear,      8,  2, &[16, 16, 16, 16], &[0, 16, 32, 48],   &[R, G, B, A]),
            F::R16G16B16A16_UINT                  => s(Dt::Uint,    Linear,      8,  2, &[16, 16, 16, 16], &[0, 16, 32, 48],   &[R, G, B, A]),
            F::R16G16B16A16_SINT                  => s(Dt::Sint,    Linear,      8,  2, &[16, 16, 16, 16], &[0, 16, 32, 48],   &[R, G, B, A]),
            F::R16G16B16A16_SFLOAT                => s(Dt::Sfloat,  Linear,      8,  2, &[16, 16, 16, 16], &[0, 16, 32, 48],   &[R, G, B, A]),

            // ---- R32 ----
            F::R32_UINT                           => s(Dt::Uint,    Linear,      4,  4, &[32],             &[0],               &[R]),
            F::R32_SINT                           => s(Dt::Sint,    Linear,      4,  4, &[32],             &[0],               &[R]),
            F::R32_SFLOAT                         => s(Dt::Sfloat,  Linear,      4,  4, &[32],             &[0],               &[R]),

            // ---- R32G32 ----
            F::R32G32_UINT                        => s(Dt::Uint,    Linear,      8,  4, &[32, 32],         &[0, 32],           &[R, G]),
            F::R32G32_SINT                        => s(Dt::Sint,    Linear,      8,  4, &[32, 32],         &[0, 32],           &[R, G]),
            F::R32G32_SFLOAT                      => s(Dt::Sfloat,  Linear,      8,  4, &[32, 32],         &[0, 32],           &[R, G]),

            // ---- R32G32B32 ----
            F::R32G32B32_UINT                     => s(Dt::Uint,    Linear,      12, 4, &[32, 32, 32],     &[0, 32, 64],       &[R, G, B]),
            F::R32G32B32_SINT                     => s(Dt::Sint,    Linear,      12, 4, &[32, 32, 32],     &[0, 32, 64],       &[R, G, B]),
            F::R32G32B32_SFLOAT                   => s(Dt::Sfloat,  Linear,      12, 4, &[32, 32, 32],     &[0, 32, 64],       &[R, G, B]),

            // ---- R32G32B32A32 ----
            F::R32G32B32A32_UINT                  => s(Dt::Uint,    Linear,      16, 4, &[32, 32, 32, 32], &[0, 32, 64, 96],   &[R, G, B, A]),
            F::R32G32B32A32_SINT                  => s(Dt::Sint,    Linear,      16, 4, &[32, 32, 32, 32], &[0, 32, 64, 96],   &[R, G, B, A]),
            F::R32G32B32A32_SFLOAT                => s(Dt::Sfloat,  Linear,      16, 4, &[32, 32, 32, 32], &[0, 32, 64, 96],   &[R, G, B, A]),

            // ---- R64 ----
            F::R64_UINT                           => s(Dt::Uint,    Linear,      8,  8, &[64],             &[0],               &[R]),
            F::R64_SINT                           => s(Dt::Sint,    Linear,      8,  8, &[64],             &[0],               &[R]),
            F::R64_SFLOAT                         => s(Dt::Sfloat,  Linear,      8,  8, &[64],             &[0],               &[R]),

            // ---- R64G64 ----
            F::R64G64_UINT                        => s(Dt::Uint,    Linear,      16, 8, &[64, 64],         &[0, 64],           &[R, G]),
            F::R64G64_SINT                        => s(Dt::Sint,    Linear,      16, 8, &[64, 64],         &[0, 64],           &[R, G]),
            F::R64G64_SFLOAT                      => s(Dt::Sfloat,  Linear,      16, 8, &[64, 64],         &[0, 64],           &[R, G]),

            // ---- R64G64B64 ----
            F::R64G64B64_UINT                     => s(Dt::Uint,    Linear,      24, 8, &[64, 64, 64],     &[0, 64, 128],      &[R, G, B]),
            F::R64G64B64_SINT                     => s(Dt::Sint,    Linear,      24, 8, &[64, 64, 64],     &[0, 64, 128],      &[R, G, B]),
            F::R64G64B64_SFLOAT                   => s(Dt::Sfloat,  Linear,      24, 8, &[64, 64, 64],     &[0, 64, 128],      &[R, G, B]),

            // ---- R64G64B64A64 ----
            F::R64G64B64A64_UINT                  => s(Dt::Uint,    Linear,      32, 8, &[64, 64, 64, 64], &[0, 64, 128, 192], &[R, G, B, A]),
            F::R64G64B64A64_SINT                  => s(Dt::Sint,    Linear,      32, 8, &[64, 64, 64, 64], &[0, 64, 128, 192], &[R, G, B, A]),
            F::R64G64B64A64_SFLOAT                => s(Dt::Sfloat,  Linear,      32, 8, &[64, 64, 64, 64], &[0, 64, 128, 192], &[R, G, B, A]),

            // ---- Packed float (MSB→LSB: B10, G11, R11) ----
            F::B10G11R11_UFLOAT_PACK32            => s(Dt::Ufloat,  Linear,      4,  4, &[11, 11, 10],     &[0, 11, 22],       &[R, G, B]),

            // ---- Depth/stencil ----
            F::D16_UNORM                          => s(Dt::Unorm,   Linear,      2,  2, &[16],             &[0],               &[D]),
            F::X8_D24_UNORM_PACK32                => s(Dt::Unorm,   Linear,      4,  4, &[24],             &[0],               &[D]),
            F::D32_SFLOAT                         => s(Dt::Sfloat,  Linear,      4,  4, &[32],             &[0],               &[D]),
            F::S8_UINT                            => s(Dt::Uint,    Linear,      1,  1, &[8],              &[0],               &[S]),

            // ---- Extended packed 16-bit ----
            F::A4R4G4B4_UNORM_PACK16              => s(Dt::Unorm,   Linear,      2,  2, &[4, 4, 4, 4],     &[0, 4, 8, 12],     &[B, G, R, A]),
            F::A4B4G4R4_UNORM_PACK16              => s(Dt::Unorm,   Linear,      2,  2, &[4, 4, 4, 4],     &[0, 4, 8, 12],     &[R, G, B, A]),
            F::A1B5G5R5_UNORM_PACK16              => s(Dt::Unorm,   Linear,      2,  2, &[5, 5, 5, 1],     &[0, 5, 10, 15],    &[R, G, B, A]),

            // ---- Extended Fs ----
            F::R16G16_SFIXED5                     => s(Dt::Sfixed5, Linear,      4,  2, &[16, 16],         &[0, 16],           &[R, G]),
            F::A8_UNORM                           => s(Dt::Unorm,   Linear,      1,  1, &[8],              &[0],               &[A]),

            // ---- R10X6 unorm/uint ----
            // nPACK16 Fs are MSB-justified: the N-bit value sits in the
            // high bits of each 16-bit word, with (16-N) padding bits at the
            // bottom. So R10X6 has bit_offset = 16 - 10 = 6 within each word.
            F::R10X6_UNORM_PACK16                 => s(Dt::Unorm,   Linear,      2,  2, &[10],             &[6],               &[R]),
            F::R10X6G10X6_UNORM_2PACK16           => s(Dt::Unorm,   Linear,      4,  2, &[10, 10],         &[6, 22],           &[R, G]),
            F::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => s(Dt::Unorm,   Linear,      8,  2, &[10, 10, 10, 10], &[6, 22, 38, 54],   &[R, G, B, A]),
            F::R10X6_UINT_PACK16                  => s(Dt::Uint,    Linear,      2,  2, &[10],             &[6],               &[R]),
            F::R10X6G10X6_UINT_2PACK16            => s(Dt::Uint,    Linear,      4,  2, &[10, 10],         &[6, 22],           &[R, G]),
            F::R10X6G10X6B10X6A10X6_UINT_4PACK16  => s(Dt::Uint,    Linear,      8,  2, &[10, 10, 10, 10], &[6, 22, 38, 54],   &[R, G, B, A]),

            // ---- R12X4 unorm/uint (MSB-justified, offset = 16 - 12 = 4) ----
            F::R12X4_UNORM_PACK16                 => s(Dt::Unorm,   Linear,      2,  2, &[12],             &[4],               &[R]),
            F::R12X4G12X4_UNORM_2PACK16           => s(Dt::Unorm,   Linear,      4,  2, &[12, 12],         &[4, 20],           &[R, G]),
            F::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => s(Dt::Unorm,   Linear,      8,  2, &[12, 12, 12, 12], &[4, 20, 36, 52],   &[R, G, B, A]),
            F::R12X4_UINT_PACK16                  => s(Dt::Uint,    Linear,      2,  2, &[12],             &[4],               &[R]),
            F::R12X4G12X4_UINT_2PACK16            => s(Dt::Uint,    Linear,      4,  2, &[12, 12],         &[4, 20],           &[R, G]),
            F::R12X4G12X4B12X4A12X4_UINT_4PACK16  => s(Dt::Uint,    Linear,      8,  2, &[12, 12, 12, 12], &[4, 20, 36, 52],   &[R, G, B, A]),

            // ---- R14X2 unorm/uint (MSB-justified, offset = 16 - 14 = 2) ----
            F::R14X2_UNORM_PACK16                 => s(Dt::Unorm,   Linear,      2,  2, &[14],             &[2],               &[R]),
            F::R14X2G14X2_UNORM_2PACK16           => s(Dt::Unorm,   Linear,      4,  2, &[14, 14],         &[2, 18],           &[R, G]),
            F::R14X2G14X2B14X2A14X2_UNORM_4PACK16 => s(Dt::Unorm,   Linear,      8,  2, &[14, 14, 14, 14], &[2, 18, 34, 50],   &[R, G, B, A]),
            F::R14X2_UINT_PACK16                  => s(Dt::Uint,    Linear,      2,  2, &[14],             &[2],               &[R]),
            F::R14X2G14X2_UINT_2PACK16            => s(Dt::Uint,    Linear,      4,  2, &[14, 14],         &[2, 18],           &[R, G]),
            F::R14X2G14X2B14X2A14X2_UINT_4PACK16  => s(Dt::Uint,    Linear,      8,  2, &[14, 14, 14, 14], &[2, 18, 34, 50],   &[R, G, B, A]),

            // ---- Shared-exponent ----
            F::E5B9G9R9_UFLOAT_PACK32 => Builder::Rgb9e5,

            // ---- Combined depth-stencil ----
            //                           ds(bits, datatype)
            F::D16_UNORM_S8_UINT  => ds(16, Dt::Unorm),
            F::D24_UNORM_S8_UINT  => ds(24, Dt::Unorm),
            F::D32_SFLOAT_S8_UINT => ds(32, Dt::Sfloat),

            // ---- 4:2:2 subsampled ----
            //                                           c422(order, bits)
            F::G8B8G8R8_422_UNORM                     => c422(ChromaSubsamplingSampleOrder::Gbgr,  8),
            F::B8G8R8G8_422_UNORM                     => c422(ChromaSubsamplingSampleOrder::Bgrg, 8),
            F::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => c422(ChromaSubsamplingSampleOrder::Gbgr,  10),
            F::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => c422(ChromaSubsamplingSampleOrder::Bgrg, 10),
            F::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => c422(ChromaSubsamplingSampleOrder::Gbgr,  12),
            F::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => c422(ChromaSubsamplingSampleOrder::Bgrg, 12),
            F::G16B16G16R16_422_UNORM                 => c422(ChromaSubsamplingSampleOrder::Gbgr,  16),
            F::B16G16R16G16_422_UNORM                 => c422(ChromaSubsamplingSampleOrder::Bgrg, 16),

            // ---- Compressed: BCn ----
            //                              c(model,      srgb,        dim,       bpb,  datatype,   channels)
            F::BC1_RGB_UNORM_BLOCK       => c(Cm::BC1A,   Counterpart, [4,  4,  1], 8,  Dt::Unorm,  &[BC_COLOR]),
            F::BC1_RGB_SRGB_BLOCK        => c(Cm::BC1A,   Srgb,        [4,  4,  1], 8,  Dt::Unorm,  &[BC_COLOR]),
            F::BC1_RGBA_UNORM_BLOCK      => c(Cm::BC1A,   Counterpart, [4,  4,  1], 8,  Dt::Unorm,  &[BC1A_ALPHA]),
            F::BC1_RGBA_SRGB_BLOCK       => c(Cm::BC1A,   Srgb,        [4,  4,  1], 8,  Dt::Unorm,  &[BC1A_ALPHA]),
            F::BC2_UNORM_BLOCK           => c(Cm::BC2,    Counterpart, [4,  4,  1], 16, Dt::Unorm,  &[BC_ALPHA, BC_COLOR]),
            F::BC2_SRGB_BLOCK            => c(Cm::BC2,    Srgb,        [4,  4,  1], 16, Dt::Unorm,  &[BC_ALPHA, BC_COLOR]),
            F::BC3_UNORM_BLOCK           => c(Cm::BC3,    Counterpart, [4,  4,  1], 16, Dt::Unorm,  &[BC_ALPHA, BC_COLOR]),
            F::BC3_SRGB_BLOCK            => c(Cm::BC3,    Srgb,        [4,  4,  1], 16, Dt::Unorm,  &[BC_ALPHA, BC_COLOR]),
            F::BC4_UNORM_BLOCK           => c(Cm::BC4,    Linear,      [4,  4,  1], 8,  Dt::Unorm,  &[BC_COLOR]),
            F::BC4_SNORM_BLOCK           => c(Cm::BC4,    Linear,      [4,  4,  1], 8,  Dt::Snorm,  &[BC_COLOR]),
            F::BC5_UNORM_BLOCK           => c(Cm::BC5,    Linear,      [4,  4,  1], 16, Dt::Unorm,  &[BC5_RED, BC5_GREEN]),
            F::BC5_SNORM_BLOCK           => c(Cm::BC5,    Linear,      [4,  4,  1], 16, Dt::Snorm,  &[BC5_RED, BC5_GREEN]),
            F::BC6H_UFLOAT_BLOCK         => c(Cm::BC6H,   Linear,      [4,  4,  1], 16, Dt::Ufloat, &[BC_COLOR]),
            F::BC6H_SFLOAT_BLOCK         => c(Cm::BC6H,   Linear,      [4,  4,  1], 16, Dt::Sfloat, &[BC_COLOR]),
            F::BC7_UNORM_BLOCK           => c(Cm::BC7,    Counterpart, [4,  4,  1], 16, Dt::Unorm,  &[BC_COLOR]),
            F::BC7_SRGB_BLOCK            => c(Cm::BC7,    Srgb,        [4,  4,  1], 16, Dt::Unorm,  &[BC_COLOR]),

            // ---- Compressed: ETC2 / EAC ----
            F::ETC2_R8G8B8_UNORM_BLOCK   => c(Cm::ETC2,   Counterpart, [4,  4,  1], 8,  Dt::Unorm,  &[ETC2_COLOR]),
            F::ETC2_R8G8B8_SRGB_BLOCK    => c(Cm::ETC2,   Srgb,        [4,  4,  1], 8,  Dt::Unorm,  &[ETC2_COLOR]),
            F::ETC2_R8G8B8A1_UNORM_BLOCK => c(Cm::ETC2,   Counterpart, [4,  4,  1], 8,  Dt::Unorm,  &[ETC2_COLOR, ETC2_ALPHA]),
            F::ETC2_R8G8B8A1_SRGB_BLOCK  => c(Cm::ETC2,   Srgb,        [4,  4,  1], 8,  Dt::Unorm,  &[ETC2_COLOR, ETC2_ALPHA]),
            F::ETC2_R8G8B8A8_UNORM_BLOCK => c(Cm::ETC2,   Counterpart, [4,  4,  1], 16, Dt::Unorm,  &[ETC2_ALPHA, ETC2_COLOR]),
            F::ETC2_R8G8B8A8_SRGB_BLOCK  => c(Cm::ETC2,   Srgb,        [4,  4,  1], 16, Dt::Unorm,  &[ETC2_ALPHA, ETC2_COLOR]),
            F::EAC_R11_UNORM_BLOCK       => c(Cm::ETC2,   Linear,      [4,  4,  1], 8,  Dt::Unorm,  &[ETC2_RED]),
            F::EAC_R11_SNORM_BLOCK       => c(Cm::ETC2,   Linear,      [4,  4,  1], 8,  Dt::Snorm,  &[ETC2_RED]),
            F::EAC_R11G11_UNORM_BLOCK    => c(Cm::ETC2,   Linear,      [4,  4,  1], 16, Dt::Unorm,  &[ETC2_RED, ETC2_GREEN]),
            F::EAC_R11G11_SNORM_BLOCK    => c(Cm::ETC2,   Linear,      [4,  4,  1], 16, Dt::Snorm,  &[ETC2_RED, ETC2_GREEN]),

            // ---- Compressed: ASTC 2D ----
            F::ASTC_4x4_UNORM_BLOCK      => c(Cm::ASTC,   Counterpart, [4,  4,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_4x4_SRGB_BLOCK       => c(Cm::ASTC,   Srgb,        [4,  4,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_4x4_SFLOAT_BLOCK     => c(Cm::ASTC,   Linear,      [4,  4,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_5x4_UNORM_BLOCK      => c(Cm::ASTC,   Counterpart, [5,  4,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x4_SRGB_BLOCK       => c(Cm::ASTC,   Srgb,        [5,  4,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x4_SFLOAT_BLOCK     => c(Cm::ASTC,   Linear,      [5,  4,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_5x5_UNORM_BLOCK      => c(Cm::ASTC,   Counterpart, [5,  5,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x5_SRGB_BLOCK       => c(Cm::ASTC,   Srgb,        [5,  5,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x5_SFLOAT_BLOCK     => c(Cm::ASTC,   Linear,      [5,  5,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_6x5_UNORM_BLOCK      => c(Cm::ASTC,   Counterpart, [6,  5,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x5_SRGB_BLOCK       => c(Cm::ASTC,   Srgb,        [6,  5,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x5_SFLOAT_BLOCK     => c(Cm::ASTC,   Linear,      [6,  5,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_6x6_UNORM_BLOCK      => c(Cm::ASTC,   Counterpart, [6,  6,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x6_SRGB_BLOCK       => c(Cm::ASTC,   Srgb,        [6,  6,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x6_SFLOAT_BLOCK     => c(Cm::ASTC,   Linear,      [6,  6,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_8x5_UNORM_BLOCK      => c(Cm::ASTC,   Counterpart, [8,  5,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_8x5_SRGB_BLOCK       => c(Cm::ASTC,   Srgb,        [8,  5,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_8x5_SFLOAT_BLOCK     => c(Cm::ASTC,   Linear,      [8,  5,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_8x6_UNORM_BLOCK      => c(Cm::ASTC,   Counterpart, [8,  6,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_8x6_SRGB_BLOCK       => c(Cm::ASTC,   Srgb,        [8,  6,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_8x6_SFLOAT_BLOCK     => c(Cm::ASTC,   Linear,      [8,  6,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_8x8_UNORM_BLOCK      => c(Cm::ASTC,   Counterpart, [8,  8,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_8x8_SRGB_BLOCK       => c(Cm::ASTC,   Srgb,        [8,  8,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_8x8_SFLOAT_BLOCK     => c(Cm::ASTC,   Linear,      [8,  8,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_10x5_UNORM_BLOCK     => c(Cm::ASTC,   Counterpart, [10, 5,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_10x5_SRGB_BLOCK      => c(Cm::ASTC,   Srgb,        [10, 5,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_10x5_SFLOAT_BLOCK    => c(Cm::ASTC,   Linear,      [10, 5,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_10x6_UNORM_BLOCK     => c(Cm::ASTC,   Counterpart, [10, 6,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_10x6_SRGB_BLOCK      => c(Cm::ASTC,   Srgb,        [10, 6,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_10x6_SFLOAT_BLOCK    => c(Cm::ASTC,   Linear,      [10, 6,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_10x8_UNORM_BLOCK     => c(Cm::ASTC,   Counterpart, [10, 8,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_10x8_SRGB_BLOCK      => c(Cm::ASTC,   Srgb,        [10, 8,  1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_10x8_SFLOAT_BLOCK    => c(Cm::ASTC,   Linear,      [10, 8,  1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_10x10_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [10, 10, 1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_10x10_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [10, 10, 1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_10x10_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [10, 10, 1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_12x10_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [12, 10, 1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_12x10_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [12, 10, 1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_12x10_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [12, 10, 1], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_12x12_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [12, 12, 1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_12x12_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [12, 12, 1], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_12x12_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [12, 12, 1], 16, Dt::Sfloat, &[ASTC_DATA]),

            // ---- Compressed: ASTC 3D ----
            F::ASTC_3x3x3_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [3,  3,  3], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_3x3x3_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [3,  3,  3], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_3x3x3_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [3,  3,  3], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_4x3x3_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [4,  3,  3], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_4x3x3_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [4,  3,  3], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_4x3x3_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [4,  3,  3], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_4x4x3_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [4,  4,  3], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_4x4x3_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [4,  4,  3], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_4x4x3_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [4,  4,  3], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_4x4x4_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [4,  4,  4], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_4x4x4_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [4,  4,  4], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_4x4x4_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [4,  4,  4], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_5x4x4_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [5,  4,  4], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x4x4_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [5,  4,  4], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x4x4_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [5,  4,  4], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_5x5x4_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [5,  5,  4], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x5x4_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [5,  5,  4], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x5x4_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [5,  5,  4], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_5x5x5_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [5,  5,  5], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x5x5_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [5,  5,  5], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_5x5x5_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [5,  5,  5], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_6x5x5_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [6,  5,  5], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x5x5_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [6,  5,  5], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x5x5_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [6,  5,  5], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_6x6x5_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [6,  6,  5], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x6x5_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [6,  6,  5], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x6x5_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [6,  6,  5], 16, Dt::Sfloat, &[ASTC_DATA]),
            F::ASTC_6x6x6_UNORM_BLOCK    => c(Cm::ASTC,   Counterpart, [6,  6,  6], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x6x6_SRGB_BLOCK     => c(Cm::ASTC,   Srgb,        [6,  6,  6], 16, Dt::Unorm,  &[ASTC_DATA]),
            F::ASTC_6x6x6_SFLOAT_BLOCK   => c(Cm::ASTC,   Linear,      [6,  6,  6], 16, Dt::Sfloat, &[ASTC_DATA]),

            // ---- Compressed: PVRTC ----
            F::PVRTC1_2BPP_UNORM_BLOCK   => c(Cm::PVRTC,  Counterpart, [8,  4,  1], 8,  Dt::Unorm,  &[PVRTC_COLOR]),
            F::PVRTC1_2BPP_SRGB_BLOCK    => c(Cm::PVRTC,  Srgb,        [8,  4,  1], 8,  Dt::Unorm,  &[PVRTC_COLOR]),
            F::PVRTC1_4BPP_UNORM_BLOCK   => c(Cm::PVRTC,  Counterpart, [4,  4,  1], 8,  Dt::Unorm,  &[PVRTC_COLOR]),
            F::PVRTC1_4BPP_SRGB_BLOCK    => c(Cm::PVRTC,  Srgb,        [4,  4,  1], 8,  Dt::Unorm,  &[PVRTC_COLOR]),
            F::PVRTC2_2BPP_UNORM_BLOCK   => c(Cm::PVRTC2, Counterpart, [8,  4,  1], 8,  Dt::Unorm,  &[PVRTC_COLOR]),
            F::PVRTC2_2BPP_SRGB_BLOCK    => c(Cm::PVRTC2, Srgb,        [8,  4,  1], 8,  Dt::Unorm,  &[PVRTC_COLOR]),
            F::PVRTC2_4BPP_UNORM_BLOCK   => c(Cm::PVRTC2, Counterpart, [4,  4,  1], 8,  Dt::Unorm,  &[PVRTC_COLOR]),
            F::PVRTC2_4BPP_SRGB_BLOCK    => c(Cm::PVRTC2, Srgb,        [4,  4,  1], 8,  Dt::Unorm,  &[PVRTC_COLOR]),

            // Unknown format value
            _ => return None,
        };
        Some(res)
    }

    /// Builds a [`Basic`] DFD block from this scheme.
    ///
    /// Optional parameters override the defaults derived from the scheme:
    /// - `alpha_premultiplied`: sets [`DataFormatFlags::ALPHA_PREMULTIPLIED`] (default: straight alpha).
    /// - `transfer_function`: overrides the transfer function (default: [`TransferFunction::Linear`]
    ///   or [`TransferFunction::SRGB`] based on the scheme's [`FormatInherentTransferFunction`]).
    /// - `color_primaries`: overrides color primaries (default: [`ColorPrimaries::BT709`]).
    /// - `color_model`: overrides color model (default: [`ColorModel::RGBSDA`]).
    ///
    /// # Errors
    ///
    /// Returns [`BuildError`] if the caller's overrides conflict with the format's
    /// constraints:
    /// - Depth-stencil formats reject all overrides (fixed layout).
    /// - Compressed formats reject `color_model` overrides (must use intrinsic model).
    /// - Formats with an sRGB counterpart must not use sRGB transfer; sRGB variants
    ///   must use sRGB transfer.
    pub fn build(
        &self,
        alpha_premultiplied: bool,
        transfer_function: Option<TransferFunction>,
        color_primaries: Option<ColorPrimaries>,
        color_model: Option<ColorModel>,
    ) -> Result<Basic, BuildError> {
        match *self {
            Builder::Standard {
                datatype,
                srgb,
                bytes_per_texel,
                bit_count,
                bit_offset,
                channel_ids,
                ..
            } => {
                let color_model = color_model.unwrap_or(ColorModel::RGBSDA);
                let color_primaries = color_primaries.unwrap_or(ColorPrimaries::BT709);
                let transfer_function = resolve_transfer_function(srgb, transfer_function)?;
                let flags = if alpha_premultiplied {
                    DataFormatFlags::ALPHA_PREMULTIPLIED
                } else {
                    DataFormatFlags::STRAIGHT_ALPHA
                };

                // channel_ids, bit_offset, and bit_count are all in ascending
                // bit-offset order (guaranteed by the describe() table), so
                // samples can be emitted directly without sorting.
                let mut sample_information = Vec::with_capacity(channel_ids.len());
                for i in 0..channel_ids.len() {
                    let (lower, upper) = lower_upper(datatype, bit_count[i]);
                    sample_information.push(SampleInformation {
                        bit_offset: bit_offset[i] as u16,
                        bit_length: NonZeroU8::new(bit_count[i]).unwrap(),
                        channel_type: channel_ids[i],
                        channel_type_qualifiers: sample_qualifiers(datatype, srgb, channel_ids[i]),
                        sample_positions: SAMPLE_POS_ORIGIN,
                        lower,
                        upper,
                    });
                }

                let mut bytes_planes = [0u8; 8];
                bytes_planes[0] = bytes_per_texel;

                Ok(Basic {
                    color_model: Some(color_model),
                    color_primaries: Some(color_primaries),
                    transfer_function: Some(transfer_function),
                    flags,
                    texel_block_dimensions: [NonZeroU8::new(1).unwrap(); 4],
                    bytes_planes,
                    sample_information,
                })
            }

            Builder::DepthStencil {
                depth_bits,
                depth_datatype,
            } => {
                if alpha_premultiplied {
                    return Err(BuildError::DepthStencilPremultipliedAlpha);
                }
                if transfer_function.is_some() {
                    return Err(BuildError::DepthStencilTransferFunction);
                }
                if color_primaries.is_some() {
                    return Err(BuildError::DepthStencilColorPrimaries);
                }
                if color_model.is_some() {
                    return Err(BuildError::DepthStencilColorModel);
                }

                // All depth-stencil formats are single-plane packed.
                // D16_UNORM_S8_UINT: 4 bytes (D16 @ 0, S8 @ 16)
                // D24_UNORM_S8_UINT: 4 bytes (S8 @ 0, D24 @ 8)
                // D32_SFLOAT_S8_UINT: 8 bytes (D32 @ 0, S8 @ 32)
                let bytes_plane0: u8 = match depth_bits {
                    16 => 4,
                    24 => 4,
                    32 => 8,
                    _ => unreachable!("unsupported depth bit width: {depth_bits}"),
                };

                let mut bytes_planes = [0u8; 8];
                bytes_planes[0] = bytes_plane0;

                let depth_quals = qualifiers(depth_datatype);
                let (depth_lower, depth_upper) = lower_upper(depth_datatype, depth_bits);
                let (stencil_lower, stencil_upper) = lower_upper(Datatype::Uint, 8);

                // D24_UNORM_S8_UINT stores stencil in the low byte (S8 @ 0,
                // D24 @ 8) matching X8_D24_UNORM_PACK32 layout. The other two
                // store depth first.
                let (depth_offset, stencil_offset) = match depth_bits {
                    16 => (0u16, 16u16),
                    24 => (8, 0),
                    32 => (0, 32),
                    _ => unreachable!("unsupported depth bit width: {depth_bits}"),
                };

                // Samples are sorted by bit_offset.
                let mut sample_information = Vec::from([
                    SampleInformation {
                        bit_offset: depth_offset,
                        bit_length: NonZeroU8::new(depth_bits).unwrap(),
                        channel_type: CHANNEL_DEPTH,
                        channel_type_qualifiers: depth_quals,
                        sample_positions: SAMPLE_POS_ORIGIN,
                        lower: depth_lower,
                        upper: depth_upper,
                    },
                    SampleInformation {
                        bit_offset: stencil_offset,
                        bit_length: NonZeroU8::new(8).unwrap(),
                        channel_type: CHANNEL_STENCIL,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: SAMPLE_POS_ORIGIN,
                        lower: stencil_lower,
                        upper: stencil_upper,
                    },
                ]);

                sample_information.sort_unstable_by_key(|s| s.bit_offset);

                Ok(Basic {
                    color_model: Some(ColorModel::RGBSDA),
                    color_primaries: Some(ColorPrimaries::BT709),
                    transfer_function: Some(TransferFunction::Linear),
                    flags: DataFormatFlags::empty(),
                    texel_block_dimensions: [NonZeroU8::new(1).unwrap(); 4],
                    bytes_planes,
                    sample_information,
                })
            }

            Builder::Rgb9e5 => {
                let color_model = color_model.unwrap_or(ColorModel::RGBSDA);
                let color_primaries = color_primaries.unwrap_or(ColorPrimaries::BT709);
                let transfer_function = transfer_function.unwrap_or(TransferFunction::Linear);
                let flags = if alpha_premultiplied {
                    DataFormatFlags::ALPHA_PREMULTIPLIED
                } else {
                    DataFormatFlags::STRAIGHT_ALPHA
                };

                let mut bytes_planes = [0u8; 8];
                bytes_planes[0] = 4;

                // Layout (LSB→MSB): R9 @ 0, G9 @ 9, B9 @ 18, E5 @ 27.
                //
                // Samples are interleaved per-channel: R_base, R_exp, G_base,
                // G_exp, B_base, B_exp — NOT all bases then all exponents.
                const CHANNELS: [u8; 3] = [CHANNEL_R, CHANNEL_G, CHANNEL_B];
                const BASE_OFFSETS: [u16; 3] = [0, 9, 18];

                let mut sample_information = Vec::with_capacity(6);
                for i in 0..3 {
                    sample_information.push(SampleInformation {
                        bit_offset: BASE_OFFSETS[i],
                        bit_length: NonZeroU8::new(RGB9E5_MANTISSA_BITS).unwrap(),
                        channel_type: CHANNELS[i],
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: SAMPLE_POS_ORIGIN,
                        lower: 0,
                        upper: RGB9E5_MANTISSA_UPPER,
                    });
                    sample_information.push(SampleInformation {
                        bit_offset: RGB9E5_EXPONENT_OFFSET,
                        bit_length: NonZeroU8::new(RGB9E5_EXPONENT_BITS).unwrap(),
                        channel_type: CHANNELS[i],
                        channel_type_qualifiers: ChannelTypeQualifiers::EXPONENT,
                        sample_positions: SAMPLE_POS_ORIGIN,
                        lower: RGB9E5_EXPONENT_BIAS,
                        upper: RGB9E5_EXPONENT_MAX,
                    });
                }

                Ok(Basic {
                    color_model: Some(color_model),
                    color_primaries: Some(color_primaries),
                    transfer_function: Some(transfer_function),
                    flags,
                    texel_block_dimensions: [NonZeroU8::new(1).unwrap(); 4],
                    bytes_planes,
                    sample_information,
                })
            }

            Builder::Compressed {
                color_model: model,
                srgb,
                block_dimensions,
                bytes_per_block,
                datatype,
                channel_types,
            } => {
                if color_model.is_some() {
                    return Err(BuildError::CompressedColorModel);
                }
                let color_primaries = color_primaries.unwrap_or(ColorPrimaries::BT709);
                let transfer_function = resolve_transfer_function(srgb, transfer_function)?;
                let flags = if alpha_premultiplied {
                    DataFormatFlags::ALPHA_PREMULTIPLIED
                } else {
                    DataFormatFlags::STRAIGHT_ALPHA
                };

                let (lower, upper) = compressed_lower_upper(datatype);

                // Single-sample formats: one sample covering the whole block.
                // Dual-sample formats with 16-byte blocks: two 64-bit halves.
                // Dual-sample formats with 8-byte blocks: both samples cover the
                // same 64-bit block (e.g. ETC2 punchthrough alpha).
                let sample_bits = if channel_types.len() == 1 {
                    bytes_per_block * 8
                } else {
                    64
                };
                let sample_stride: u16 = if channel_types.len() > 1 && bytes_per_block > 8 {
                    64
                } else {
                    0
                };

                let mut sample_information = Vec::with_capacity(channel_types.len());
                for (i, &ch) in channel_types.iter().enumerate() {
                    sample_information.push(SampleInformation {
                        bit_offset: (i as u16) * sample_stride,
                        bit_length: NonZeroU8::new(sample_bits).unwrap(),
                        channel_type: ch,
                        channel_type_qualifiers: sample_qualifiers(datatype, srgb, ch),
                        sample_positions: SAMPLE_POS_ORIGIN,
                        lower,
                        upper,
                    });
                }

                let mut bytes_planes = [0u8; 8];
                bytes_planes[0] = bytes_per_block;

                Ok(Basic {
                    color_model: Some(model),
                    color_primaries: Some(color_primaries),
                    transfer_function: Some(transfer_function),
                    flags,
                    texel_block_dimensions: [
                        NonZeroU8::new(block_dimensions[0]).unwrap(),
                        NonZeroU8::new(block_dimensions[1]).unwrap(),
                        NonZeroU8::new(block_dimensions[2]).unwrap(),
                        NonZeroU8::new(1).unwrap(),
                    ],
                    bytes_planes,
                    sample_information,
                })
            }

            Builder::Subsampled422 {
                sample_order,
                bit_width,
            } => {
                let color_model = color_model.unwrap_or(ColorModel::YUVSDA);
                let color_primaries = color_primaries.unwrap_or(ColorPrimaries::BT709);
                let transfer_function = transfer_function.unwrap_or(TransferFunction::Linear);
                let flags = if alpha_premultiplied {
                    DataFormatFlags::ALPHA_PREMULTIPLIED
                } else {
                    DataFormatFlags::STRAIGHT_ALPHA
                };

                let (lower, upper) = lower_upper(Datatype::Unorm, bit_width);

                // Each channel occupies a word whose size is the next power-of-two
                // byte boundary (8-bit → 8-bit word, 10/12/16-bit → 16-bit word).
                let word_bits = (bit_width as u16).next_power_of_two().max(8);
                let pad_bits = word_bits - bit_width as u16;
                let bytes_per_block = (word_bits * 4 / 8) as u8;

                // Memory order and texel positions:
                //   GBGR: [Y₀, U, Y₁, V]  — Y at positions 0 and 1, U and V co-sited at 0
                //   BGRG: [U, Y₀, V, Y₁]  — Y at positions 0 and 1, U and V co-sited at 0
                let layout: [(u8, u8); 4] = if sample_order == ChromaSubsamplingSampleOrder::Gbgr {
                    [(CHANNEL_Y, 0), (CHANNEL_U, 0), (CHANNEL_Y, 1), (CHANNEL_V, 0)]
                } else {
                    [(CHANNEL_U, 0), (CHANNEL_Y, 0), (CHANNEL_V, 0), (CHANNEL_Y, 1)]
                };

                let sample_information = layout
                    .iter()
                    .enumerate()
                    .map(|(i, &(channel, pos_x))| SampleInformation {
                        bit_offset: (i as u16) * word_bits + pad_bits,
                        bit_length: NonZeroU8::new(bit_width).unwrap(),
                        channel_type: channel,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [pos_x, HALF_TEXEL, 0, 0],
                        lower,
                        upper,
                    })
                    .collect();

                let mut bytes_planes = [0u8; 8];
                bytes_planes[0] = bytes_per_block;

                Ok(Basic {
                    color_model: Some(color_model),
                    color_primaries: Some(color_primaries),
                    transfer_function: Some(transfer_function),
                    flags,
                    texel_block_dimensions: [
                        NonZeroU8::new(2).unwrap(),
                        NonZeroU8::new(1).unwrap(),
                        NonZeroU8::new(1).unwrap(),
                        NonZeroU8::new(1).unwrap(),
                    ],
                    bytes_planes,
                    sample_information,
                })
            }
        }
    }
}

/// Returns the [`ChannelTypeQualifiers`] for the given datatype.
fn qualifiers(datatype: Datatype) -> ChannelTypeQualifiers {
    match datatype {
        Datatype::Unorm => ChannelTypeQualifiers::empty(),
        Datatype::Snorm => ChannelTypeQualifiers::SIGNED,
        Datatype::Uint => ChannelTypeQualifiers::empty(),
        Datatype::Sint => ChannelTypeQualifiers::SIGNED,
        Datatype::Sfloat => ChannelTypeQualifiers::SIGNED | ChannelTypeQualifiers::FLOAT,
        Datatype::Ufloat => ChannelTypeQualifiers::FLOAT,
        Datatype::Sfixed5 => ChannelTypeQualifiers::SIGNED,
    }
}

/// Returns the [`ChannelTypeQualifiers`] for a specific sample, accounting for
/// the sRGB alpha exception.
///
/// In sRGB formats, the alpha channel (ID 15) is always stored linearly — only
/// the color channels use the sRGB OETF. The DFD signals this with the LINEAR
/// qualifier on the alpha sample.
fn sample_qualifiers(
    datatype: Datatype,
    srgb: FormatInherentTransferFunction,
    channel_id: u8,
) -> ChannelTypeQualifiers {
    let mut quals = qualifiers(datatype);
    if srgb == FormatInherentTransferFunction::Srgb && channel_id == CHANNEL_ALPHA {
        quals |= ChannelTypeQualifiers::LINEAR;
    }
    quals
}

/// Returns the (lower, upper) sample bounds for the given datatype and bit width.
fn lower_upper(datatype: Datatype, bits: u8) -> (u32, u32) {
    match datatype {
        Datatype::Unorm => (0, (1u32 << bits) - 1),
        Datatype::Snorm => {
            let max = (1u32 << (bits - 1)) - 1;
            let min = (-(max as i32)) as u32;
            (min, max)
        }
        // Integer (non-normalized) formats use 0/1 bounds per the DFD spec,
        // meaning "the entire representable range". The bit width is irrelevant.
        Datatype::Uint => (0, 1),
        Datatype::Sint => ((-1i32) as u32, 1),
        Datatype::Sfloat => ((-1.0f32).to_bits(), (1.0f32).to_bits()),
        Datatype::Ufloat => (0, (1.0f32).to_bits()),
        // R16G16_SFIXED5: 16-bit signed with 5 fractional bits.
        // Range is -2^10 / 2^5 = -32 to +2^10 / 2^5 = 32.
        Datatype::Sfixed5 => ((-32i32) as u32, 32),
    }
}

/// Returns the (lower, upper) sample bounds for compressed format datatypes.
///
/// Unlike [`lower_upper`], there is no meaningful per-channel bit width for
/// compressed formats. The bounds represent the conceptual numeric range.
fn compressed_lower_upper(datatype: Datatype) -> (u32, u32) {
    match datatype {
        Datatype::Unorm => (0, 0xFFFFFFFF),
        // Compressed snorm uses full i32 range (not the per-component
        // symmetric range used by uncompressed formats).
        Datatype::Snorm => (i32::MIN as u32, i32::MAX as u32),
        Datatype::Sfloat => ((-1.0f32).to_bits(), (1.0f32).to_bits()),
        Datatype::Ufloat => (0, (1.0f32).to_bits()),
        _ => unreachable!("unsupported compressed datatype"),
    }
}

/// Validates the transfer function against the [`SrgbTransfer`] constraint.
///
/// Returns the resolved transfer function, or an error if the caller's
/// override conflicts with the sRGB variant rules.
#[rustfmt::skip]
fn resolve_transfer_function(
    srgb: FormatInherentTransferFunction,
    requested: Option<TransferFunction>,
) -> Result<TransferFunction, BuildError> {
    match (srgb, requested) {
        // Srgb variant: must be SRGB, reject non-SRGB overrides.
        (FormatInherentTransferFunction::Srgb, None) =>                         Ok(TransferFunction::SRGB),
        (FormatInherentTransferFunction::Srgb, Some(TransferFunction::SRGB)) => Ok(TransferFunction::SRGB),
        (FormatInherentTransferFunction::Srgb, Some(_)) =>                      Err(BuildError::SrgbTransferRequired),
        // HasSrgbCounterpart: must NOT be SRGB.
        (FormatInherentTransferFunction::LinearWithSrgbCounterpart, None) =>                         Ok(TransferFunction::Linear),
        (FormatInherentTransferFunction::LinearWithSrgbCounterpart, Some(TransferFunction::SRGB)) => Err(BuildError::SrgbTransferNotAllowed),
        (FormatInherentTransferFunction::LinearWithSrgbCounterpart, Some(tf)) =>   Ok(tf),
        // Linear: no restriction.
        (FormatInherentTransferFunction::Linear, None) =>                       Ok(TransferFunction::Linear),
        (FormatInherentTransferFunction::Linear, Some(tf)) => Ok(tf),
    }
}
