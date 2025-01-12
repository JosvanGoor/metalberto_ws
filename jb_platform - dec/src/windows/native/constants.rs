/*
    Constants
*/

pub const AF_INET: i32 = 2;
pub const AF_INET6: i32 = 23;
pub const AF_UNSPEC: i32 = 0;
pub const SOCK_DGRAM: i32 = 2;
pub const SOCK_STREAM: i32 = 1;
pub const INADDR_ANY: u32 = 0;
pub const INET6_ADDRSTRLEN: usize = 65;
pub const IPPROTO_TCP: i32 = 6;
pub const IPPROTO_UDP: i32 = 17;
pub const WSADESCRIPTION_LEN: usize = 256 + 1;
pub const WSASYS_STATUS_LEN: usize = 128 + 1;
pub const WSA_VERSION_2_2: u16 = 2 << 8 | 2;