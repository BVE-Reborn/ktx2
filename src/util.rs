//! Parsing utility functions.

use crate::ParseError;

/// Reads `N` bytes from `bytes` at the current `offset`, advancing `offset` by `N`.
pub fn read_bytes<const N: usize>(bytes: &[u8], offset: &mut usize) -> Result<[u8; N], ParseError> {
    let v = bytes
        .get(*offset..*offset + N)
        .ok_or(ParseError::UnexpectedEnd)?
        .try_into()
        .unwrap();
    *offset += N;
    Ok(v)
}

/// Reads a little-endian `u16` from `bytes` at the current `offset`, advancing `offset` by 2.
pub fn read_u16(bytes: &[u8], offset: &mut usize) -> Result<u16, ParseError> {
    let v = u16::from_le_bytes(read_bytes(bytes, offset)?);
    Ok(v)
}

/// Reads a little-endian `u32` from `bytes` at the current `offset`, advancing `offset` by 4.
pub fn bytes_to_u32(bytes: &[u8], offset: &mut usize) -> Result<u32, ParseError> {
    let v = u32::from_le_bytes(
        bytes
            .get(*offset..*offset + 4)
            .ok_or(ParseError::UnexpectedEnd)?
            .try_into()
            .unwrap(),
    );
    *offset += 4;
    Ok(v)
}

/// Right-shifts `value` by `shift` bits, then masks out all but the lower `mask` bits of the result.
///
/// This is the same operation as bitfield extraction in shading languages.
pub fn shift_and_mask_lower(shift: u32, mask: u32, value: u32) -> u32 {
    (value >> shift) & ((1 << mask) - 1)
}
