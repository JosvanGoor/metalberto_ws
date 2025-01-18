mod error;

mod i32_enum;
use i32_enum::i32_enum_impl;

mod into_json;
use into_json::into_json_impl;

mod from_json;
use from_json::from_json_impl;

use proc_macro::TokenStream;

#[proc_macro_derive(I32Enum)]
pub fn i32enum(input: TokenStream) -> TokenStream {
    match i32_enum_impl(input) {
        Ok(stream) => stream,
        Err(err) => err.into(),
    }
}

#[proc_macro_derive(IntoJson)]
pub fn into_json(input: TokenStream) -> TokenStream {
    match into_json_impl(input) {
        Ok(stream) => stream,
        Err(err) => err.into(),
    }
}

#[proc_macro_derive(FromJson)]
pub fn from_json(input: TokenStream) -> TokenStream {
    match from_json_impl(input) {
        Ok(stream) => stream,
        Err(err) => err.into(),
    }
}