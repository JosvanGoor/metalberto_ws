
pub fn ntbs_to_string(bytes: &[u8]) -> std::result::Result<String, std::str::Utf8Error> {
    let null = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
    Ok(String::from(std::str::from_utf8(&bytes[..null])?))
}

pub fn string_to_ntbs(string: &str) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(string.len() + 1);
    buffer.extend_from_slice(string.as_bytes());
    buffer.push(b'\0');
    buffer
}