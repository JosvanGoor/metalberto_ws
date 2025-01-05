use jb::json::Value;
use jb::json::{FromJson, IntoJson};
use std::collections::HashMap;

#[derive(IntoJson)]
struct Test {
    field1: i32,
    field2: Option<f64>
}

fn main() {
    let mut array: Vec<Value> = Vec::new();
    for idx in 0..12 {
        array.push(Value::Integer(idx));
    }

    let value = Value::Array(array);
    let integers: Vec<i32> = Vec::from_json(value).unwrap();
    println!("{:?}", integers);

    let values = integers.into_json();
    println!("{:?}", values);

    println!("{:?}", Test { field1: 12, field2: Some(33.4)}.into_json());
}