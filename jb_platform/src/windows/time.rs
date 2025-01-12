use std::ffi::c_void;

use crate::ffi::constants::EINVAL;
use crate::ffi::functions::strftime;
use crate::ffi::types::size_t;
use crate::general::onreturn::OnReturn;
use crate::general::result::{FfiError, FfiResult};
use crate::general::strings::{ntbs_to_string, string_to_ntbs};

use super::ffi::functions::{free, localtime, time};
use super::ffi::structs::tm;
use super::ffi::types::time_t;

#[derive(Default, Debug, Clone, Copy)]
pub struct Time(time_t);

pub const LOG_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

impl Time {

    pub fn now() -> Self {
        Self(unsafe { time(std::ptr::null_mut::<time_t>()) })
    }

    pub fn format(&self, format: &str) -> FfiResult<String> {
        let expanded_ptr = unsafe { localtime(&self.0) };

        if expanded_ptr.is_null() {
            return Err(FfiError::FailedToAllocate);
        }

        let mut buffer = [0u8; 256];// todo: dynamically grow if it isn't big enough
        let dest_size = buffer.len() as size_t;
        let format = string_to_ntbs(format);
        
        let written = unsafe { strftime(buffer.as_mut_ptr(), dest_size, format.as_ptr(), expanded_ptr) };
        if written == 0 {
            return Err(FfiError::BufferTooSmall);
        }
        
        Ok(ntbs_to_string(&buffer[..written as usize])?)
    }

}