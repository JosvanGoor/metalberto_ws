// FOR DEVELOPMENT
#![allow(dead_code)]

#[cfg(target_os = "linux")]
pub(crate) mod linux;
#[cfg(target_os = "linux")]
pub(crate) use linux::native::*;
#[cfg(target_os = "linux")]
pub use linux::net;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;