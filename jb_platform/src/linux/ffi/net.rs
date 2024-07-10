/*
    Implements ip adressing & host lookup
*/

use std::ffi::c_void;

pub(crate) const AF_INET: i32 = 2;
pub(crate) const AF_INET6: i32 = 10;
pub(crate) const AF_UNSPEC: i32 = 0;
pub(crate) const MSG_OOB: i32 = 0x01;
pub(crate) const MSG_PEEK: i32 = 0x02;
pub(crate) const MSG_DONTWAIT: i32 = 0x40;
pub(crate) const MSG_WAITALL: i32 = 0x100;
pub(crate) const MSG_MORE: i32 = 0x8000;
pub(crate) const INET_ADDRSTRLEN: usize = 16;
pub(crate) const INET6_ADDRSTRLEN: usize = 46;
pub(crate) const INADDR_ANY: u32 = 0;
pub(crate) const IPPROTO_TCP: i32 = 6;
pub(crate) const IPPROTO_UDP: i32 = 17;
pub(crate) const SOCK_STREAM: i32 = 1;
pub(crate) const SOCK_DGRAM: i32 = 2;

// general
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct SockAddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14]
}

impl Default for SockAddr {
    fn default() -> Self {
        Self { sa_family: Default::default(), sa_data: std::ptr::null() }
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct AddrInfo {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: u32,
    pub ai_addr: *mut SockAddr,
    pub ai_canonname: *const u8,
    pub ai_next: *const AddrInfo,
}

impl Default for AddrInfo {
    fn default() -> Self {
        Self {
            ai_flags: Default::default(),
            ai_family: Default::default(),
            ai_socktype: Default::default(),
            ai_protocol: Default::default(),
            ai_addrlen: Default::default(),
            ai_addr: std::ptr::null_mut(),
            ai_canonname: std::ptr::null(),
            ai_next: std::ptr::null()
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct SockAddrIn {
    pub sin_family: u16,
    pub sin_port: u16,
    pub sin_addr: u32,
    pub sin_zero: [u8; 8]
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct SockAddrIn6 {
    pub sin6_family: u16,
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u8; 16],
    pub sin6_scope_id: u32
}


#[link(name = "c")]
extern "C" {
    pub(crate) fn getaddrinfo(node: *const i8, service: *const u8, hints: *const AddrInfo, results: *mut *mut AddrInfo) -> i32;
    pub(crate) fn freeaddrinfo(res: *mut AddrInfo);
    pub(crate) fn gai_strerror(err_code: i32) -> *const i8;
    pub(crate) fn inet_ntop(af: i32, src: *const c_void, dst: *mut u8, size: u32) -> *const u8;
}