use proc_macro::TokenStream;
use quote::quote;

#[derive(Debug)]
pub enum JbDeriveError {
    SyntaxError(syn::Error),
    StructNotSupported,
    EnumNotSupported,
    UnnamedFieldNotSupported,
    FailedToGetTypePath,
    LiteralMissing,
    LiteralTypeError,
    LiteralParseError,
}

pub type JbDeriveResult<T> = std::result::Result<T, JbDeriveError>;

impl From<syn::Error> for JbDeriveError {
    fn from(value: syn::Error) -> Self {
        Self::SyntaxError(value)
    }
}

impl From<JbDeriveError> for TokenStream {
    fn from(value: JbDeriveError) -> Self {
        let message = format!("{:?}", value);
        quote! { compile_error!(#message); }.into()
    }
}