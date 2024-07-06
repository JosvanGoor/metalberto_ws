
pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_DEL: i32 = 2;
pub const EPOLL_CTL_MOD: i32 = 3;

pub const EPOLLIN: i32 = 0x001;
pub const EPOLLPRI: i32 = 0x002;
pub const EPOLLOUT: i32 = 0x004;
pub const EPOLLRDNORM: i32 = 0x040;
pub const EPOLLRDBAND: i32 = 0x080;
pub const EPOLLWRNORM: i32 = 0x100;
pub const EPOLLWRBAND: i32 = 0x200;
pub const EPOLLMSG: i32 = 0x400;
pub const EPOLLERR: i32 = 0x008;
pub const EPOLLHUP: i32 = 0x010;
pub const EPOLLRDHUP: i32 = 0x2000;
pub const EPOLLEXCLUSIVE: i32 = 1 << 28;
pub const EPOLLWAKEUP: i32 = 1 << 29;
pub const EPOLLONESHOT: i32 = 1 << 30;
pub const EPOLLEt: i32 = 1 << 31;

#[derive(Debug)]
#[repr(C, packed)]
pub struct EpollEvent {
    pub(crate) events: u32,
    pub(crate) epoll_tag: usize
}

#[link(name = "c")]
extern "C" {
    pub fn epoll_create(size: i32) -> i32;
    pub fn epoll_ctl(epfd: i32, op: i32, fd: i32, event: *mut EpollEvent) -> i32;
    pub fn epoll_wait(epfd: i32, events: *mut EpollEvent, max_events: i32, timeout_ms: i32) -> i32;
    
    pub fn close(fd: i32) -> i32;
}