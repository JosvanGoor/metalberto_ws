/*
    Implements generic constants, structs and functions
*/

pub(crate) type FileDescriptor = i32;

#[link(name = "c")]
extern "C" {
    pub fn close(fd: FileDescriptor) -> i32; // closes file handle
}