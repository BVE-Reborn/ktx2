use alloc::{string::String, vec, vec::Vec};

use crate::dfd::{Basic, Block, BuildError, DataFormatFlags};
use crate::{ColorModel, ColorPrimaries, Format, Header, Index, LevelIndex, SupercompressionScheme, TransferFunction};

/// Error returned by [`Writer::build`] when the configuration is invalid.
#[derive(Debug)]
#[non_exhaustive]
pub enum WriteError {
    /// `pixel_width` is zero.
    ZeroWidth,
    /// `face_count` is zero.
    ZeroFaceCount,
    /// No level data was provided. Call [`Writer::add_level`] at least once.
    NoLevelData,
    /// Both DFD builder methods (e.g. [`Writer::color_primaries`]) and
    /// [`Writer::custom_dfd_blocks`] were used. Use one or the other.
    ConflictingDfdConfiguration,
    /// Both [`Writer::application_name`] and [`Writer::disable_default_writer_key`] were used.
    ConflictingWriterKey,
    /// Format is `None` (VK_FORMAT_UNDEFINED) with no custom DFD provided.
    /// Use [`Writer::custom_dfd_blocks`] to supply a DFD for undefined formats.
    UndefinedFormatRequiresCustomDfd,
    /// Auto-generated DFD failed due to an invalid combination of format and
    /// DFD overrides. See the individual DFD builder methods
    /// ([`Writer::color_primaries`], [`Writer::transfer_function`],
    /// [`Writer::color_model`], [`Writer::alpha_premultiplied`]) for the
    /// restrictions on each setting, or use [`Writer::skip_dfd_validation`] to
    /// bypass these checks.
    DfdBuild(BuildError),
}

impl core::fmt::Display for WriteError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ZeroWidth => f.pad("pixel_width must be non-zero"),
            Self::ZeroFaceCount => f.pad("face_count must be non-zero"),
            Self::NoLevelData => f.pad("at least one level of data is required"),
            Self::ConflictingDfdConfiguration => f.pad("cannot use both DFD builder methods and custom_dfd_blocks()"),
            Self::ConflictingWriterKey => f.pad("cannot use both application_name() and disable_default_writer_key()"),
            Self::UndefinedFormatRequiresCustomDfd => {
                f.pad("VK_FORMAT_UNDEFINED requires custom DFD blocks via custom_dfd_blocks()")
            }
            Self::DfdBuild(e) => write!(f, "DFD generation failed: {}", e),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for WriteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DfdBuild(e) => Some(e),
            _ => None,
        }
    }
}

/// Auto-generated DFD settings, configured via builder methods.
#[derive(Debug, Clone, Default)]
struct DfdSettings {
    color_primaries: Option<ColorPrimaries>,
    transfer_function: Option<TransferFunction>,
    color_model: Option<ColorModel>,
    alpha_premultiplied: bool,
    skip_validation: bool,
}

/// Controls the `KTXwriter` key-value entry.
#[derive(Debug, Clone)]
enum WriterKey {
    /// Default: "ktx2 rust crate v{version}"
    Default,
    /// Custom: "{app_name} / ktx2 rust crate v{version}"
    Application(String),
    /// Disabled entirely.
    Disabled,
}

impl Default for WriterKey {
    fn default() -> Self {
        Self::Default
    }
}

/// Internal storage for a single mip level's data.
#[derive(Debug, Clone)]
struct LevelData {
    data: Vec<u8>,
    /// `None` means uncompressed_byte_length == data.len().
    uncompressed_byte_length: Option<u64>,
}

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}

fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) / alignment * alignment
}

/// Builder for assembling valid KTX2 files.
///
/// # Required
///
/// - **Format and dimensions**: provided via a constructor ([`new`](Self::new),
///   [`new_2d`](Self::new_2d), etc.)
/// - **Level data**: at least one level via [`add_level`](Self::add_level)
///
/// # Recommended
///
/// - Application name via [`application_name`](Self::application_name)
///
/// # Advanced
///
/// - DFD customization via [`color_primaries`](Self::color_primaries),
///   [`transfer_function`](Self::transfer_function), etc.
/// - Custom DFD blocks via [`custom_dfd_blocks`](Self::custom_dfd_blocks)
/// - Supercompression via [`supercompression_scheme`](Self::supercompression_scheme)
///   and [`supercompression_global_data`](Self::supercompression_global_data)
///
/// All builder methods use consume-self chaining.
#[derive(Debug, Clone)]
pub struct Writer {
    header: Header,
    levels: Vec<LevelData>,
    dfd_settings: Option<DfdSettings>,
    custom_dfd: Option<(Vec<Block>, u32)>,
    key_value_pairs: Vec<(String, Vec<u8>)>,
    writer_key: WriterKey,
    sgd: Vec<u8>,
}

impl Writer {
    /// Create a writer with full header control.
    ///
    /// The `index`, `level_count`, and `type_size` fields in the header are
    /// ignored and recomputed at [`build`](Self::build) time.
    ///
    /// Level data must be provided separately via [`add_level`](Self::add_level).
    pub fn new(header: Header) -> Self {
        Self {
            header,
            levels: Vec::new(),
            dfd_settings: None,
            custom_dfd: None,
            key_value_pairs: Vec::new(),
            writer_key: WriterKey::Default,
            sgd: Vec::new(),
        }
    }

    // ---- 1D ----

    /// Create a writer for a 1D texture.
    ///
    /// Sets `pixel_height = 0`, `pixel_depth = 0`, `face_count = 1`, `layer_count = 0`.
    pub fn new_1d(format: Format, width: u32) -> Self {
        Self::new(Header {
            format: Some(format),
            // Filled in by `build()` from DFD generation.
            type_size: 0,
            pixel_width: width,
            pixel_height: 0,
            pixel_depth: 0,
            layer_count: 0,
            face_count: 1,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
    }

    /// Create a writer for a 1D array texture.
    ///
    /// Sets `pixel_height = 0`, `pixel_depth = 0`, `face_count = 1`.
    ///
    /// Level data is ordered by array layer (layer 0, layer 1, ...).
    pub fn new_1d_array(format: Format, width: u32, layers: u32) -> Self {
        Self::new(Header {
            format: Some(format),
            // Filled in by `build()` from DFD generation.
            type_size: 0,
            pixel_width: width,
            pixel_height: 0,
            pixel_depth: 0,
            layer_count: layers,
            face_count: 1,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
    }

    // ---- 2D ----

    /// Create a writer for a simple 2D texture.
    ///
    /// Sets `face_count = 1`, `layer_count = 0` (non-array), `pixel_depth = 0`.
    pub fn new_2d(format: Format, width: u32, height: u32) -> Self {
        Self::new(Header {
            format: Some(format),
            // Filled in by `build()` from DFD generation.
            type_size: 0,
            pixel_width: width,
            pixel_height: height,
            pixel_depth: 0,
            layer_count: 0,
            face_count: 1,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
    }

    /// Create a writer for a 2D array texture.
    ///
    /// Sets `pixel_depth = 0`, `face_count = 1`.
    ///
    /// Level data is ordered by array layer (layer 0, layer 1, ...).
    pub fn new_2d_array(format: Format, width: u32, height: u32, layers: u32) -> Self {
        Self::new(Header {
            format: Some(format),
            // Filled in by `build()` from DFD generation.
            type_size: 0,
            pixel_width: width,
            pixel_height: height,
            pixel_depth: 0,
            layer_count: layers,
            face_count: 1,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
    }

    // ---- Cubemap ----

    /// Create a writer for a cubemap (6 faces).
    ///
    /// Level data for each level must contain all 6 faces concatenated in the
    /// order: +X, −X, +Y, −Y, +Z, −Z.
    pub fn new_cubemap(format: Format, width: u32, height: u32) -> Self {
        Self::new(Header {
            format: Some(format),
            // Filled in by `build()` from DFD generation.
            type_size: 0,
            pixel_width: width,
            pixel_height: height,
            pixel_depth: 0,
            layer_count: 0,
            face_count: 6,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
    }

    /// Create a writer for a cubemap array texture.
    ///
    /// Level data for each level is ordered by array layer, then by face within
    /// each layer. Faces are in the order: +X, −X, +Y, −Y, +Z, −Z.
    pub fn new_cubemap_array(format: Format, width: u32, height: u32, layers: u32) -> Self {
        Self::new(Header {
            format: Some(format),
            // Filled in by `build()` from DFD generation.
            type_size: 0,
            pixel_width: width,
            pixel_height: height,
            pixel_depth: 0,
            layer_count: layers,
            face_count: 6,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
    }

    // ---- 3D ----

    /// Create a writer for a 3D (volume) texture.
    ///
    /// Level data for each level is ordered by depth slice (z=0, z=1, ...).
    pub fn new_3d(format: Format, width: u32, height: u32, depth: u32) -> Self {
        Self::new(Header {
            format: Some(format),
            // Filled in by `build()` from DFD generation.
            type_size: 0,
            pixel_width: width,
            pixel_height: height,
            pixel_depth: depth,
            layer_count: 0,
            face_count: 1,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
    }

    /// Create a writer for a 3D array texture.
    ///
    /// Level data for each level is ordered by array layer, then by depth slice
    /// within each layer (z=0, z=1, ...).
    pub fn new_3d_array(format: Format, width: u32, height: u32, depth: u32, layers: u32) -> Self {
        Self::new(Header {
            format: Some(format),
            // Filled in by `build()` from DFD generation.
            type_size: 0,
            pixel_width: width,
            pixel_height: height,
            pixel_depth: depth,
            layer_count: layers,
            face_count: 1,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
    }

    // ---- Level methods ----

    /// Append a mip level.
    ///
    /// Levels must be added in order from level 0 (largest / full resolution)
    /// to level N−1 (smallest). Each subsequent level is typically half the
    /// dimensions of the previous.
    ///
    /// # Data layout
    ///
    /// Within each level, data is laid out as:
    /// **layer → face → depth slice → row → texel/block**
    ///
    /// - **Layers**: `layer_count` images (1 if the header's `layer_count` is 0)
    /// - **Faces**: `face_count` images per layer (6 for cubemaps, 1 otherwise).
    ///   Cubemap face order: +X, −X, +Y, −Y, +Z, −Z.
    /// - **Depth slices**: `pixel_depth` slices per face (for 3D textures)
    /// - **Rows**: tightly packed from top to bottom
    pub fn add_level(mut self, data: Vec<u8>) -> Self {
        self.levels.push(LevelData {
            data,
            uncompressed_byte_length: None,
        });
        self
    }

    /// Append multiple mip levels at once.
    ///
    /// Equivalent to calling [`add_level`](Self::add_level) for each element.
    /// Levels must be in order from largest (level 0) to smallest.
    pub fn add_levels(mut self, levels: impl IntoIterator<Item = Vec<u8>>) -> Self {
        for data in levels {
            self.levels.push(LevelData {
                data,
                uncompressed_byte_length: None,
            });
        }
        self
    }

    /// Append a supercompressed mip level.
    ///
    /// Use this instead of [`add_level`](Self::add_level) when the data has
    /// been pre-compressed (e.g. via zstd). `data` is the compressed bytes;
    /// `uncompressed_byte_length` is the size after decompression.
    ///
    /// Levels must be added in order from level 0 (largest) to level N−1
    /// (smallest). See [`add_level`](Self::add_level) for the data layout
    /// within each level.
    pub fn add_supercompressed_level(mut self, data: Vec<u8>, uncompressed_byte_length: u64) -> Self {
        self.levels.push(LevelData {
            data,
            uncompressed_byte_length: Some(uncompressed_byte_length),
        });
        self
    }

    /// Append multiple supercompressed mip levels at once.
    ///
    /// Equivalent to calling [`add_supercompressed_level`](Self::add_supercompressed_level)
    /// for each `(data, uncompressed_byte_length)` pair. Levels must be in
    /// order from largest (level 0) to smallest.
    pub fn add_supercompressed_levels(mut self, levels: impl IntoIterator<Item = (Vec<u8>, u64)>) -> Self {
        for (data, uncompressed) in levels {
            self.levels.push(LevelData {
                data,
                uncompressed_byte_length: Some(uncompressed),
            });
        }
        self
    }

    // ---- DFD builder methods ----

    /// Override the color primaries in the auto-generated DFD.
    ///
    /// Defaults to [`ColorPrimaries::BT709`] (the sRGB/Rec. 709 primaries
    /// used by most consumer content).
    ///
    /// # Restrictions
    ///
    /// Depth-stencil formats always use BT.709 primaries and do not allow
    /// overrides. Setting this on a depth-stencil format will cause
    /// [`build`](Self::build) to return
    /// [`WriteError::DfdBuild`]`(`[`DepthStencilColorPrimaries`](BuildError::DepthStencilColorPrimaries)`)`.
    /// Use [`skip_dfd_validation`](Self::skip_dfd_validation) to bypass this.
    ///
    /// Mutually exclusive with [`custom_dfd_blocks`](Self::custom_dfd_blocks).
    pub fn color_primaries(mut self, primaries: ColorPrimaries) -> Self {
        self.dfd_settings
            .get_or_insert_with(DfdSettings::default)
            .color_primaries = Some(primaries);
        self
    }

    /// Override how encoded sample values are converted to linear light
    /// in the auto-generated DFD.
    ///
    /// The default depends on the format: sRGB format variants (e.g.
    /// [`R8G8B8A8_SRGB`](Format::R8G8B8A8_SRGB)) default to
    /// [`TransferFunction::SRGB`]; all others default to
    /// [`TransferFunction::Linear`]. When you override the default
    /// transfer function, unlike with `srgb`-variant formats, the
    /// "alpha" channel is not automatically marked as linear by DFD generation,
    /// due to an ambiguity in the specification. See [this issue][ktx-spec-231].
    /// However, conventionally, the alpha channel of these formats is still
    /// expected to be linear.
    ///
    /// # Restrictions
    ///
    /// - **Depth-stencil formats** always use linear transfer and do not allow
    ///   overrides → [`DepthStencilTransferFunction`](BuildError::DepthStencilTransferFunction).
    /// - **UNORM formats with an SRGB counterpart** (e.g.
    ///   [`R8G8B8A8_UNORM`](Format::R8G8B8A8_UNORM)) must not be set to
    ///   [`TransferFunction::SRGB`]; use the `_SRGB` variant of the format
    ///   instead → [`SrgbTransferNotAllowed`](BuildError::SrgbTransferNotAllowed).
    /// - **SRGB format variants** (e.g.
    ///   [`R8G8B8A8_SRGB`](Format::R8G8B8A8_SRGB)) must use
    ///   [`TransferFunction::SRGB`] → [`SrgbTransferRequired`](BuildError::SrgbTransferRequired).
    ///
    /// Use [`skip_dfd_validation`](Self::skip_dfd_validation) to bypass these
    /// checks. Mutually exclusive with
    /// [`custom_dfd_blocks`](Self::custom_dfd_blocks).
    ///
    /// [ktx-spec-231]: https://github.com/KhronosGroup/KTX-Specification/issues/231
    pub fn transfer_function(mut self, tf: TransferFunction) -> Self {
        self.dfd_settings
            .get_or_insert_with(DfdSettings::default)
            .transfer_function = Some(tf);
        self
    }

    /// Override the color model in the auto-generated DFD.
    ///
    /// Defaults to [`ColorModel::RGBSDA`] for most formats,
    /// [`ColorModel::YUVSDA`] for 4:2:2 subsampled formats, or the intrinsic
    /// model for compressed formats (e.g. [`ColorModel::BC7`](ColorModel::BC7),
    /// [`ColorModel::ASTC`](ColorModel::ASTC)). If this is changed from the
    /// default for the format, the DFD will be formally invalid, but this may
    /// be useful for some applications.
    ///
    /// # Restrictions
    ///
    /// - **Depth-stencil formats** always use [`ColorModel::RGBSDA`] and do not
    ///   allow overrides → [`DepthStencilColorModel`](BuildError::DepthStencilColorModel).
    /// - **Compressed formats** must use their intrinsic color model →
    ///   [`CompressedColorModel`](BuildError::CompressedColorModel).
    ///
    /// Use [`skip_dfd_validation`](Self::skip_dfd_validation) to bypass these
    /// checks. Mutually exclusive with
    /// [`custom_dfd_blocks`](Self::custom_dfd_blocks).
    pub fn color_model(mut self, model: ColorModel) -> Self {
        self.dfd_settings.get_or_insert_with(DfdSettings::default).color_model = Some(model);
        self
    }

    /// Set the alpha premultiplied flag in the auto-generated DFD.
    ///
    /// When `true`, sets the
    /// [`ALPHA_PREMULTIPLIED`](DataFormatFlags::ALPHA_PREMULTIPLIED) flag,
    /// indicating that color channel values have already been scaled by the
    /// alpha channel. When `false` (the default), alpha is straight
    /// (non-premultiplied).
    ///
    /// # Restrictions
    ///
    /// Depth-stencil formats have no alpha channel and do not allow this flag.
    /// Setting `true` on a depth-stencil format will cause [`build`](Self::build)
    /// to return [`WriteError::DfdBuild`]`(`[`DepthStencilPremultipliedAlpha`](BuildError::DepthStencilPremultipliedAlpha)`)`.
    /// Use [`skip_dfd_validation`](Self::skip_dfd_validation) to bypass this.
    ///
    /// Mutually exclusive with [`custom_dfd_blocks`](Self::custom_dfd_blocks).
    pub fn alpha_premultiplied(mut self, premultiplied: bool) -> Self {
        self.dfd_settings
            .get_or_insert_with(DfdSettings::default)
            .alpha_premultiplied = premultiplied;
        self
    }

    /// Bypass DFD validation when auto-generating.
    ///
    /// Normally, the DFD builder methods ([`color_primaries`](Self::color_primaries),
    /// [`transfer_function`](Self::transfer_function), etc.) enforce
    /// format-specific restrictions and will return errors for invalid
    /// combinations (see each method's documentation). This method disables
    /// those checks, allowing non-standard combinations that would otherwise
    /// be rejected.
    ///
    /// Mutually exclusive with [`custom_dfd_blocks`](Self::custom_dfd_blocks).
    pub fn skip_dfd_validation(mut self) -> Self {
        self.dfd_settings
            .get_or_insert_with(DfdSettings::default)
            .skip_validation = true;
        self
    }

    /// Replace the auto-generated DFD entirely with custom blocks.
    ///
    /// `type_size` is the size in bytes of each component in the texture data
    /// (e.g. 1 for 8-bit formats, 2 for 16-bit, 4 for 32-bit) and used to fill
    /// in [`Header::type_size`].
    ///
    /// Mutually exclusive with DFD builder methods ([`color_primaries`](Self::color_primaries),
    /// [`transfer_function`](Self::transfer_function), [`color_model`](Self::color_model),
    /// [`alpha_premultiplied`](Self::alpha_premultiplied), [`skip_dfd_validation`](Self::skip_dfd_validation)).
    pub fn custom_dfd_blocks(mut self, blocks: Vec<Block>, type_size: u32) -> Self {
        self.custom_dfd = Some((blocks, type_size));
        self
    }

    // ---- Key-value methods ----

    /// Add a key-value metadata pair.
    ///
    /// Keys are NUL-terminated UTF-8 strings per the KTX2 spec. The NUL
    /// terminator is added automatically. Values are raw bytes.
    pub fn key_value(mut self, key: &str, value: impl AsRef<[u8]>) -> Self {
        self.key_value_pairs.push((String::from(key), value.as_ref().to_vec()));
        self
    }

    /// Set the application name for the `KTXwriter` key.
    ///
    /// The resulting value will be `"{name} / ktx2 rust crate v{version}"`.
    ///
    /// Mutually exclusive with [`disable_default_writer_key`](Self::disable_default_writer_key).
    pub fn application_name(mut self, name: impl Into<String>) -> Self {
        self.writer_key = WriterKey::Application(name.into());
        self
    }

    /// Suppress the automatic `KTXwriter` key-value entry entirely.
    ///
    /// Mutually exclusive with [`application_name`](Self::application_name).
    pub fn disable_default_writer_key(mut self) -> Self {
        self.writer_key = WriterKey::Disabled;
        self
    }

    // ---- Supercompression methods ----

    /// Set the supercompression scheme in the header.
    pub fn supercompression_scheme(mut self, scheme: SupercompressionScheme) -> Self {
        self.header.supercompression_scheme = Some(scheme);
        self
    }

    /// Set the Supercompression Global Data (SGD) section.
    pub fn supercompression_global_data(mut self, data: Vec<u8>) -> Self {
        self.sgd = data;
        self
    }

    // ---- Build ----

    /// Build the KTX2 file bytes.
    ///
    /// # Errors
    ///
    /// Returns [`WriteError`] if:
    /// - `pixel_width` is zero
    /// - `face_count` is zero
    /// - No level data was provided (call [`add_level`](Self::add_level) first)
    /// - Both DFD builder methods and [`custom_dfd_blocks()`](Self::custom_dfd_blocks) were used
    /// - Format is `None` (VK_FORMAT_UNDEFINED) with no custom DFD
    /// - A DFD override violates format-specific restrictions (see the
    ///   individual setter docs for details)
    pub fn build(self) -> Result<Vec<u8>, WriteError> {
        // 1. Validate header fields
        if self.header.pixel_width == 0 {
            return Err(WriteError::ZeroWidth);
        }
        if self.header.face_count == 0 {
            return Err(WriteError::ZeroFaceCount);
        }
        if self.levels.is_empty() {
            return Err(WriteError::NoLevelData);
        }

        // 2. Check DFD mutual exclusivity
        if self.dfd_settings.is_some() && self.custom_dfd.is_some() {
            return Err(WriteError::ConflictingDfdConfiguration);
        }

        // 3. Generate DFD blocks and type_size
        let (dfd_blocks, type_size) = self.generate_dfd()?;

        // 4. Serialize DFD section (4-byte total size prefix + block bytes)
        let dfd_block_bytes: Vec<u8> = dfd_blocks.iter().flat_map(|b| b.to_vec()).collect();
        let dfd_total_size = 4 + dfd_block_bytes.len();

        // 5. Build KVD section
        let kvd_bytes = self.build_kvd();

        // 6. Compute file layout
        let level_count = self.levels.len();
        let level_index_size = level_count * LevelIndex::LENGTH;

        let dfd_offset = Header::LENGTH + level_index_size;
        let after_dfd = dfd_offset + dfd_total_size;

        let kvd_offset;
        let after_kvd;
        if kvd_bytes.is_empty() {
            kvd_offset = 0;
            after_kvd = after_dfd;
        } else {
            kvd_offset = after_dfd;
            after_kvd = kvd_offset + kvd_bytes.len();
        }

        let sgd_offset;
        let after_sgd;
        if self.sgd.is_empty() {
            sgd_offset = 0u64;
            after_sgd = after_kvd;
        } else {
            // SGD must be 8-byte aligned
            let aligned = align_up(after_kvd, 8);
            sgd_offset = aligned as u64;
            after_sgd = aligned + self.sgd.len();
        }

        // Determine level alignment from DFD bytes_planes[0]
        let bytes_per_block = dfd_blocks
            .iter()
            .find_map(|b| match b {
                Block::Basic(basic) => Some(basic.bytes_planes[0] as u32),
                _ => None,
            })
            .unwrap_or(4);
        let level_alignment = lcm(bytes_per_block.max(1), 4) as usize;

        // Levels are stored smallest-first in the file, but indexed level 0
        // (largest) first. Per KTX2 spec section 3.9.7, the smallest mip level
        // appears first in the file, immediately after the metadata sections.
        let mut level_offsets = Vec::with_capacity(level_count);
        let mut cursor = after_sgd;
        for i in (0..level_count).rev() {
            cursor = align_up(cursor, level_alignment);
            level_offsets.push((i, cursor));
            cursor += self.levels[i].data.len();
        }

        let file_size = cursor;

        // Build level index (level 0 first in the index)
        let mut level_index_entries = vec![
            LevelIndex {
                byte_offset: 0,
                byte_length: 0,
                uncompressed_byte_length: 0
            };
            level_count
        ];
        for &(idx, offset) in &level_offsets {
            let level = &self.levels[idx];
            let byte_length = level.data.len() as u64;
            let uncompressed = level.uncompressed_byte_length.unwrap_or(byte_length);
            level_index_entries[idx] = LevelIndex {
                byte_offset: offset as u64,
                byte_length,
                uncompressed_byte_length: uncompressed,
            };
        }

        // Build header with computed values
        let mut header = self.header;
        header.type_size = type_size;
        header.level_count = level_count as u32;
        header.index = Index {
            dfd_byte_offset: dfd_offset as u32,
            dfd_byte_length: dfd_total_size as u32,
            kvd_byte_offset: kvd_offset as u32,
            kvd_byte_length: kvd_bytes.len() as u32,
            sgd_byte_offset: sgd_offset,
            sgd_byte_length: self.sgd.len() as u64,
        };

        // 7. Allocate and write all sections
        let mut buf = vec![0u8; file_size];

        // Header
        buf[..Header::LENGTH].copy_from_slice(&header.as_bytes());

        // Level index
        for (i, entry) in level_index_entries.iter().enumerate() {
            let start = Header::LENGTH + i * LevelIndex::LENGTH;
            buf[start..start + LevelIndex::LENGTH].copy_from_slice(&entry.as_bytes());
        }

        // DFD section
        buf[dfd_offset..dfd_offset + 4].copy_from_slice(&(dfd_total_size as u32).to_le_bytes());
        buf[dfd_offset + 4..dfd_offset + 4 + dfd_block_bytes.len()].copy_from_slice(&dfd_block_bytes);

        // KVD section
        if !kvd_bytes.is_empty() {
            buf[kvd_offset..kvd_offset + kvd_bytes.len()].copy_from_slice(&kvd_bytes);
        }

        // SGD section
        if !self.sgd.is_empty() {
            let sgd_start = sgd_offset as usize;
            buf[sgd_start..sgd_start + self.sgd.len()].copy_from_slice(&self.sgd);
        }

        // Level data
        for &(idx, offset) in &level_offsets {
            let data = &self.levels[idx].data;
            buf[offset..offset + data.len()].copy_from_slice(data);
        }

        Ok(buf)
    }

    /// Generate DFD blocks and type_size based on the configured DFD mode.
    fn generate_dfd(&self) -> Result<(Vec<Block>, u32), WriteError> {
        // Custom DFD takes priority
        if let Some((ref blocks, type_size)) = self.custom_dfd {
            return Ok((blocks.clone(), type_size));
        }

        let format = self.header.format.ok_or(WriteError::UndefinedFormatRequiresCustomDfd)?;

        match self.dfd_settings {
            Some(ref settings) if settings.skip_validation => {
                // Generate default, then apply overrides without validation
                let (mut basic, type_size) = Basic::from_format(format).map_err(WriteError::DfdBuild)?;
                if let Some(cp) = settings.color_primaries {
                    basic.color_primaries = Some(cp);
                }
                if let Some(tf) = settings.transfer_function {
                    basic.transfer_function = Some(tf);
                }
                if let Some(cm) = settings.color_model {
                    basic.color_model = Some(cm);
                }
                if settings.alpha_premultiplied {
                    basic.flags = DataFormatFlags::ALPHA_PREMULTIPLIED;
                }
                Ok((vec![Block::Basic(basic)], type_size))
            }
            Some(ref settings) => {
                let (basic, type_size) = Basic::from_format_with(
                    format,
                    settings.alpha_premultiplied,
                    settings.transfer_function,
                    settings.color_primaries,
                    settings.color_model,
                )
                .map_err(WriteError::DfdBuild)?;
                Ok((vec![Block::Basic(basic)], type_size))
            }
            None => {
                let (basic, type_size) = Basic::from_format(format).map_err(WriteError::DfdBuild)?;
                Ok((vec![Block::Basic(basic)], type_size))
            }
        }
    }

    /// Build the key-value data section bytes.
    fn build_kvd(&self) -> Vec<u8> {
        let mut entries: Vec<(&str, &[u8])> = Vec::new();

        // Writer key (prepended before user entries)
        // KTXwriter value must be NUL-terminated per the KTX2 spec.
        let writer_value = match &self.writer_key {
            WriterKey::Default => Some(alloc::format!("ktx2 rust crate v{}\0", CRATE_VERSION).into_bytes()),
            WriterKey::Application(name) => {
                Some(alloc::format!("{} / ktx2 rust crate v{}\0", name, CRATE_VERSION).into_bytes())
            }
            WriterKey::Disabled => None,
        };
        if let Some(ref value) = writer_value {
            entries.push(("KTXwriter", value.as_slice()));
        }

        // User entries
        for (key, value) in &self.key_value_pairs {
            entries.push((key.as_str(), value.as_slice()));
        }

        if entries.is_empty() {
            return Vec::new();
        }

        let mut kvd = Vec::new();
        for (key, value) in &entries {
            // key + NUL + value
            let key_and_value_len = key.len() + 1 + value.len();

            // 4-byte length prefix
            kvd.extend_from_slice(&(key_and_value_len as u32).to_le_bytes());
            kvd.extend_from_slice(key.as_bytes());
            kvd.push(0); // NUL terminator
            kvd.extend_from_slice(value);

            // Pad to 4-byte alignment
            let total_so_far = kvd.len();
            let padding = (4 - (total_so_far % 4)) % 4;
            kvd.extend(core::iter::repeat(0).take(padding));
        }

        kvd
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Reader;

    #[test]
    fn roundtrip_2d_rgba8() {
        let pixel_data = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1)
            .add_level(pixel_data.clone())
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).expect("Reader should parse writer output");
        let header = reader.header();
        assert_eq!(header.format, Some(Format::R8G8B8A8_SRGB));
        assert_eq!(header.pixel_width, 1);
        assert_eq!(header.pixel_height, 1);
        assert_eq!(header.pixel_depth, 0);
        assert_eq!(header.face_count, 1);
        assert_eq!(header.layer_count, 0);
        assert_eq!(header.level_count, 1);
        assert_eq!(header.type_size, 1);

        let levels: Vec<_> = reader.levels().collect();
        assert_eq!(levels.len(), 1);
        assert_eq!(levels[0].data, &pixel_data[..]);

        // Check DFD
        let dfd = reader.dfd_blocks();
        assert_eq!(dfd.len(), 1);
    }

    #[test]
    fn roundtrip_cubemap() {
        let face_size = 4; // 1x1 RGBA8
        let data = vec![0u8; face_size * 6];
        let ktx2_bytes = Writer::new_cubemap(Format::R8G8B8A8_UNORM, 1, 1)
            .add_level(data.clone())
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().face_count, 6);
        let levels: Vec<_> = reader.levels().collect();
        assert_eq!(levels[0].data, &data[..]);
    }

    #[test]
    fn roundtrip_mip_levels() {
        let level0 = vec![0u8; 16]; // 2x2 RGBA8
        let level1 = vec![0u8; 4]; // 1x1 RGBA8
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 2, 2)
            .add_levels([level0.clone(), level1.clone()])
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().level_count, 2);
        let levels: Vec<_> = reader.levels().collect();
        assert_eq!(levels.len(), 2);
        assert_eq!(levels[0].data, &level0[..]);
        assert_eq!(levels[1].data, &level1[..]);
    }

    #[test]
    fn custom_dfd_roundtrip() {
        let (basic, type_size) = Basic::from_format(Format::R8G8B8A8_SRGB).unwrap();
        let pixel_data = vec![0u8; 4];
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1)
            .add_level(pixel_data)
            .custom_dfd_blocks(vec![Block::Basic(basic.clone())], type_size)
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        let dfd = reader.dfd_blocks();
        assert_eq!(dfd.len(), 1);
        assert_eq!(dfd[0], Block::Basic(basic));
    }

    #[test]
    fn conflicting_dfd_error() {
        let (basic, type_size) = Basic::from_format(Format::R8G8B8A8_SRGB).unwrap();
        let result = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1)
            .add_level(vec![0u8; 4])
            .color_primaries(ColorPrimaries::BT709)
            .custom_dfd_blocks(vec![Block::Basic(basic)], type_size)
            .build();

        assert!(matches!(result, Err(WriteError::ConflictingDfdConfiguration)));
    }

    #[test]
    fn application_name_kvd() {
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1)
            .add_level(vec![0u8; 4])
            .application_name("MyApp")
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        let kvd: Vec<_> = reader.key_value_data().collect();
        let writer_entry = kvd.iter().find(|(k, _)| *k == "KTXwriter").unwrap();
        let expected = alloc::format!("MyApp / ktx2 rust crate v{}\0", CRATE_VERSION);
        assert_eq!(writer_entry.1, expected.as_bytes());
    }

    #[test]
    fn disable_writer_key() {
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1)
            .add_level(vec![0u8; 4])
            .disable_default_writer_key()
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        let kvd: Vec<_> = reader.key_value_data().collect();
        assert!(kvd.iter().all(|(k, _)| *k != "KTXwriter"));
    }

    #[test]
    fn zero_width_error() {
        let result = Writer::new(Header {
            format: Some(Format::R8G8B8A8_SRGB),
            type_size: 1,
            pixel_width: 0,
            pixel_height: 1,
            pixel_depth: 0,
            layer_count: 0,
            face_count: 1,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
        .add_level(vec![0u8; 4])
        .build();

        assert!(matches!(result, Err(WriteError::ZeroWidth)));
    }

    #[test]
    fn no_level_data_error() {
        let result = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1).build();
        assert!(matches!(result, Err(WriteError::NoLevelData)));
    }

    #[test]
    fn undefined_format_error() {
        let result = Writer::new(Header {
            format: None,
            type_size: 1,
            pixel_width: 1,
            pixel_height: 1,
            pixel_depth: 0,
            layer_count: 0,
            face_count: 1,
            level_count: 0,
            supercompression_scheme: None,
            index: Index::default(),
        })
        .add_level(vec![0u8; 4])
        .build();

        assert!(matches!(result, Err(WriteError::UndefinedFormatRequiresCustomDfd)));
    }

    #[test]
    fn default_writer_key() {
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1)
            .add_level(vec![0u8; 4])
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        let kvd: Vec<_> = reader.key_value_data().collect();
        let writer_entry = kvd.iter().find(|(k, _)| *k == "KTXwriter").unwrap();
        let expected = alloc::format!("ktx2 rust crate v{CRATE_VERSION}\0");
        assert_eq!(writer_entry.1, expected.as_bytes());
    }

    #[test]
    fn user_key_value_pairs() {
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1)
            .add_level(vec![0u8; 4])
            .key_value("KTXorientation", b"rd")
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        let kvd: Vec<_> = reader.key_value_data().collect();
        let orient = kvd.iter().find(|(k, _)| *k == "KTXorientation").unwrap();
        assert_eq!(orient.1, b"rd");
    }

    #[test]
    fn roundtrip_2d_array() {
        let layers = 3u32;
        let data = vec![0u8; 4 * layers as usize]; // 1x1 RGBA8 per layer
        let ktx2_bytes = Writer::new_2d_array(Format::R8G8B8A8_SRGB, 1, 1, layers)
            .add_level(data.clone())
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().layer_count, 3);
        let levels: Vec<_> = reader.levels().collect();
        assert_eq!(levels[0].data, &data[..]);
    }

    #[test]
    fn roundtrip_1d() {
        let data = vec![0u8; 4]; // 1 texel RGBA8
        let ktx2_bytes = Writer::new_1d(Format::R8G8B8A8_SRGB, 1)
            .add_level(data.clone())
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().pixel_height, 0);
        assert_eq!(reader.header().pixel_depth, 0);
    }

    #[test]
    fn roundtrip_1d_array() {
        let layers = 2u32;
        let data = vec![0u8; 4 * layers as usize];
        let ktx2_bytes = Writer::new_1d_array(Format::R8G8B8A8_SRGB, 1, layers)
            .add_level(data.clone())
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().pixel_height, 0);
        assert_eq!(reader.header().layer_count, 2);
    }

    #[test]
    fn roundtrip_cubemap_array() {
        let layers = 2u32;
        let data = vec![0u8; 4 * 6 * layers as usize]; // 1x1 RGBA8 * 6 faces * 2 layers
        let ktx2_bytes = Writer::new_cubemap_array(Format::R8G8B8A8_SRGB, 1, 1, layers)
            .add_level(data.clone())
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().face_count, 6);
        assert_eq!(reader.header().layer_count, 2);
    }

    #[test]
    fn roundtrip_3d() {
        let depth = 2u32;
        let data = vec![0u8; 4 * depth as usize]; // 1x1x2 RGBA8
        let ktx2_bytes = Writer::new_3d(Format::R8G8B8A8_SRGB, 1, 1, depth)
            .add_level(data.clone())
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().pixel_depth, 2);
    }

    #[test]
    fn roundtrip_3d_array() {
        let depth = 2u32;
        let layers = 3u32;
        let data = vec![0u8; 4 * depth as usize * layers as usize];
        let ktx2_bytes = Writer::new_3d_array(Format::R8G8B8A8_SRGB, 1, 1, depth, layers)
            .add_level(data.clone())
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().pixel_depth, 2);
        assert_eq!(reader.header().layer_count, 3);
    }

    #[test]
    fn add_levels_plural() {
        let level0 = vec![0u8; 16];
        let level1 = vec![0u8; 4];
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 2, 2)
            .add_levels([level0.clone(), level1.clone()])
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        assert_eq!(reader.header().level_count, 2);
        let levels: Vec<_> = reader.levels().collect();
        assert_eq!(levels[0].data, &level0[..]);
        assert_eq!(levels[1].data, &level1[..]);
    }

    #[test]
    fn add_supercompressed_level() {
        let data = vec![0u8; 2]; // compressed
        let ktx2_bytes = Writer::new_2d(Format::R8G8B8A8_SRGB, 1, 1)
            .supercompression_scheme(SupercompressionScheme::Zstandard)
            .add_supercompressed_level(data.clone(), 4)
            .build()
            .unwrap();

        let reader = Reader::new(&ktx2_bytes[..]).unwrap();
        let levels: Vec<_> = reader.levels().collect();
        assert_eq!(levels[0].data, &data[..]);
        assert_eq!(levels[0].uncompressed_byte_length, 4);
    }
}
