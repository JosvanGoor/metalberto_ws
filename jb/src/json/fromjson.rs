use std::collections::HashMap;

use super::{JsonMappingError, Value};

pub trait FromJson where Self: Sized {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError>;
}

macro_rules! from_json_fundamentals {
    ($type:ty) => {
        impl FromJson for $type {
            fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
                Self::try_from(value)
            }
        }
    };
}

from_json_fundamentals!(i8);
from_json_fundamentals!(u8);
from_json_fundamentals!(i16);
from_json_fundamentals!(u16);
from_json_fundamentals!(i32);
from_json_fundamentals!(u32);
from_json_fundamentals!(i64);
from_json_fundamentals!(u64);
from_json_fundamentals!(i128);
from_json_fundamentals!(u128);
from_json_fundamentals!(f32);
from_json_fundamentals!(f64);
from_json_fundamentals!(bool);
from_json_fundamentals!(String);

impl<T: FromJson> FromJson for Option<T> {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
        if matches!(value, Value::Null) {
            return Ok(None)
        }
        Ok(Some(T::from_json(value)?))
    }
}

impl<T: FromJson> FromJson for Vec<T> {
    fn from_json(value: Value) -> std::result::Result<Self, JsonMappingError> {
        if let Value::Array(array) = value {
            let mut rval = Vec::with_capacity(array.len());
            for value in array.into_iter() {
                rval.push(T::from_json(value)?);
            }
            Ok(rval)
        } else {
            Err(JsonMappingError::TypeMismatch)
        }
    }
}