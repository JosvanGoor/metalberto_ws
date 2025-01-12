
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct AddrInfo {
    pub ai_flags: i32,
    pub ai_family: i32,
    pub ai_socktype: i32,
    pub ai_protocol: i32,
    pub ai_addrlen: u32,
    pub ai_addr: *mut SockAddr,
    pub ai_canonname: *const u8,
    pub ai_next: *const AddrInfo,
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct EpollEvent {
    pub events: u32,
    pub epoll_tag: usize
}

#[derive(Debug)]
#[repr(C)]
pub struct PollFd {
    fd: i32,
    events: i16,
    events_out: i16
}

#[derive(Copy, Clone, Debug)]
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