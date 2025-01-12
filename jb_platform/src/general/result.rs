
#[derive(Debug)]
pub enum FfiError {
    BufferTooSmall,
    FailedToAllocate,
    Utf8Error(std::str::Utf8Error),
}

pub type FfiResult<T> = std::result::Result<T, FfiError>;

// MARK: Conversion
impl From<std::str::Utf8Error> for FfiError {
    fn from(value: std::str::Utf8Error) -> Self {
        FfiError::Utf8Error(value)
    }
}