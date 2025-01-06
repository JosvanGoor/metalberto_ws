use jb::json::Value;
use jb::json::{FromJson, IntoJson};
use std::collections::HashMap;

#[derive(Debug, FromJson, IntoJson)]
struct SecondaryObject {
    obj1: i32,
    obj2: f32
}

#[derive(Debug, FromJson, IntoJson)]
struct Object {
    val1: i64,
    val2: bool,
    val3: Option<SecondaryObject>,
    val4: Vec<SecondaryObject>
}

#[derive(Debug, FromJson, IntoJson)]
struct Root {
    root: i128,
    list: Vec<f32>,
    text: String,
    object: Option<Object>
}

fn main() {
    let file = std::fs::read_to_string("result.json").unwrap();
    let parsed = jb::json::json_from_string(&file).unwrap();
    let root = Root::from_json(parsed).unwrap();
    println!("root: {:?}", root);
    std::fs::write("output.json", jb::json::json_to_string(&root.into_json())).unwrap();
}