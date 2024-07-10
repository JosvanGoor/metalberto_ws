#![cfg(target_os = "linux")]

pub(crate) mod ffi;

mod epoll;
pub use epoll::Epoll;

mod socketaddress;
pub use socketaddress::SocketAddress;