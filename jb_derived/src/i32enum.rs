use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit, Variant};

use crate::error::{JbDeriveError, JbDeriveResult};

fn parse_i32_literal(variant: &Variant) -> JbDeriveResult<i32> {
    let expr = match variant.discriminant.as_ref() {
        Some(e) => e,
        None => return Err(JbDeriveError::LiteralMissing),
    };

    let literal = match &expr.1 {
        Expr::Lit(l) => &l.lit,
        _ => return Err(JbDeriveError::LiteralMissing),
    };

    let value = match literal {
        Lit::Int(i) => i.base10_parse::<i32>().or(Err(JbDeriveError::LiteralParseError))?,
        _ => return Err(JbDeriveError::LiteralTypeError),
    };

    Ok(value)
}

pub fn i32_enum_impl(input: TokenStream) -> JbDeriveResult<TokenStream> {
    let input: DeriveInput = syn::parse(input)?;

    let enumerations = match input.data {
        Data::Enum(d) => d,
        _ => {
            return Err(JbDeriveError::StructNotSupported);
        }
    };

    let enum_name = &input.ident;

    let pairs: &Vec<(String, i32)> =
        &enumerations.variants.iter().map(|variant| (variant.ident.to_string(), parse_i32_literal(variant).unwrap())).collect::<Vec<(String, i32)>>();

    let mut into_mapping = Vec::new();
    let mut from_mapping = Vec::new();
    for (name, num) in pairs {
        let id = quote::format_ident!("{}", name);
        into_mapping.push(quote!( #enum_name::#id => #num, ));
        from_mapping.push(quote!( #num => Some(#enum_name::#id), ));
    }

    Ok(quote! {
           impl I32Enum for #enum_name {

               fn into_i32(self) -> i32 {
                   match self {
                       #(#into_mapping)*
                   }
               }

               fn from_i32(value: i32) -> Option<Self> {
                   match value {
                       #(#from_mapping)*
                       _ => None,
                   }
               }
           }
       }.into())
}
