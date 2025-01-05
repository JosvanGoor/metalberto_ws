use super::Value;

pub trait IntoJson {
    fn into_json(self) -> Value;
}

macro_rules! into_json_fundamentals {
    ($type:ty) => {
        impl IntoJson for $type {
            fn into_json(self) -> Value {
                self.into()
            }
        }
    };
}

into_json_fundamentals!(i8);
into_json_fundamentals!(u8);
into_json_fundamentals!(i16);
into_json_fundamentals!(u16);
into_json_fundamentals!(i32);
into_json_fundamentals!(u32);
into_json_fundamentals!(i64);
into_json_fundamentals!(u64);
into_json_fundamentals!(i128);
into_json_fundamentals!(u128);
into_json_fundamentals!(f32);
into_json_fundamentals!(f64);
into_json_fundamentals!(bool);
into_json_fundamentals!(String);

impl<T: IntoJson> IntoJson for Option<T> {
    fn into_json(self) -> Value {
        match self {
            Some(value) => value.into_json(),
            None => Value::Null,
        }
    }
}

impl<T: IntoJson> IntoJson for Vec<T> {
    fn into_json(self) -> Value {
        let mut array: Vec<Value> = Vec::with_capacity(self.len());
        for value in self.into_iter() {
            array.push(value.into_json());
        }
        Value::Array(array)
    }
}
