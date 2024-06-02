mod i32enum;
use i32enum::i32_enum_impl;

use proc_macro::TokenStream;

#[proc_macro_derive(I32Enum)]
pub fn i32enum(input: TokenStream) -> TokenStream {
    i32_enum_impl(input)
}