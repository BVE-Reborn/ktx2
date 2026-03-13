use crate::ParseError;

pub fn read_bytes<const N: usize>(bytes: &[u8], offset: &mut usize) -> Result<[u8; N], ParseError> {
    let v = bytes
        .get(*offset..*offset + N)
        .ok_or(ParseError::UnexpectedEnd)?
        .try_into()
        .unwrap();
    *offset += N;
    Ok(v)
}

pub fn read_u16(bytes: &[u8], offset: &mut usize) -> Result<u16, ParseError> {
    let v = u16::from_le_bytes(read_bytes(bytes, offset)?);
    Ok(v)
}

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

pub fn shift_and_mask_lower(shift: u32, mask: u32, value: u32) -> u32 {
    (value >> shift) & ((1 << mask) - 1)
}
