mod i32enum;
use i32enum::i32_enum_impl;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(I32Enum)]
pub fn i32enum(input: TokenStream) -> TokenStream {
    match i32_enum_impl(input) {
        Ok(stream) => stream,
        Err(err) => {
            let message = format!("{:?}", err);
            quote! { compile_error!(#message); }.into()
        },
    }
}