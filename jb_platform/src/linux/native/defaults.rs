use super::structs::{AddrInfo, SockAddr};

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

impl Default for SockAddr {
    fn default() -> Self {
        Self { sa_family: Default::default(), sa_data: [0u8; 14] }
    }
}