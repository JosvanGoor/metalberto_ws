use crate::structs::AddrInfo;

use super::constants::{WSADESCRIPTION_LEN, WSASYS_STATUS_LEN};
use super::structs::WsaData;

impl Default for WsaData {
    fn default() -> Self {
        Self {
            version: Default::default(),
            high_version: Default::default(),
            max_sockets: Default::default(),
            max_udp_dg: Default::default(),
            vendor_info: std::ptr::null(),
            description: [0u8; WSADESCRIPTION_LEN],
            system_status: [0u8; WSASYS_STATUS_LEN]
        }
    }
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
            ai_next: std::ptr::null_mut()
        }
    }
}