#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Default)]
pub struct tm {
    tm_sec:   i32,
    tm_min:   i32,
    tm_hour:  i32,
    tm_mday:  i32,
    tm_mon:   i32,
    tm_year:  i32,
    tm_wday:  i32,
    tm_yday:  i32,
    tm_isdst: i32,
}
