use crate::{BasicDataFormatDescriptorHeader, ChannelTypeQualifiers, DataFormatFlags, SampleInformation};
use core::{fmt, num::NonZeroU32};

macro_rules! pseudo_enum {
    ($(#[$attr:meta])* $name:ident { $($case:ident = $value:literal,)* }) => {
        $(#[$attr])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name(pub NonZeroU32);
        #[allow(non_upper_case_globals)]
        impl $name {
            pub fn new(x: u32) -> Option<Self> {
                Some(Self(NonZeroU32::new(x)?))
            }

            $(
                pub const $case: Self = Self(unsafe { NonZeroU32::new_unchecked($value) });
            )*
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let name = match self.0.get() {
                    $($value => Some(stringify!($case)),)*
                    _ => None,
                };
                match name {
                    Some(name) => f.pad(name),
                    None => write!(f, concat!(stringify!($name), "({})"), self.0.get()),
                }
            }
        }
    };
}

pseudo_enum! {
    /// Known texture formats
    Format {
        R4G4_UNORM_PACK8 = 1,
        R4G4B4A4_UNORM_PACK16 = 2,
        B4G4R4A4_UNORM_PACK16 = 3,
        R5G6B5_UNORM_PACK16 = 4,
        B5G6R5_UNORM_PACK16 = 5,
        R5G5B5A1_UNORM_PACK16 = 6,
        B5G5R5A1_UNORM_PACK16 = 7,
        A1R5G5B5_UNORM_PACK16 = 8,
        R8_UNORM = 9,
        R8_SNORM = 10,
        R8_UINT = 13,
        R8_SINT = 14,
        R8_SRGB = 15,
        R8G8_UNORM = 16,
        R8G8_SNORM = 17,
        R8G8_UINT = 20,
        R8G8_SINT = 21,
        R8G8_SRGB = 22,
        R8G8B8_UNORM = 23,
        R8G8B8_SNORM = 24,
        R8G8B8_UINT = 27,
        R8G8B8_SINT = 28,
        R8G8B8_SRGB = 29,
        B8G8R8_UNORM = 30,
        B8G8R8_SNORM = 31,
        B8G8R8_UINT = 34,
        B8G8R8_SINT = 35,
        B8G8R8_SRGB = 36,
        R8G8B8A8_UNORM = 37,
        R8G8B8A8_SNORM = 38,
        R8G8B8A8_UINT = 41,
        R8G8B8A8_SINT = 42,
        R8G8B8A8_SRGB = 43,
        B8G8R8A8_UNORM = 44,
        B8G8R8A8_SNORM = 45,
        B8G8R8A8_UINT = 48,
        B8G8R8A8_SINT = 49,
        B8G8R8A8_SRGB = 50,
        A2R10G10B10_UNORM_PACK32 = 58,
        A2R10G10B10_SNORM_PACK32 = 59,
        A2R10G10B10_UINT_PACK32 = 62,
        A2R10G10B10_SINT_PACK32 = 63,
        A2B10G10R10_UNORM_PACK32 = 64,
        A2B10G10R10_SNORM_PACK32 = 65,
        A2B10G10R10_UINT_PACK32 = 68,
        A2B10G10R10_SINT_PACK32 = 69,
        R16_UNORM = 70,
        R16_SNORM = 71,
        R16_UINT = 74,
        R16_SINT = 75,
        R16_SFLOAT = 76,
        R16G16_UNORM = 77,
        R16G16_SNORM = 78,
        R16G16_UINT = 81,
        R16G16_SINT = 82,
        R16G16_SFLOAT = 83,
        R16G16B16_UNORM = 84,
        R16G16B16_SNORM = 85,
        R16G16B16_UINT = 88,
        R16G16B16_SINT = 89,
        R16G16B16_SFLOAT = 90,
        R16G16B16A16_UNORM = 91,
        R16G16B16A16_SNORM = 92,
        R16G16B16A16_UINT = 95,
        R16G16B16A16_SINT = 96,
        R16G16B16A16_SFLOAT = 97,
        R32_UINT = 98,
        R32_SINT = 99,
        R32_SFLOAT = 100,
        R32G32_UINT = 101,
        R32G32_SINT = 102,
        R32G32_SFLOAT = 103,
        R32G32B32_UINT = 104,
        R32G32B32_SINT = 105,
        R32G32B32_SFLOAT = 106,
        R32G32B32A32_UINT = 107,
        R32G32B32A32_SINT = 108,
        R32G32B32A32_SFLOAT = 109,
        R64_UINT = 110,
        R64_SINT = 111,
        R64_SFLOAT = 112,
        R64G64_UINT = 113,
        R64G64_SINT = 114,
        R64G64_SFLOAT = 115,
        R64G64B64_UINT = 116,
        R64G64B64_SINT = 117,
        R64G64B64_SFLOAT = 118,
        R64G64B64A64_UINT = 119,
        R64G64B64A64_SINT = 120,
        R64G64B64A64_SFLOAT = 121,
        B10G11R11_UFLOAT_PACK32 = 122,
        E5B9G9R9_UFLOAT_PACK32 = 123,
        D16_UNORM = 124,
        X8_D24_UNORM_PACK32 = 125,
        D32_SFLOAT = 126,
        S8_UINT = 127,
        D16_UNORM_S8_UINT = 128,
        D24_UNORM_S8_UINT = 129,
        D32_SFLOAT_S8_UINT = 130,
        BC1_RGB_UNORM_BLOCK = 131,
        BC1_RGB_SRGB_BLOCK = 132,
        BC1_RGBA_UNORM_BLOCK = 133,
        BC1_RGBA_SRGB_BLOCK = 134,
        BC2_UNORM_BLOCK = 135,
        BC2_SRGB_BLOCK = 136,
        BC3_UNORM_BLOCK = 137,
        BC3_SRGB_BLOCK = 138,
        BC4_UNORM_BLOCK = 139,
        BC4_SNORM_BLOCK = 140,
        BC5_UNORM_BLOCK = 141,
        BC5_SNORM_BLOCK = 142,
        BC6H_UFLOAT_BLOCK = 143,
        BC6H_SFLOAT_BLOCK = 144,
        BC7_UNORM_BLOCK = 145,
        BC7_SRGB_BLOCK = 146,
        ETC2_R8G8B8_UNORM_BLOCK = 147,
        ETC2_R8G8B8_SRGB_BLOCK = 148,
        ETC2_R8G8B8A1_UNORM_BLOCK = 149,
        ETC2_R8G8B8A1_SRGB_BLOCK = 150,
        ETC2_R8G8B8A8_UNORM_BLOCK = 151,
        ETC2_R8G8B8A8_SRGB_BLOCK = 152,
        EAC_R11_UNORM_BLOCK = 153,
        EAC_R11_SNORM_BLOCK = 154,
        EAC_R11G11_UNORM_BLOCK = 155,
        EAC_R11G11_SNORM_BLOCK = 156,
        ASTC_4x4_UNORM_BLOCK = 157,
        ASTC_4x4_SRGB_BLOCK = 158,
        ASTC_5x4_UNORM_BLOCK = 159,
        ASTC_5x4_SRGB_BLOCK = 160,
        ASTC_5x5_UNORM_BLOCK = 161,
        ASTC_5x5_SRGB_BLOCK = 162,
        ASTC_6x5_UNORM_BLOCK = 163,
        ASTC_6x5_SRGB_BLOCK = 164,
        ASTC_6x6_UNORM_BLOCK = 165,
        ASTC_6x6_SRGB_BLOCK = 166,
        ASTC_8x5_UNORM_BLOCK = 167,
        ASTC_8x5_SRGB_BLOCK = 168,
        ASTC_8x6_UNORM_BLOCK = 169,
        ASTC_8x6_SRGB_BLOCK = 170,
        ASTC_8x8_UNORM_BLOCK = 171,
        ASTC_8x8_SRGB_BLOCK = 172,
        ASTC_10x5_UNORM_BLOCK = 173,
        ASTC_10x5_SRGB_BLOCK = 174,
        ASTC_10x6_UNORM_BLOCK = 175,
        ASTC_10x6_SRGB_BLOCK = 176,
        ASTC_10x8_UNORM_BLOCK = 177,
        ASTC_10x8_SRGB_BLOCK = 178,
        ASTC_10x10_UNORM_BLOCK = 179,
        ASTC_10x10_SRGB_BLOCK = 180,
        ASTC_12x10_UNORM_BLOCK = 181,
        ASTC_12x10_SRGB_BLOCK = 182,
        ASTC_12x12_UNORM_BLOCK = 183,
        ASTC_12x12_SRGB_BLOCK = 184,
        ASTC_4x4_SFLOAT_BLOCK = 1000066000,
        ASTC_5x4_SFLOAT_BLOCK = 1000066001,
        ASTC_5x5_SFLOAT_BLOCK = 1000066002,
        ASTC_6x5_SFLOAT_BLOCK = 1000066003,
        ASTC_6x6_SFLOAT_BLOCK = 1000066004,
        ASTC_8x5_SFLOAT_BLOCK = 1000066005,
        ASTC_8x6_SFLOAT_BLOCK = 1000066006,
        ASTC_8x8_SFLOAT_BLOCK = 1000066007,
        ASTC_10x5_SFLOAT_BLOCK = 1000066008,
        ASTC_10x6_SFLOAT_BLOCK = 1000066009,
        ASTC_10x8_SFLOAT_BLOCK = 1000066010,
        ASTC_10x10_SFLOAT_BLOCK = 1000066011,
        ASTC_12x10_SFLOAT_BLOCK = 1000066012,
        ASTC_12x12_SFLOAT_BLOCK = 1000066013,
    }
}

pseudo_enum! {
    /// Known supercompression schemes
    SupercompressionScheme {
        BasisLZ = 1,
        Zstandard = 2,
        ZLIB = 3,
    }
}

pseudo_enum! {
    ColorModel {
        RGBSDA = 1,
        YUVSDA = 2,
        YIQSDA = 3,
        LabSDA = 4,
        CMYKA = 5,
        XYZW = 6,
        HSVAAng = 7,
        HSLAAng = 8,
        HSVAHex = 9,
        HSLAHex = 10,
        YCgCoA = 11,
        YcCbcCrc = 12,
        ICtCp = 13,
        CIEXYZ = 14,
        CIEXYY = 15,
        BC1A = 128,
        BC2 = 129,
        BC3 = 130,
        BC4 = 131,
        BC5 = 132,
        BC6H = 133,
        BC7 = 134,
        ETC1 = 160,
        ETC2 = 161,
        ASTC = 162,
        ETC1S = 163,
        PVRTC = 164,
        PVRTC2 = 165,
        UASTC = 166,
    }
}

pseudo_enum! {
    ColorPrimaries {
        BT709 = 1,
        BT601EBU = 2,
        BT601SMPTE = 3,
        BT2020 = 4,
        CIEXYZ = 5,
        ACES = 6,
        ACESCC = 7,
        NTSC1953 = 8,
        PAL525 = 9,
        DISPLAYP3 = 10,
        AdobeRGB = 11,
    }
}

pseudo_enum! {
    TransferFunction {
        Linear = 1,
        SRGB = 2,
        ITU = 3,
        NTSC = 4,
        SLOG = 5,
        SLOG2 = 6,
        BT1886 = 7,
        HLGOETF = 8,
        HLGEOTF = 9,
        PQEOTF = 10,
        PQOETF = 11,
        DCIP3 = 12,
        PALOETF = 13,
        PAL625EOTF = 14,
        ST240 = 15,
        ACESCC = 16,
        ACESCCT = 17,
        AdobeRGB = 18,
    }
}

pub struct ParsedBasicDataFormatDescriptor {
    pub header: BasicDataFormatDescriptorHeader,
    pub sample_information: &'static [SampleInformation],
}

impl Format {
    pub fn type_size(self) -> Option<u32> {
        Some(match self {
            Self::R4G4_UNORM_PACK8 => 0,
            Self::R4G4B4A4_UNORM_PACK16 => 1,
            Self::B4G4R4A4_UNORM_PACK16 => 2,
            Self::R5G6B5_UNORM_PACK16 => 2,
            Self::B5G6R5_UNORM_PACK16 => 2,
            Self::R5G5B5A1_UNORM_PACK16 => 2,
            Self::B5G5R5A1_UNORM_PACK16 => 2,
            Self::A1R5G5B5_UNORM_PACK16 => 2,
            Self::R8_UNORM => 2,
            Self::R8_SNORM => 1,
            Self::R8_UINT => 1,
            Self::R8_SINT => 1,
            Self::R8_SRGB => 1,
            Self::R8G8_UNORM => 1,
            Self::R8G8_SNORM => 1,
            Self::R8G8_UINT => 1,
            Self::R8G8_SINT => 1,
            Self::R8G8_SRGB => 1,
            Self::R8G8B8_UNORM => 1,
            Self::R8G8B8_SNORM => 1,
            Self::R8G8B8_UINT => 1,
            Self::R8G8B8_SINT => 1,
            Self::R8G8B8_SRGB => 1,
            Self::B8G8R8_UNORM => 1,
            Self::B8G8R8_SNORM => 1,
            Self::B8G8R8_UINT => 1,
            Self::B8G8R8_SINT => 1,
            Self::B8G8R8_SRGB => 1,
            Self::R8G8B8A8_UNORM => 1,
            Self::R8G8B8A8_SNORM => 1,
            Self::R8G8B8A8_UINT => 1,
            Self::R8G8B8A8_SINT => 1,
            Self::R8G8B8A8_SRGB => 1,
            Self::B8G8R8A8_UNORM => 1,
            Self::B8G8R8A8_SNORM => 1,
            Self::B8G8R8A8_UINT => 1,
            Self::B8G8R8A8_SINT => 1,
            Self::B8G8R8A8_SRGB => 1,
            Self::A2R10G10B10_UNORM_PACK32 => 1,
            Self::A2R10G10B10_SNORM_PACK32 => 4,
            Self::A2R10G10B10_UINT_PACK32 => 4,
            Self::A2R10G10B10_SINT_PACK32 => 4,
            Self::A2B10G10R10_UNORM_PACK32 => 4,
            Self::A2B10G10R10_SNORM_PACK32 => 4,
            Self::A2B10G10R10_UINT_PACK32 => 4,
            Self::A2B10G10R10_SINT_PACK32 => 4,
            Self::R16_UNORM => 4,
            Self::R16_SNORM => 2,
            Self::R16_UINT => 2,
            Self::R16_SINT => 2,
            Self::R16_SFLOAT => 2,
            Self::R16G16_UNORM => 2,
            Self::R16G16_SNORM => 2,
            Self::R16G16_UINT => 2,
            Self::R16G16_SINT => 2,
            Self::R16G16_SFLOAT => 2,
            Self::R16G16B16_UNORM => 2,
            Self::R16G16B16_SNORM => 2,
            Self::R16G16B16_UINT => 2,
            Self::R16G16B16_SINT => 2,
            Self::R16G16B16_SFLOAT => 2,
            Self::R16G16B16A16_UNORM => 2,
            Self::R16G16B16A16_SNORM => 2,
            Self::R16G16B16A16_UINT => 2,
            Self::R16G16B16A16_SINT => 2,
            Self::R16G16B16A16_SFLOAT => 2,
            Self::R32_UINT => 2,
            Self::R32_SINT => 4,
            Self::R32_SFLOAT => 4,
            Self::R32G32_UINT => 4,
            Self::R32G32_SINT => 4,
            Self::R32G32_SFLOAT => 4,
            Self::R32G32B32_UINT => 4,
            Self::R32G32B32_SINT => 4,
            Self::R32G32B32_SFLOAT => 4,
            Self::R32G32B32A32_UINT => 4,
            Self::R32G32B32A32_SINT => 4,
            Self::R32G32B32A32_SFLOAT => 4,
            Self::R64_UINT => 4,
            Self::R64_SINT => 8,
            Self::R64_SFLOAT => 8,
            Self::R64G64_UINT => 8,
            Self::R64G64_SINT => 8,
            Self::R64G64_SFLOAT => 8,
            Self::R64G64B64_UINT => 8,
            Self::R64G64B64_SINT => 8,
            Self::R64G64B64_SFLOAT => 8,
            Self::R64G64B64A64_UINT => 8,
            Self::R64G64B64A64_SINT => 8,
            Self::R64G64B64A64_SFLOAT => 8,
            Self::B10G11R11_UFLOAT_PACK32 => 8,
            Self::E5B9G9R9_UFLOAT_PACK32 => 4,
            Self::D16_UNORM => 4,
            Self::X8_D24_UNORM_PACK32 => 4,
            Self::D32_SFLOAT => 4,
            Self::S8_UINT => 4,
            Self::D16_UNORM_S8_UINT => 4,
            Self::D24_UNORM_S8_UINT => 0,
            Self::D32_SFLOAT_S8_UINT => 0,
            Self::BC1_RGB_UNORM_BLOCK => 0,
            Self::BC1_RGB_SRGB_BLOCK => 1,
            Self::BC1_RGBA_UNORM_BLOCK => 1,
            Self::BC1_RGBA_SRGB_BLOCK => 1,
            Self::BC2_UNORM_BLOCK => 1,
            Self::BC2_SRGB_BLOCK => 1,
            Self::BC3_UNORM_BLOCK => 1,
            Self::BC3_SRGB_BLOCK => 1,
            Self::BC4_UNORM_BLOCK => 1,
            Self::BC4_SNORM_BLOCK => 1,
            Self::BC5_UNORM_BLOCK => 1,
            Self::BC5_SNORM_BLOCK => 1,
            Self::BC6H_UFLOAT_BLOCK => 1,
            Self::BC6H_SFLOAT_BLOCK => 1,
            Self::BC7_UNORM_BLOCK => 1,
            Self::BC7_SRGB_BLOCK => 1,
            Self::ETC2_R8G8B8_UNORM_BLOCK => 1,
            Self::ETC2_R8G8B8_SRGB_BLOCK => 1,
            Self::ETC2_R8G8B8A1_UNORM_BLOCK => 1,
            Self::ETC2_R8G8B8A1_SRGB_BLOCK => 1,
            Self::ETC2_R8G8B8A8_UNORM_BLOCK => 1,
            Self::ETC2_R8G8B8A8_SRGB_BLOCK => 1,
            Self::EAC_R11_UNORM_BLOCK => 1,
            Self::EAC_R11_SNORM_BLOCK => 1,
            Self::EAC_R11G11_UNORM_BLOCK => 1,
            Self::EAC_R11G11_SNORM_BLOCK => 1,
            Self::ASTC_4x4_UNORM_BLOCK => 1,
            Self::ASTC_4x4_SRGB_BLOCK => 1,
            Self::ASTC_5x4_UNORM_BLOCK => 1,
            Self::ASTC_5x4_SRGB_BLOCK => 1,
            Self::ASTC_5x5_UNORM_BLOCK => 1,
            Self::ASTC_5x5_SRGB_BLOCK => 1,
            Self::ASTC_6x5_UNORM_BLOCK => 1,
            Self::ASTC_6x5_SRGB_BLOCK => 1,
            Self::ASTC_6x6_UNORM_BLOCK => 1,
            Self::ASTC_6x6_SRGB_BLOCK => 1,
            Self::ASTC_8x5_UNORM_BLOCK => 1,
            Self::ASTC_8x5_SRGB_BLOCK => 1,
            Self::ASTC_8x6_UNORM_BLOCK => 1,
            Self::ASTC_8x6_SRGB_BLOCK => 1,
            Self::ASTC_8x8_UNORM_BLOCK => 1,
            Self::ASTC_8x8_SRGB_BLOCK => 1,
            Self::ASTC_10x5_UNORM_BLOCK => 1,
            Self::ASTC_10x5_SRGB_BLOCK => 1,
            Self::ASTC_10x6_UNORM_BLOCK => 1,
            Self::ASTC_10x6_SRGB_BLOCK => 1,
            Self::ASTC_10x8_UNORM_BLOCK => 1,
            Self::ASTC_10x8_SRGB_BLOCK => 1,
            Self::ASTC_10x10_UNORM_BLOCK => 1,
            Self::ASTC_10x10_SRGB_BLOCK => 1,
            Self::ASTC_12x10_UNORM_BLOCK => 1,
            Self::ASTC_12x10_SRGB_BLOCK => 1,
            Self::ASTC_12x12_UNORM_BLOCK => 1,
            Self::ASTC_12x12_SRGB_BLOCK => 1,
            _ => return None,
        })
    }

    pub fn basic_data_format_descriptor(self) -> Option<ParsedBasicDataFormatDescriptor> {
        Some(match self {
            Self::R4G4_UNORM_PACK8 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [1, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 4,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                        SampleInformation {
                            bit_offset: 4,
                            bit_length: 4,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                    ],
                };
                DFD
            }
            Self::R4G4B4A4_UNORM_PACK16 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 4,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                        SampleInformation {
                            bit_offset: 4,
                            bit_length: 4,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 4,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                        SampleInformation {
                            bit_offset: 12,
                            bit_length: 4,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                    ],
                };
                DFD
            }
            Self::B4G4R4A4_UNORM_PACK16 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 4,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                        SampleInformation {
                            bit_offset: 4,
                            bit_length: 4,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 4,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                        SampleInformation {
                            bit_offset: 12,
                            bit_length: 4,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 15,
                        },
                    ],
                };
                DFD
            }
            Self::R5G6B5_UNORM_PACK16 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 5,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 5,
                            bit_length: 6,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 63,
                        },
                        SampleInformation {
                            bit_offset: 11,
                            bit_length: 5,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                    ],
                };
                DFD
            }
            Self::B5G6R5_UNORM_PACK16 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 5,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 5,
                            bit_length: 6,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 63,
                        },
                        SampleInformation {
                            bit_offset: 11,
                            bit_length: 5,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                    ],
                };
                DFD
            }
            Self::R5G5B5A1_UNORM_PACK16 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 1,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 1,
                            bit_length: 5,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 6,
                            bit_length: 5,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 11,
                            bit_length: 5,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                    ],
                };
                DFD
            }
            Self::B5G5R5A1_UNORM_PACK16 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 1,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 1,
                            bit_length: 5,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 6,
                            bit_length: 5,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 11,
                            bit_length: 5,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                    ],
                };
                DFD
            }
            Self::A1R5G5B5_UNORM_PACK16 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 5,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 5,
                            bit_length: 5,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 5,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 15,
                            bit_length: 1,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R8_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [1, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 8,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 255,
                    }],
                };
                DFD
            }
            Self::R8_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [1, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 8,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                        sample_positions: [0, 0, 0, 0],
                        lower: 4294967169,
                        upper: 127,
                    }],
                };
                DFD
            }
            Self::R8_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [1, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 8,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::R8_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [1, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 8,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                        sample_positions: [0, 0, 0, 0],
                        lower: 4294967295,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::R8_SRGB => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [1, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 8,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 255,
                    }],
                };
                DFD
            }
            Self::R8G8_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8_SRGB => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8_SRGB => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8_SRGB => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [3, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8A8_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8A8_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8A8_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8A8_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R8G8B8A8_SRGB => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::LINEAR,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8A8_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8A8_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967169,
                            upper: 127,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8A8_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8A8_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::B8G8R8A8_SRGB => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 8,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 8,
                            bit_length: 8,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 8,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                        SampleInformation {
                            bit_offset: 24,
                            bit_length: 8,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::LINEAR,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 255,
                        },
                    ],
                };
                DFD
            }
            Self::A2R10G10B10_UNORM_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1023,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 10,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1023,
                        },
                        SampleInformation {
                            bit_offset: 20,
                            bit_length: 10,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1023,
                        },
                        SampleInformation {
                            bit_offset: 30,
                            bit_length: 2,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 3,
                        },
                    ],
                };
                DFD
            }
            Self::A2R10G10B10_SNORM_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294966785,
                            upper: 511,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 10,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294966785,
                            upper: 511,
                        },
                        SampleInformation {
                            bit_offset: 20,
                            bit_length: 10,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294966785,
                            upper: 511,
                        },
                        SampleInformation {
                            bit_offset: 30,
                            bit_length: 2,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::A2R10G10B10_UINT_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 10,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 20,
                            bit_length: 10,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 30,
                            bit_length: 2,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::A2R10G10B10_SINT_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 10,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 20,
                            bit_length: 10,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 30,
                            bit_length: 2,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::A2B10G10R10_UNORM_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 10,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1023,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 10,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1023,
                        },
                        SampleInformation {
                            bit_offset: 20,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1023,
                        },
                        SampleInformation {
                            bit_offset: 30,
                            bit_length: 2,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 3,
                        },
                    ],
                };
                DFD
            }
            Self::A2B10G10R10_SNORM_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 10,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294966785,
                            upper: 511,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 10,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294966785,
                            upper: 511,
                        },
                        SampleInformation {
                            bit_offset: 20,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294966785,
                            upper: 511,
                        },
                        SampleInformation {
                            bit_offset: 30,
                            bit_length: 2,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::A2B10G10R10_UINT_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 10,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 10,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 20,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 30,
                            bit_length: 2,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::A2B10G10R10_SINT_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 10,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 10,
                            bit_length: 10,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 20,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 30,
                            bit_length: 2,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R16_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 16,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 65535,
                    }],
                };
                DFD
            }
            Self::R16_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 16,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                        sample_positions: [0, 0, 0, 0],
                        lower: 4294934529,
                        upper: 32767,
                    }],
                };
                DFD
            }
            Self::R16_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 16,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::R16_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 16,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                        sample_positions: [0, 0, 0, 0],
                        lower: 4294967295,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::R16_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 16,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                        sample_positions: [0, 0, 0, 0],
                        lower: 3212836864,
                        upper: 1065353216,
                    }],
                };
                DFD
            }
            Self::R16G16_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [6, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [6, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [6, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [6, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [6, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16A16_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                        SampleInformation {
                            bit_offset: 48,
                            bit_length: 16,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 65535,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16A16_SNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                        SampleInformation {
                            bit_offset: 48,
                            bit_length: 16,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294934529,
                            upper: 32767,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16A16_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 48,
                            bit_length: 16,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16A16_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 48,
                            bit_length: 16,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R16G16B16A16_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 16,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 16,
                            bit_length: 16,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 16,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 48,
                            bit_length: 16,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::R32_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 32,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::R32_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 32,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                        sample_positions: [0, 0, 0, 0],
                        lower: 4294967295,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::R32_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 32,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                        sample_positions: [0, 0, 0, 0],
                        lower: 3212836864,
                        upper: 1065353216,
                    }],
                };
                DFD
            }
            Self::R32G32_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R32G32_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R32G32_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::R32G32B32_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [12, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 32,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R32G32B32_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [12, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 32,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R32G32B32_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [12, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 32,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::R32G32B32A32_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 32,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 96,
                            bit_length: 32,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R32G32B32A32_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 32,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 96,
                            bit_length: 32,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R32G32B32A32_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 32,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 32,
                            bit_length: 32,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 32,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 96,
                            bit_length: 32,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::R64_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::R64_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                        sample_positions: [0, 0, 0, 0],
                        lower: 4294967295,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::R64_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                        sample_positions: [0, 0, 0, 0],
                        lower: 3212836864,
                        upper: 1065353216,
                    }],
                };
                DFD
            }
            Self::R64G64_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R64G64_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R64G64_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::R64G64B64_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [24, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 128,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R64G64B64_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [24, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 128,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R64G64B64_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [24, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 128,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::R64G64B64A64_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [32, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 128,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 192,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R64G64B64A64_SINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [32, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 128,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                        SampleInformation {
                            bit_offset: 192,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 4294967295,
                            upper: 1,
                        },
                    ],
                };
                DFD
            }
            Self::R64G64B64A64_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [32, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 128,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 192,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                            sample_positions: [0, 0, 0, 0],
                            lower: 3212836864,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::B10G11R11_UFLOAT_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 11,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::FLOAT,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 11,
                            bit_length: 11,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::FLOAT,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1065353216,
                        },
                        SampleInformation {
                            bit_offset: 22,
                            bit_length: 10,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::FLOAT,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 1065353216,
                        },
                    ],
                };
                DFD
            }
            Self::E5B9G9R9_UFLOAT_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 9,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 8448,
                        },
                        SampleInformation {
                            bit_offset: 27,
                            bit_length: 5,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::EXPONENT,
                            sample_positions: [0, 0, 0, 0],
                            lower: 15,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 9,
                            bit_length: 9,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 8448,
                        },
                        SampleInformation {
                            bit_offset: 27,
                            bit_length: 5,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::EXPONENT,
                            sample_positions: [0, 0, 0, 0],
                            lower: 15,
                            upper: 31,
                        },
                        SampleInformation {
                            bit_offset: 18,
                            bit_length: 9,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 8448,
                        },
                        SampleInformation {
                            bit_offset: 27,
                            bit_length: 5,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::EXPONENT,
                            sample_positions: [0, 0, 0, 0],
                            lower: 15,
                            upper: 31,
                        },
                    ],
                };
                DFD
            }
            Self::D16_UNORM => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: None,
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [2, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 16,
                        channel_type: 14,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 65535,
                    }],
                };
                DFD
            }
            Self::X8_D24_UNORM_PACK32 => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: None,
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 24,
                        channel_type: 14,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 16777215,
                    }],
                };
                DFD
            }
            Self::D32_SFLOAT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: None,
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [4, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 32,
                        channel_type: 14,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                        sample_positions: [0, 0, 0, 0],
                        lower: 3212836864,
                        upper: 1065353216,
                    }],
                };
                DFD
            }
            Self::S8_UINT => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::RGBSDA),
                        color_primaries: None,
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [1, 1, 1, 1],
                        bytes_planes: [1, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 8,
                        channel_type: 13,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 1,
                    }],
                };
                DFD
            }
            Self::BC1_RGB_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC1A),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::BC1_RGB_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC1A),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::BC1_RGBA_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC1A),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 1,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::BC1_RGBA_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC1A),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 1,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::BC2_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::BC2_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::LINEAR,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::BC3_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC3),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::BC3_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC3),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::LINEAR,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::BC4_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC4),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::BC4_SNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC4),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                        sample_positions: [0, 0, 0, 0],
                        lower: 2147483648,
                        upper: 2147483647,
                    }],
                };
                DFD
            }
            Self::BC5_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC5),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::BC5_SNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC5),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 2147483648,
                            upper: 2147483647,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 2147483648,
                            upper: 2147483647,
                        },
                    ],
                };
                DFD
            }
            Self::BC6H_UFLOAT_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC6H),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::FLOAT,
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 1065353216,
                    }],
                };
                DFD
            }
            Self::BC6H_SFLOAT_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC6H),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED.union(ChannelTypeQualifiers::FLOAT),
                        sample_positions: [0, 0, 0, 0],
                        lower: 3212836864,
                        upper: 1065353216,
                    }],
                };
                DFD
            }
            Self::BC7_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC7),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::BC7_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::BC7),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ETC2_R8G8B8_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 2,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ETC2_R8G8B8_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 2,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ETC2_R8G8B8A1_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::ETC2_R8G8B8A1_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::LINEAR,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::ETC2_R8G8B8A8_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::ETC2_R8G8B8A8_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 15,
                            channel_type_qualifiers: ChannelTypeQualifiers::LINEAR,
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 2,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::EAC_R11_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::EAC_R11_SNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [8, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 64,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                        sample_positions: [0, 0, 0, 0],
                        lower: 2147483648,
                        upper: 2147483647,
                    }],
                };
                DFD
            }
            Self::EAC_R11G11_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                            sample_positions: [0, 0, 0, 0],
                            lower: 0,
                            upper: 4294967295,
                        },
                    ],
                };
                DFD
            }
            Self::EAC_R11G11_SNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ETC2),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[
                        SampleInformation {
                            bit_offset: 0,
                            bit_length: 64,
                            channel_type: 0,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 2147483648,
                            upper: 2147483647,
                        },
                        SampleInformation {
                            bit_offset: 64,
                            bit_length: 64,
                            channel_type: 1,
                            channel_type_qualifiers: ChannelTypeQualifiers::SIGNED,
                            sample_positions: [0, 0, 0, 0],
                            lower: 2147483648,
                            upper: 2147483647,
                        },
                    ],
                };
                DFD
            }
            Self::ASTC_4x4_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_4x4_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [4, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_5x4_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [5, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_5x4_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [5, 4, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_5x5_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [5, 5, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_5x5_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [5, 5, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_6x5_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [6, 5, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_6x5_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [6, 5, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_6x6_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [6, 6, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_6x6_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [6, 6, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_8x5_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [8, 5, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_8x5_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [8, 5, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_8x6_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [8, 6, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_8x6_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [8, 6, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_8x8_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [8, 8, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_8x8_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [8, 8, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_10x5_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [10, 5, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_10x5_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [10, 5, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_10x6_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [10, 6, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_10x6_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [10, 6, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_10x8_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [10, 8, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_10x8_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [10, 8, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_10x10_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [10, 10, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_10x10_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [10, 10, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_12x10_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [12, 10, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_12x10_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [12, 10, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_12x12_UNORM_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::Linear),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [12, 12, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            Self::ASTC_12x12_SRGB_BLOCK => {
                const DFD: ParsedBasicDataFormatDescriptor = ParsedBasicDataFormatDescriptor {
                    header: BasicDataFormatDescriptorHeader {
                        color_model: Some(ColorModel::ASTC),
                        color_primaries: Some(ColorPrimaries::BT709),
                        transfer_function: Some(TransferFunction::SRGB),
                        flags: DataFormatFlags::STRAIGHT_ALPHA,
                        texel_block_dimensions: [12, 12, 1, 1],
                        bytes_planes: [16, 0, 0, 0, 0, 0, 0, 0],
                    },
                    sample_information: &[SampleInformation {
                        bit_offset: 0,
                        bit_length: 128,
                        channel_type: 0,
                        channel_type_qualifiers: ChannelTypeQualifiers::empty(),
                        sample_positions: [0, 0, 0, 0],
                        lower: 0,
                        upper: 4294967295,
                    }],
                };
                DFD
            }
            _ => return None,
        })
    }
}
