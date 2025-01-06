use std::collections::HashMap;

use super::{FromJson, JsonMappingError, Value};


pub fn get_or_error<T: FromJson>(json: &HashMap<String, Value>, name: &str) -> std::result::Result<T, JsonMappingError> {
    let field = json.get(name).ok_or(JsonMappingError::FieldError(name.into()))?;
    T::from_json(field.clone())
}

pub fn get_or_none<T: FromJson>(json: &HashMap<String, Value>, name: &str) -> std::result::Result<Option<T>, JsonMappingError> {
    let field = json.get(name).cloned().unwrap_or(Value::Null);
    Option::<T>::from_json(field)
}