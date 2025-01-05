use crate::structs::{AddrInfo, WsaData};
use crate::types::{PSTR, PCSTR, WORD};
use std::ffi::c_void;

#[link(name = "ws2_32")]
extern "C" {
    pub fn WSAGetLastError() -> i32;
    pub fn WSAStartup(version_required: WORD, wsa_data: *mut WsaData) -> i32;
    pub fn WSACleanup() -> i32;
    
    pub fn getaddrinfo(node_name: *const i8, service_name: PCSTR, hints: *const AddrInfo, results: *mut *mut AddrInfo) -> i32;
    pub fn freeaddrinfo(res: *mut AddrInfo);
    pub fn inet_ntop(family: i32, src: *const c_void, dst: PSTR, size: usize) -> PCSTR;
}

#[link(name = "user32")]
extern "C" {
    //https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-formatmessage
}