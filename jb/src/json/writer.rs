use std::collections::HashMap;
use std::io::{Result, Write};

use super::Value;

fn write_array(output: &mut impl Write, array: &[Value]) -> Result<()> {
    write!(output, "[")?;
    
    if !array.is_empty() {
        write(output, &array[0])?;
    }

    for value in array.iter().skip(1) {
        write!(output, ",")?;
        write(output, value)?;
    }

    write!(output, "]")
}

fn write_dict(output: &mut impl Write, dict: &HashMap<String, Value>) -> Result<()> {
    write!(output, "{{")?;
    let mut iter = dict.iter();

    if let Some((key, value)) = iter.next() {
        write!(output, "\"{}\":", key)?;
        write(output, value)?;
    }

    for (key, value) in iter {
        write!(output, ",\"{}\":", key)?;
        write(output, value)?;
    }

    write!(output, "}}")
}

fn write(output: &mut impl Write, value: &Value) -> Result<()> {
    match value {
        Value::Array(vec) => write_array(output, vec),
        Value::Boolean(bool) => write!(output, "{}", bool),
        Value::Dict(hash_map) => write_dict(output, hash_map),
        Value::Float(float) => write!(output, "{}", float),
        Value::Integer(int) => write!(output, "{}", int),
        Value::Null => write!(output, "null"),
        Value::String(string) => write!(output, "\"{}\"", string), // TODO: escape
    }
}

pub fn write_to_json(output: &mut impl Write, value: &Value) -> Result<()> {
    write(output, value)
}