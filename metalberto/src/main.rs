mod common;
use common::traits::I32Enum;

mod http;
use http::enums::HttpResponseStatusCode;

use metalberto_derived::I32Enum;



#[derive(I32Enum, Debug)]
enum TestEnum {
    One = 1,
    Two = 2,
    Three = 3
}

fn main() {
    println!("TestEnum::One: {}", TestEnum::One.into_i32());
    println!("TestEnum::Two: {}", TestEnum::Two.into_i32());
    println!("TestEnum::Three: {}", TestEnum::Three.into_i32());
    println!("1: {:?}", TestEnum::from_i32(2).unwrap());

    println!("404: {:?}", HttpResponseStatusCode::from_i32(404).unwrap());
}