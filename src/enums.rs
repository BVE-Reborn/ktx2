use core::{
    fmt,
    num::{NonZeroU32, NonZeroU8},
};

macro_rules! pseudo_enum {
    ($(#[$attr:meta])* $container:ident($prim:ident) $name:ident { $($(#[$variant_attr:meta])* $case:ident = $value:literal,)* }) => {
        $(#[$attr])*
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $name($container);

        #[allow(non_upper_case_globals)]
        impl $name {
            pub fn new(x: $prim) -> Option<Self> {
                Some(Self($container::new(x)?))
            }

            pub fn value(&self) -> $prim {
                self.0.get()
            }

            $(
                $(#[$variant_attr])*
                pub const $case: Self = Self(unsafe { $container::new_unchecked($value) });
            )*

            pub const ALL: &'static [Self] = &[$(Self::$case),*];
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
    /// Vulkan `VkFormat` values for the texture's texel format.
    ///
    /// Wraps a non-zero `u32`. A value of `0` (`VK_FORMAT_UNDEFINED`) is
    /// represented as `None` at the [`Header`](crate::Header) level, used
    /// for Basis Universal and other supercompressed universal formats
    /// where the actual GPU format is chosen at transcode time.
    ///
    /// Unknown non-zero format values are preserved — use [`value()`](Self::value)
    /// to get the raw `u32` for passing to Vulkan.
    ///
    /// Intentionally omitted formats:
    /// - (scaled) *_USCALED/*_SSCALED: prohibited by KTX2. They are intended
    ///   for vertex data, almost no implementations support them for texturing,
    ///   and the DFD cannot distinguish them from int values.
    /// - (multiplanar) *_[2-9]PLANE_*: prohibited by KTX2. Multiplanar formats
    ///   are not supported.
    /// - (tensors) VK_ARM_tensors: basic DFDs cannot represent tensor formats,
    ///   so while not explicitly prohibited, these are omitted.
    ///
    /// NOTE: If variants are added or removed, update the match in
    /// `dfd/generate.rs` as well.
    NonZeroU32(u32) Format {
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
        // R8_USCALED = 11,    -- prohibited (scaled)
        // R8_SSCALED = 12,    -- prohibited (scaled)
        R8_UINT = 13,
        R8_SINT = 14,
        R8_SRGB = 15,
        R8G8_UNORM = 16,
        R8G8_SNORM = 17,
        // R8G8_USCALED = 18,  -- prohibited (scaled)
        // R8G8_SSCALED = 19,  -- prohibited (scaled)
        R8G8_UINT = 20,
        R8G8_SINT = 21,
        R8G8_SRGB = 22,
        R8G8B8_UNORM = 23,
        R8G8B8_SNORM = 24,
        // R8G8B8_USCALED = 25,  -- prohibited (scaled)
        // R8G8B8_SSCALED = 26,  -- prohibited (scaled)
        R8G8B8_UINT = 27,
        R8G8B8_SINT = 28,
        R8G8B8_SRGB = 29,
        B8G8R8_UNORM = 30,
        B8G8R8_SNORM = 31,
        // B8G8R8_USCALED = 32,  -- prohibited (scaled)
        // B8G8R8_SSCALED = 33,  -- prohibited (scaled)
        B8G8R8_UINT = 34,
        B8G8R8_SINT = 35,
        B8G8R8_SRGB = 36,
        R8G8B8A8_UNORM = 37,
        R8G8B8A8_SNORM = 38,
        // R8G8B8A8_USCALED = 39,  -- prohibited (scaled)
        // R8G8B8A8_SSCALED = 40,  -- prohibited (scaled)
        R8G8B8A8_UINT = 41,
        R8G8B8A8_SINT = 42,
        R8G8B8A8_SRGB = 43,
        B8G8R8A8_UNORM = 44,
        B8G8R8A8_SNORM = 45,
        // B8G8R8A8_USCALED = 46,  -- prohibited (scaled)
        // B8G8R8A8_SSCALED = 47,  -- prohibited (scaled)
        B8G8R8A8_UINT = 48,
        B8G8R8A8_SINT = 49,
        B8G8R8A8_SRGB = 50,
        A8B8G8R8_UNORM_PACK32 = 51,
        A8B8G8R8_SNORM_PACK32 = 52,
        // A8B8G8R8_USCALED_PACK32 = 53,  -- prohibited (scaled)
        // A8B8G8R8_SSCALED_PACK32 = 54,  -- prohibited (scaled)
        A8B8G8R8_UINT_PACK32 = 55,
        A8B8G8R8_SINT_PACK32 = 56,
        A8B8G8R8_SRGB_PACK32 = 57,
        A2R10G10B10_UNORM_PACK32 = 58,
        A2R10G10B10_SNORM_PACK32 = 59,
        // A2R10G10B10_USCALED_PACK32 = 60,  -- prohibited (scaled)
        // A2R10G10B10_SSCALED_PACK32 = 61,  -- prohibited (scaled)
        A2R10G10B10_UINT_PACK32 = 62,
        A2R10G10B10_SINT_PACK32 = 63,
        A2B10G10R10_UNORM_PACK32 = 64,
        A2B10G10R10_SNORM_PACK32 = 65,
        // A2B10G10R10_USCALED_PACK32 = 66,  -- prohibited (scaled)
        // A2B10G10R10_SSCALED_PACK32 = 67,  -- prohibited (scaled)
        A2B10G10R10_UINT_PACK32 = 68,
        A2B10G10R10_SINT_PACK32 = 69,
        R16_UNORM = 70,
        R16_SNORM = 71,
        // R16_USCALED = 72,  -- prohibited (scaled)
        // R16_SSCALED = 73,  -- prohibited (scaled)
        R16_UINT = 74,
        R16_SINT = 75,
        R16_SFLOAT = 76,
        R16G16_UNORM = 77,
        R16G16_SNORM = 78,
        // R16G16_USCALED = 79,  -- prohibited (scaled)
        // R16G16_SSCALED = 80,  -- prohibited (scaled)
        R16G16_UINT = 81,
        R16G16_SINT = 82,
        R16G16_SFLOAT = 83,
        R16G16B16_UNORM = 84,
        R16G16B16_SNORM = 85,
        // R16G16B16_USCALED = 86,  -- prohibited (scaled)
        // R16G16B16_SSCALED = 87,  -- prohibited (scaled)
        R16G16B16_UINT = 88,
        R16G16B16_SINT = 89,
        R16G16B16_SFLOAT = 90,
        R16G16B16A16_UNORM = 91,
        R16G16B16A16_SNORM = 92,
        // R16G16B16A16_USCALED = 93,  -- prohibited (scaled)
        // R16G16B16A16_SSCALED = 94,  -- prohibited (scaled)
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
        PVRTC1_2BPP_UNORM_BLOCK = 1000054000,
        PVRTC1_4BPP_UNORM_BLOCK = 1000054001,
        PVRTC2_2BPP_UNORM_BLOCK = 1000054002,
        PVRTC2_4BPP_UNORM_BLOCK = 1000054003,
        PVRTC1_2BPP_SRGB_BLOCK = 1000054004,
        PVRTC1_4BPP_SRGB_BLOCK = 1000054005,
        PVRTC2_2BPP_SRGB_BLOCK = 1000054006,
        PVRTC2_4BPP_SRGB_BLOCK = 1000054007,
        G8B8G8R8_422_UNORM = 1000156000,
        B8G8R8G8_422_UNORM = 1000156001,
        // 10001560{02..=06}: G8 3PLANE/2PLANE 420/422/444 -- prohibited (multiplanar)
        R10X6_UNORM_PACK16 = 1000156007,
        R10X6G10X6_UNORM_2PACK16 = 1000156008,
        R10X6G10X6B10X6A10X6_UNORM_4PACK16 = 1000156009,
        G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 = 1000156010,
        B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 = 1000156011,
        // 10001560{12..=16}: G10X6 3PLANE/2PLANE 420/422/444 -- prohibited (multiplanar)
        R12X4_UNORM_PACK16 = 1000156017,
        R12X4G12X4_UNORM_2PACK16 = 1000156018,
        R12X4G12X4B12X4A12X4_UNORM_4PACK16 = 1000156019,
        G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 = 1000156020,
        B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 = 1000156021,
        // 10001560{22..=26}: G12X4 3PLANE/2PLANE 420/422/444 -- prohibited (multiplanar)
        G16B16G16R16_422_UNORM = 1000156027,
        B16G16R16G16_422_UNORM = 1000156028,
        // 10001560{29..=33}: G16 3PLANE/2PLANE 420/422/444 -- prohibited (multiplanar)
        // 10003300{00..=03}: 2PLANE_444 variants (G8, G10X6, G12X4, G16) -- prohibited (multiplanar)
        ASTC_3x3x3_UNORM_BLOCK = 1000288000,
        ASTC_3x3x3_SRGB_BLOCK = 1000288001,
        ASTC_3x3x3_SFLOAT_BLOCK = 1000288002,
        ASTC_4x3x3_UNORM_BLOCK = 1000288003,
        ASTC_4x3x3_SRGB_BLOCK = 1000288004,
        ASTC_4x3x3_SFLOAT_BLOCK = 1000288005,
        ASTC_4x4x3_UNORM_BLOCK = 1000288006,
        ASTC_4x4x3_SRGB_BLOCK = 1000288007,
        ASTC_4x4x3_SFLOAT_BLOCK = 1000288008,
        ASTC_4x4x4_UNORM_BLOCK = 1000288009,
        ASTC_4x4x4_SRGB_BLOCK = 1000288010,
        ASTC_4x4x4_SFLOAT_BLOCK = 1000288011,
        ASTC_5x4x4_UNORM_BLOCK = 1000288012,
        ASTC_5x4x4_SRGB_BLOCK = 1000288013,
        ASTC_5x4x4_SFLOAT_BLOCK = 1000288014,
        ASTC_5x5x4_UNORM_BLOCK = 1000288015,
        ASTC_5x5x4_SRGB_BLOCK = 1000288016,
        ASTC_5x5x4_SFLOAT_BLOCK = 1000288017,
        ASTC_5x5x5_UNORM_BLOCK = 1000288018,
        ASTC_5x5x5_SRGB_BLOCK = 1000288019,
        ASTC_5x5x5_SFLOAT_BLOCK = 1000288020,
        ASTC_6x5x5_UNORM_BLOCK = 1000288021,
        ASTC_6x5x5_SRGB_BLOCK = 1000288022,
        ASTC_6x5x5_SFLOAT_BLOCK = 1000288023,
        ASTC_6x6x5_UNORM_BLOCK = 1000288024,
        ASTC_6x6x5_SRGB_BLOCK = 1000288025,
        ASTC_6x6x5_SFLOAT_BLOCK = 1000288026,
        ASTC_6x6x6_UNORM_BLOCK = 1000288027,
        ASTC_6x6x6_SRGB_BLOCK = 1000288028,
        ASTC_6x6x6_SFLOAT_BLOCK = 1000288029,
        A4R4G4B4_UNORM_PACK16 = 1000340000,
        A4B4G4R4_UNORM_PACK16 = 1000340001,
        // 10004600{00..=03}: VK_ARM_tensors -- omitted (tensors)
        R16G16_SFIXED5 = 1000464000,
        A1B5G5R5_UNORM_PACK16 = 1000470000,
        A8_UNORM = 1000470001,
        R10X6_UINT_PACK16 = 1000609000,
        R10X6G10X6_UINT_2PACK16 = 1000609001,
        R10X6G10X6B10X6A10X6_UINT_4PACK16 = 1000609002,
        R12X4_UINT_PACK16 = 1000609003,
        R12X4G12X4_UINT_2PACK16 = 1000609004,
        R12X4G12X4B12X4A12X4_UINT_4PACK16 = 1000609005,
        R14X2_UINT_PACK16 = 1000609006,
        R14X2G14X2_UINT_2PACK16 = 1000609007,
        R14X2G14X2B14X2A14X2_UINT_4PACK16 = 1000609008,
        R14X2_UNORM_PACK16 = 1000609009,
        R14X2G14X2_UNORM_2PACK16 = 1000609010,
        R14X2G14X2B14X2A14X2_UNORM_4PACK16 = 1000609011,
        // G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16 = 1000609012,  -- prohibited (multiplanar)
        // G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16 = 1000609013,  -- prohibited (multiplanar)
    }
}

pseudo_enum! {
    /// Supercompression applied to mip level data.
    ///
    /// `None` (raw value `0`) at the [`Header`](crate::Header) level means
    /// no supercompression — level data is ready to use directly.
    ///
    /// Vendor-specific schemes use IDs `>= 0x10000`.
    NonZeroU32(u32) SupercompressionScheme {
        /// Basis Universal LZ. Level data must be transcoded, not just
        /// decompressed. Global data section contains codebooks.
        /// [`Level::uncompressed_byte_length`](crate::Level::uncompressed_byte_length)
        /// will be `0`.
        BasisLZ = 1,
        /// Zstandard (zstd). Decompress each level independently.
        Zstandard = 2,
        /// ZLIB (deflate). Decompress each level independently.
        ZLIB = 3,
    }
}

pseudo_enum! {
    /// Color model from the Khronos Data Format specification.
    ///
    /// `None` (raw value `0`) means unspecified. Values 128+ are
    /// compressed-format models (BC, ETC, ASTC, etc.).
    NonZeroU8(u8) ColorModel {
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
    /// Color primaries from the Khronos Data Format specification.
    ///
    /// `None` (raw value `0`) means unspecified. [`BT709`](Self::BT709) is
    /// the most common value (sRGB / Rec. 709).
    NonZeroU8(u8) ColorPrimaries {
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
    /// Transfer function (OETF/EOTF) from the Khronos Data Format specification.
    ///
    /// `None` (raw value `0`) means unspecified. [`SRGB`](Self::SRGB) and
    /// [`Linear`](Self::Linear) are the most common values for game textures.
    NonZeroU8(u8) TransferFunction {
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
