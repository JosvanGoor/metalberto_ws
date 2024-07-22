use super::constants::{WSADESCRIPTION_LEN, WSASYS_STATUS_LEN};
use super::types::WORD;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct AddrInfo {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: usize,
    pub ai_canonname: *const u8,
    pub ai_addr: *mut SockAddr,
    pub ai_next: *mut AddrInfo
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct SockAddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14]
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

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct WsaData {
    pub version: WORD,
    pub high_version: WORD,
    pub max_sockets: u16,
    pub max_udp_dg: u16,
    pub vendor_info: *const i8,
    pub description: [u8; WSADESCRIPTION_LEN],
    pub system_status: [u8; WSASYS_STATUS_LEN]
}