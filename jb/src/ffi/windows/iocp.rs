use super::{DWord, Handle, LikePtr};

#[link(name = "user32")]
extern {
    pub fn GetLastError() -> DWord;

    pub fn CreateIoCompletionPort(file_handle: Handle, completion_port: Handle, completion_key: LikePtr, num_threads: DWord) -> Handle;
}