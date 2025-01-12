#![allow(unused)]

// #[cfg(target_os = "linux")]
// pub(crate) mod linux;
// #[cfg(target_os = "linux")]
// pub(crate) use linux::native::*;
// #[cfg(target_os = "linux")]
// pub use linux::net;

// #[cfg(target_os = "windows")]
// mod windows;
// #[cfg(target_os = "windows")]
// pub use windows::*;

// #![cfg(target_os = "linux")]

mod general;
pub use general::result;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::ffi;
pub use windows::time;