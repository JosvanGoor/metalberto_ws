use core::str;
use std::str::Utf8Error;


/*
    Naive implementation that searches for the index of a subsequence
*/
pub fn subsequence_index(offset: usize, buffer: &Vec<u8>, sequence: &[u8]) -> Option<usize> {
    if sequence.len() > buffer.len() || offset >= (buffer.len() - sequence.len()) {
        return None;
    }

    'outer: for idx in offset..(buffer.len() - sequence.len()) {
        for inner in 0..sequence.len() {
            if buffer[idx + inner] != sequence[inner] {
                continue 'outer;
            }
        }
        return Some(idx)
    }

    return None
}

pub fn bytes_to_string(bytes: &[u8]) -> Result<String, Utf8Error> {
    Ok(String::from(str::from_utf8(bytes)?))
}


#[derive(Clone, Copy, Debug)]
pub enum BytesToI32Error { Utf8Error, ParseIntError }
pub fn bytes_to_i32(bytes: &[u8]) -> Result<i32, BytesToI32Error> {
    Ok(str::from_utf8(bytes).map_err(|_| BytesToI32Error::Utf8Error)?
        .parse::<i32>().map_err(|_| BytesToI32Error::ParseIntError)?)
}