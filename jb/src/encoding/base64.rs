const B64_ENCODING_TABLE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";
const B64_DECODING_TABLE: [u8; 256] =
    [66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 64, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
     66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 62, 66, 66, 66, 63, 52, 53, 54, 55, 56, 57, 58, 59,
     60, 61, 66, 66, 66, 0, 66, 66, 66, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
     22, 23, 24, 25, 66, 66, 66, 66, 66, 66, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
     44, 45, 46, 47, 48, 49, 50, 51, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
     66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
     66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
     66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
     66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66, 66,
     66];

#[derive(Debug)]
pub enum Base64Error {
    LengthNotMultipleOf4,
    IllegalCharacter,
}

type Base64Result<T> = Result<T, Base64Error>;

pub fn encode(data: &[u8]) -> String {
    let pad = {
        let count = 3 - (data.len() % 3);
        if count == 3 {
            0
        } else {
            count
        }
    };

    let mut output = String::with_capacity(((data.len() / 3) + 1) * 4);

    for idx in (0..data.len()).step_by(3) {
        let value: usize = (*data.get(idx).unwrap_or(&0) as usize) << 16 |
                           (*data.get(idx + 1).unwrap_or(&0) as usize) << 8 |
                           (*data.get(idx + 2).unwrap_or(&0) as usize);

        output.push(B64_ENCODING_TABLE.as_bytes()[(value >> 18) & 63] as char);
        output.push(B64_ENCODING_TABLE.as_bytes()[(value >> 12) & 63] as char);
        output.push(B64_ENCODING_TABLE.as_bytes()[(value >> 6) & 63] as char);
        output.push(B64_ENCODING_TABLE.as_bytes()[value & 63] as char);
    }

    output.push_str(&"=".repeat(pad));
    output
}

pub fn decode(encoded: &str) -> Base64Result<Vec<u8>> {
    if encoded.len() % 4 != 0 {
        return Err(Base64Error::LengthNotMultipleOf4);
    }

    let bytes = encoded.as_bytes();
    if !bytes.iter().all(|ch| B64_ENCODING_TABLE.as_bytes().iter().any(|inner| inner == ch)) {
        return Err(Base64Error::IllegalCharacter);
    }

    let padding: usize = if bytes[bytes.len() - 1] == b'=' {
        if bytes[bytes.len() - 2] == b'=' {
            2
        } else {
            1
        }
    } else {
        0
    };

    let mut buffer = Vec::new();
    buffer.reserve_exact(4 * bytes.len() / 3);

    for idx in (0..bytes.len()).step_by(4) {
        let chunk: usize = (B64_DECODING_TABLE[bytes[idx] as usize] as usize) << 18 |
                           (B64_DECODING_TABLE[bytes[idx + 1] as usize] as usize) << 12 |
                           (B64_DECODING_TABLE[bytes[idx + 2] as usize] as usize) << 6 |
                           (B64_DECODING_TABLE[bytes[idx + 3] as usize] as usize);

        buffer.push(((chunk >> 16) & 0xFF) as u8);
        buffer.push(((chunk >> 8) & 0xFF) as u8);
        buffer.push((chunk & 0xFF) as u8);
    }

    for _ in 0..padding {
        buffer.pop();
    }

    Ok(buffer)
}
