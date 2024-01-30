use std::option::Option;

pub trait I32Enum {
    fn into_i32(&self) -> i32;
    fn from_i32(value: i32) -> Option<Self> where Self: Sized;
}
