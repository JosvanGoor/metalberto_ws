use jb::common::I32Enum;
// use jb::http::Uri;
use jb_derived::I32Enum;
use jb::ffi::{CreateIoCompletionPort, GetLastError, Handle, InvalidHandle, DWord, LikePtr, NullPtr};
#[derive(I32Enum, Debug)]
enum TestEnum {
    One = 1,
    Two = 2,
    Three = 3
}

fn main() {
    unsafe {
        let handle = CreateIoCompletionPort(InvalidHandle, InvalidHandle, NullPtr, 0);
        println!("handle: {}", handle);
        println!("last error: {}", GetLastError());
    }

    // let literal = "https://john:doe@my-site.com:1234/hello?source=li&device=mobile#test123";
    // let mut uri = Uri::from(literal).unwrap();
    // println!("Uri parsing: '{}'", literal);
    // uri.set_host("www.google.nl");
    // uri.debug_print();
    // println!("Scheme default port: {}", uri.scheme_default_port().unwrap());
}