use std::ffi::{c_void, CString};
use std::io::{Error, ErrorKind};

use crate::constants::{AF_INET, AF_INET6, AF_UNSPEC, INADDR_ANY, INET6_ADDRSTRLEN};
use crate::functions::{freeaddrinfo, getaddrinfo, inet_ntop};
use crate::helpers::WinSock;
use crate::structs::{AddrInfo, SockAddrIn, SockAddrIn6};


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

impl Drop for SocketAddress {
    fn drop(&mut self) {
        WinSock::cleanup();
    }
}

impl SocketAddress {

    pub fn new() -> Self {
        WinSock::startup();
        SocketAddress::Ipv4(SockAddrIn::default())
    }

    pub fn from(host: &str, port: u16) -> std::io::Result<Self> {
        let mut addr = Self::new();
        addr.set_host(host)?;
        addr.set_port(port);
        Ok(addr)
    }

    pub fn ipv4_from(host: &str, port: u16) -> std::io::Result<Self> {
        let mut addr = Self::new();
        addr.set_ipv4_host(host)?;
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
        let addr = SockAddrIn6{
            sin6_family: AF_INET6 as u16,
            sin6_port: port.to_be(),
            ..Default::default()
        };
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

    pub fn set_ipv4_host(&mut self, hostname: &str) -> std::io::Result<()> {
        self.set_host_ex(hostname, AF_INET)
    }

    pub fn set_host(&mut self, hostname: &str) -> std::io::Result<()> {
        self.set_host_ex(hostname, AF_UNSPEC)
    }

    fn set_host_ex(&mut self, hostname: &str, family: i32) -> std::io::Result<()> {
        let hints = AddrInfo{
            ai_family: family,
            ..Default::default()
        };
        
        let hostname: CString = CString::new(hostname)?;
        let mut results: *mut AddrInfo = std::ptr::null_mut();
        let result = unsafe { getaddrinfo(hostname.as_ptr(), std::ptr::null(), &hints, &mut results) };
        if result != 0 {
            unsafe { freeaddrinfo(results); }
            let message = String::from("impl FormatMessage needed");
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

        let result = unsafe { inet_ntop(self.family() as i32, addr_ptr, buffer.as_mut_ptr(), buffer.len()) };
        if result.is_null() {
            return String::from("Failed to find hostname");
        }

        let end = buffer.iter().position(|b| *b == 0).unwrap_or(0);
        String::from(std::str::from_utf8(&buffer[0..end]).unwrap())
    }

}