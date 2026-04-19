use ktx2::dfd::Block;
use ktx2::{ColorModel, ColorPrimaries, Format, Header, Index, LevelIndex, TransferFunction};
use std::process::Command;

fn lcm(a: u32, b: u32) -> u32 {
    a / gcd(a, b) * b
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) / alignment * alignment
}

/// Check if the `ktx` CLI tool is available on the system.
fn ktx_available() -> bool {
    Command::new("ktx")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Build a minimal valid KTX2 file for the given format.
fn build_ktx2(
    format: Format,
    alpha_premultiplied: bool,
    transfer_function: Option<TransferFunction>,
    color_primaries: Option<ColorPrimaries>,
    color_model: Option<ColorModel>,
) -> Option<Vec<u8>> {
    let (basic, type_size) = ktx2::dfd::Basic::from_format_with(
        format,
        alpha_premultiplied,
        transfer_function,
        color_primaries,
        color_model,
    )
    .ok()?;

    let bytes_per_block = basic.bytes_planes[0] as u32;
    let block_w = basic.texel_block_dimensions[0].get() as u32;
    let block_h = basic.texel_block_dimensions[1].get() as u32;
    let block_d = basic.texel_block_dimensions[2].get() as u32;

    let pixel_width = block_w;
    let pixel_height = block_h;
    let pixel_depth = if block_d > 1 { block_d } else { 0 };

    // DFD section
    let dfd_block = Block::Basic(basic);
    let dfd_block_bytes = dfd_block.to_vec();
    let dfd_total_size = 4 + dfd_block_bytes.len();

    // Layout offsets
    let level_index_offset = Header::LENGTH; // 80
    let dfd_offset = level_index_offset + LevelIndex::LENGTH; // 80 + 24 = 104
    let after_dfd = dfd_offset + dfd_total_size;
    let alignment = lcm(bytes_per_block, 4) as usize;
    let level_data_offset = align_up(after_dfd, alignment);
    let level_data_size = bytes_per_block as usize; // 1 block

    let file_size = level_data_offset + level_data_size;
    let mut buf = vec![0u8; file_size];

    // Write header
    let header = Header {
        format: if color_model.is_some() { None } else { Some(format) },
        type_size,
        pixel_width,
        pixel_height,
        pixel_depth,
        layer_count: 0,
        face_count: 1,
        level_count: 1,
        supercompression_scheme: None,
        index: Index {
            dfd_byte_offset: dfd_offset as u32,
            dfd_byte_length: dfd_total_size as u32,
            kvd_byte_offset: 0,
            kvd_byte_length: 0,
            sgd_byte_offset: 0,
            sgd_byte_length: 0,
        },
    };
    buf[..Header::LENGTH].copy_from_slice(&header.as_bytes());

    // Write level index
    let level_index = LevelIndex {
        byte_offset: level_data_offset as u64,
        byte_length: level_data_size as u64,
        uncompressed_byte_length: level_data_size as u64,
    };
    buf[level_index_offset..level_index_offset + LevelIndex::LENGTH].copy_from_slice(&level_index.as_bytes());

    // Write DFD section (4-byte total size + block data)
    buf[dfd_offset..dfd_offset + 4].copy_from_slice(&(dfd_total_size as u32).to_le_bytes());
    buf[dfd_offset + 4..dfd_offset + 4 + dfd_block_bytes.len()].copy_from_slice(&dfd_block_bytes);

    // Level data is already zeroed

    Some(buf)
}

/// Run `ktx validate` on the given bytes and return stderr on failure.
fn ktx_validate(ktx2_bytes: &[u8]) -> Result<(), String> {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = Command::new("ktx")
        .args(["validate", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to spawn ktx: {e}"))?;

    if let Some(ref mut stdin) = child.stdin {
        let _ = stdin.write_all(ktx2_bytes);
    }
    drop(child.stdin.take());

    let output = child
        .wait_with_output()
        .map_err(|e| format!("ktx validate error: {e}"))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!(
            "ktx validate failed (exit {}):\nstdout: {stdout}\nstderr: {stderr}",
            output.status
        ))
    }
}

#[test]
fn validate_all_formats() {
    if !ktx_available() {
        panic!("`ktx` CLI not found!");
    }

    let mut failures: Vec<String> = Vec::new();
    let mut tested = 0;
    let mut skipped = 0;

    for &format in Format::ALL {
        let ktx2_bytes = match build_ktx2(format, false, None, None, None) {
            Some(b) => b,
            None => {
                skipped += 1;
                continue;
            }
        };
        tested += 1;

        // Self-validate with Reader::new()
        if let Err(e) = ktx2::Reader::new(&ktx2_bytes[..]) {
            failures.push(format!("{format:?}: Reader::new failed: {e:?}"));
            continue;
        }

        // External validation with ktx CLI
        if let Err(e) = ktx_validate(&ktx2_bytes) {
            failures.push(format!("{format:?}: {e}"));
        }
    }

    eprintln!("\n=== Format validation summary ===");
    eprintln!("Tested: {tested}, Skipped: {skipped}, Failed: {}", failures.len());

    if !failures.is_empty() {
        eprintln!("\nFailures:");
        for f in &failures {
            eprintln!("  - {f}");
        }
        panic!("{} format(s) failed validation", failures.len());
    }
}

/// Validates that overriding a non-sRGB format's transfer function to sRGB
/// correctly validates.
#[test]
fn validate_transfer_override() {
    if !ktx_available() {
        panic!("`ktx` CLI not found!");
    }

    // Formats without an inherent sRGB variant that have an alpha channel.
    let formats_with_alpha = [
        Format::R16G16B16A16_SFLOAT,
        Format::R16G16B16A16_UNORM,
        Format::R32G32B32A32_SFLOAT,
    ];

    let mut failures: Vec<String> = Vec::new();

    for format in formats_with_alpha {
        let ktx2_bytes = build_ktx2(format, false, Some(TransferFunction::SRGB), None, None)
            .unwrap_or_else(|| panic!("{format:?}: build_ktx2 returned None"));

        if let Err(e) = ktx2::Reader::new(&ktx2_bytes[..]) {
            failures.push(format!("{format:?}: Reader::new failed: {e:?}"));
            continue;
        }

        if let Err(e) = ktx_validate(&ktx2_bytes) {
            failures.push(format!("{format:?}: {e}"));
        }
    }

    if !failures.is_empty() {
        eprintln!("\nFailures:");
        for f in &failures {
            eprintln!("  - {f}");
        }
        panic!("{} format(s) failed validation", failures.len());
    }
}

/// Validates that overriding color primaries produces valid KTX2 files.
#[test]
fn validate_color_primaries_override() {
    if !ktx_available() {
        panic!("`ktx` CLI not found!");
    }

    let primaries = [
        ColorPrimaries::BT601EBU,
        ColorPrimaries::BT2020,
        ColorPrimaries::DISPLAYP3,
        ColorPrimaries::AdobeRGB,
    ];

    let mut failures: Vec<String> = Vec::new();

    for &cp in &primaries {
        let ktx2_bytes = build_ktx2(Format::R8G8B8A8_UNORM, false, None, Some(cp), None)
            .unwrap_or_else(|| panic!("{cp:?}: build_ktx2 returned None"));

        if let Err(e) = ktx2::Reader::new(&ktx2_bytes[..]) {
            failures.push(format!("{cp:?}: Reader::new failed: {e:?}"));
            continue;
        }

        if let Err(e) = ktx_validate(&ktx2_bytes) {
            failures.push(format!("{cp:?}: {e}"));
        }
    }

    if !failures.is_empty() {
        eprintln!("\nFailures:");
        for f in &failures {
            eprintln!("  - {f}");
        }
        panic!("{} primaries override(s) failed validation", failures.len());
    }
}

/// Validates that overriding the color model produces valid KTX2 files.
#[test]
fn validate_color_model_override() {
    if !ktx_available() {
        panic!("`ktx` CLI not found!");
    }

    // libktx only supports loading RGBSDA and YUVSDA color models.
    let models = [ColorModel::YUVSDA];

    let mut failures: Vec<String> = Vec::new();

    for &cm in &models {
        let ktx2_bytes = build_ktx2(Format::R8G8B8A8_UNORM, false, None, None, Some(cm))
            .unwrap_or_else(|| panic!("{cm:?}: build_ktx2 returned None"));

        if let Err(e) = ktx2::Reader::new(&ktx2_bytes[..]) {
            failures.push(format!("{cm:?}: Reader::new failed: {e:?}"));
            continue;
        }

        if let Err(e) = ktx_validate(&ktx2_bytes) {
            failures.push(format!("{cm:?}: {e}"));
        }
    }

    if !failures.is_empty() {
        eprintln!("\nFailures:");
        for f in &failures {
            eprintln!("  - {f}");
        }
        panic!("{} color model override(s) failed validation", failures.len());
    }
}

/// Validates that setting alpha_premultiplied produces valid KTX2 files.
#[test]
fn validate_premultiplied_alpha_override() {
    if !ktx_available() {
        panic!("`ktx` CLI not found!");
    }

    let formats_with_alpha = [
        Format::R8G8B8A8_UNORM,
        Format::R16G16B16A16_SFLOAT,
        Format::R32G32B32A32_SFLOAT,
    ];

    let mut failures: Vec<String> = Vec::new();

    for format in formats_with_alpha {
        let ktx2_bytes = build_ktx2(format, true, None, None, None)
            .unwrap_or_else(|| panic!("{format:?}: build_ktx2 returned None"));

        if let Err(e) = ktx2::Reader::new(&ktx2_bytes[..]) {
            failures.push(format!("{format:?}: Reader::new failed: {e:?}"));
            continue;
        }

        if let Err(e) = ktx_validate(&ktx2_bytes) {
            failures.push(format!("{format:?}: {e}"));
        }
    }

    if !failures.is_empty() {
        eprintln!("\nFailures:");
        for f in &failures {
            eprintln!("  - {f}");
        }
        panic!("{} premultiplied alpha override(s) failed validation", failures.len());
    }
}
