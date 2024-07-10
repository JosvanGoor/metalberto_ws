use super::ffi::epoll::*;
use super::ffi::os::*;

pub struct Epoll {
    timeout_ms: usize,
    epoll: FileDescriptor
}