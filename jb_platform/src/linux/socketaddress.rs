use std::ffi::{c_void, CStr, CString};
use std::io::{Error, ErrorKind};
use super::ffi::net::{
    freeaddrinfo, gai_strerror, getaddrinfo, inet_ntop, AddrInfo, SockAddrIn, SockAddrIn6, AF_INET, AF_INET6, AF_UNSPEC, INADDR_ANY, INET6_ADDRSTRLEN
};

#[derive(Debug)]
pub enum SocketAddress {
    Ipv4(SockAddrIn),
    Ipv6(SockAddrIn6)
}

impl Default for SocketAddress {
    fn default() -> Self {
        Self::new()
    }
}

impl SocketAddress {

    pub fn new() -> Self {
        SocketAddress::Ipv4(SockAddrIn::default())
    }

    pub fn from(host: &str, port: u16) -> std::io::Result<Self> {
        let mut addr = Self::new();
        addr.set_host(host)?;
        addr.set_port(port);
        Ok(addr)
    }

    pub fn ipv4_any(port: u16) -> Self {
        Self::Ipv4(SockAddrIn {
            sin_family: AF_INET as u16,
            sin_port: port.to_be(),
            sin_addr: INADDR_ANY,
            sin_zero: Default::default(),
        })
    }

    pub fn ipv6_any(port: u16) -> Self {
        let mut addr = SockAddrIn6::default();
        addr.sin6_family = AF_INET6 as u16;
        addr.sin6_port = port.to_be();
        Self::Ipv6(addr)
    }

    pub fn family(&self) -> u16 {
        match self {
            SocketAddress::Ipv4(ref ipv4) => ipv4.sin_family,
            SocketAddress::Ipv6(ref ipv6) => ipv6.sin6_family,
        }
    }

    pub fn port(&self) -> u16 {
        match self {
            SocketAddress::Ipv4(ref ipv4) => ipv4.sin_port.to_le(),
            SocketAddress::Ipv6(ref ipv6) => ipv6.sin6_port.to_le(),
        }
    }

    pub fn set_port(&mut self, port: u16) {
        match self {
            SocketAddress::Ipv4(ref mut ipv4) => ipv4.sin_port = port.to_be(),
            SocketAddress::Ipv6(ref mut ipv6) => ipv6.sin6_port = port.to_be(),
        }
    }

    pub fn set_host(&mut self, hostname: &str) -> std::io::Result<()> {
        let mut hints = AddrInfo::default();
        hints.ai_family = AF_UNSPEC as i32;
        
        let hostname: CString = CString::new(hostname)?;
        let mut results: *mut AddrInfo = std::ptr::null_mut();
        let result = unsafe { getaddrinfo(hostname.as_ptr(), std::ptr::null(), &hints, &mut results) };
        if result != 0 {
            unsafe { freeaddrinfo(results); }
            let message = String::from(unsafe { CStr::from_ptr(gai_strerror(result)) }.to_str().unwrap());
            return Err(Error::other(message));
        }

        unsafe {
                match (*results).ai_family {
                AF_INET => *self = SocketAddress::Ipv4(*((*results).ai_addr as *mut SockAddrIn)),
                AF_INET6 => *self = SocketAddress::Ipv6(*((*results).ai_addr as *const SockAddrIn6)),
                _ => {
                    freeaddrinfo(results);
                    return Err(Error::from(ErrorKind::AddrNotAvailable));
                }
            }
        }

        unsafe { freeaddrinfo(results); }
        Ok(())
    }

    fn addr_size(&self) -> usize {
        match self {
            SocketAddress::Ipv4(_) => std::mem::size_of::<SockAddrIn>(),
            SocketAddress::Ipv6(_) => std::mem::size_of::<SockAddrIn6>(),
        }
    }

    pub fn hostname(&self) -> String {
        let mut buffer = [0u8; INET6_ADDRSTRLEN];
        let addr_ptr = match self {
            SocketAddress::Ipv4(ipv4) => &ipv4.sin_addr as *const u32 as *const c_void,
            SocketAddress::Ipv6(ipv6) => ipv6.sin6_addr.as_ptr() as *const c_void,
        };

        let result = unsafe { inet_ntop(self.family() as i32, addr_ptr, buffer.as_mut_ptr(), buffer.len() as u32) };
        if result == std::ptr::null::<u8>() {
            return String::from("Failed to find hostname");
        }

        let end = buffer.iter().position(|b| *b == 0).or_else(|| Some(0)).unwrap();
        String::from(std::str::from_utf8(&buffer[0..end]).unwrap())
    }

}