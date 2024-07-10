
pub(crate) fn string_from_cstr(cstr: *const u8) -> Result<String, Utf8Error> {
    let wrapped = unsafe {  };
    String::from(CStr::from_raw(cstr).to_str().unwrap());
}