mod common;
use common::traits::I32Enum;

mod http;
use http::Uri;

use metalberto_derived::I32Enum;

#[derive(I32Enum, Debug)]
enum TestEnum {
    One = 1,
    Two = 2,
    Three = 3
}

fn main() {
    let literal = "https://john:doe@my-site.com:1234/hello?source=li&device=mobile#test123";
    let uri = Uri::from(literal).unwrap();
    println!("Uri parsing: '{}'", literal);
    uri.debug_print();
    println!("Scheme default port: {}", uri.scheme_default_port().unwrap());
}