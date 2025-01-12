use std::ffi::c_void;

use super::structs::tm;
use super::types::{char_t, errno_t, size_t, time_t};

extern "C" {
    // basic C
    pub fn free(ptr: *mut c_void);

    // time stuff
    pub fn time(opt_out: *mut time_t) -> time_t;
    pub fn localtime(source: *const time_t) -> *mut tm;
    pub fn strftime(dest: *mut char_t, dest_size: size_t, format: *const char_t, time_ptr: *const tm) -> size_t;
}
