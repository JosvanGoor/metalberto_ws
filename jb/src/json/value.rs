use std::collections::HashMap;

use super::error::{JsonErrorType, JsonMappingError};

#[derive(Debug, Clone)]
pub enum Value {
    Array(Vec<Value>),
    Boolean(bool),
    Dict(HashMap<String, Value>),
    Float(f64),
    Integer(i128),
    Null,
    String(String),
}

impl Value {
    pub fn borrow_array(&self) -> Result<&Vec<Value>, JsonMappingError> {
        match self {
            Value::Array(value) => Ok(value),
            _ => Err(JsonMappingError::TypeMismatch),
        }
    }

    pub fn borrow_boolean(&self) -> Result<&bool, JsonMappingError> {
        match self {
            Value::Boolean(value) => Ok(value),
            _ => Err(JsonMappingError::TypeMismatch),
        }
    }

    pub fn borrow_dict(&self) -> Result<&HashMap<String, Value>, JsonMappingError> {
        match self {
            Value::Dict(value) => Ok(value),
            _ => Err(JsonMappingError::TypeMismatch),
        }
    }

    pub fn borrow_float(&self) -> Result<&f64, JsonMappingError> {
        match self {
            Value::Float(value) => Ok(value),
            _ => Err(JsonMappingError::TypeMismatch),
        }
    }

    pub fn borrow_integer(&self) -> Result<&i128, JsonMappingError> {
        match self {
            Value::Integer(value) => Ok(value),
            _ => Err(JsonMappingError::TypeMismatch),
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn borrow_string(&self) -> Result<&String, JsonMappingError> {
        match self {
            Value::String(value) => Ok(value),
            _ => Err(JsonMappingError::TypeMismatch),
        }
    }
}

//MARK: Conversion
impl TryFrom<Value> for Vec<Value> {
    type Error = JsonMappingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Array(array) = value {
            Ok(array)
        } else {
            Err(JsonMappingError::TypeMismatch)
        }
    }
}

impl TryFrom<Value> for f32 {
    type Error = JsonMappingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Float(float) = value {
            Ok(float as f32)
        } else {
            Err(JsonMappingError::TypeMismatch)
        }
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Value::Float(value as f64)
    }
}

impl TryFrom<Value> for f64 {
    type Error = JsonMappingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Float(float) = value {
            Ok(float)
        } else {
            Err(JsonMappingError::TypeMismatch)
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl TryFrom<Value> for bool {
    type Error = JsonMappingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Boolean(boolean) = value {
            Ok(boolean)
        } else {
            Err(JsonMappingError::TypeMismatch)
        }
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl TryFrom<Value> for String {
    type Error = JsonMappingError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::String(string) = value {
            Ok(string)
        } else {
            Err(JsonMappingError::TypeMismatch)
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

macro_rules! try_from_integer {
    ($type:ty) => {
        impl TryFrom<Value> for $type {
            type Error = JsonMappingError;

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                if let Value::Integer(integer) = value {
                    Ok(integer as $type)
                } else {
                    Err(JsonMappingError::TypeMismatch)
                }
            }
        }

        impl From<$type> for Value {
            fn from(value: $type) -> Self {
                Value::Integer(value as i128)
            }
        }
    };
}

try_from_integer!(u8);
try_from_integer!(u16);
try_from_integer!(u32);
try_from_integer!(u64);
try_from_integer!(u128);
try_from_integer!(i8);
try_from_integer!(i16);
try_from_integer!(i32);
try_from_integer!(i64);
try_from_integer!(i128);
