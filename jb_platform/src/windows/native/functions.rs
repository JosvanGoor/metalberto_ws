use std::ffi::c_void;
use super::structs::{AddrInfo, WsaData};
use super::types::{PSTR, PCSTR, WORD};

#[link(name = "ws2_32")]
extern "C" {
    pub fn WSAGetLastError() -> i32;
    pub fn WSAStartup(version_required: WORD, wsa_data: *mut WsaData) -> i32;
    pub fn WSACleanup() -> i32;

    pub fn getaddrinfo(node_name: PCSTR, service_name: PCSTR, hints: *const AddrInfo, results: *mut *mut AddrInfo) -> i32;
    pub fn freeaddrinfo(res: *mut AddrInfo);
    pub fn inet_ntop(family: i32, src: c_void, dst: PSTR, size: usize) -> PCSTR;
}