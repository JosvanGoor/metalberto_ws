use std::str;

const table: [u8; 22] = *b"0123456789abcdefABCDEF";

pub fn encode(data: &[u8]) -> String {
    let mut encoded_raw = Vec::new();
    
    for byte in data.iter() {
        encoded_raw.push(table[(byte >> 4) as usize]);
        encoded_raw.push(table[(byte & 0x0F) as usize]);
    }

    String::from(str::from_utf8(encoded_raw.as_slice()).unwrap())
}