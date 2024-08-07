use std::ffi::c_void;
use super::structs::{AddrInfo, EpollEvent, PollFd, SockAddr};
use super::types::FileDescriptor;


#[link(name = "c")]
extern "C" {
    // file descriptor functions
    pub fn close(fd: FileDescriptor) -> i32; 
    pub fn ioctl(fd: FileDescriptor, op: u64, ...) -> i32;
    pub fn poll(fds: *mut PollFd, nfds: u64, timeout_ms: i32) -> i32;
    pub fn fcntl(fd: FileDescriptor, cmd: i32) -> i32;

    // epoll
    pub fn epoll_create(size: i32) -> i32;
    pub fn epoll_ctl(epfd: FileDescriptor, op: i32, fd: FileDescriptor, event: *mut EpollEvent) -> i32;
    pub fn epoll_wait(epfd: FileDescriptor, events: *mut EpollEvent, max_events: i32, timeout_ms: i32) -> i32;

    // hostname functions
    pub fn freeaddrinfo(res: *mut AddrInfo);
    pub fn gai_strerror(err_code: i32) -> *const i8;
    pub fn getaddrinfo(node: *const i8, service: *const u8, hints: *const AddrInfo, results: *mut *mut AddrInfo) -> i32;
    pub fn inet_ntop(af: i32, src: *const c_void, dst: *mut u8, size: u32) -> *const u8;
    pub fn getsockname(fd: FileDescriptor, addr: *mut SockAddr, socklen: *mut u32) -> i32;
    pub fn getpeername(fd: FileDescriptor, addr: *mut SockAddr, socklen: *mut u32) -> i32;

    // socket functions
    // pub fn socket
    // pub fn connect
}