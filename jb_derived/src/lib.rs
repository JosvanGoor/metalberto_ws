mod error;

mod i32enum;
use i32enum::i32_enum_impl;

mod intojson;
use intojson::into_json_impl;
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
