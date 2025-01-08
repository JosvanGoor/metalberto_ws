use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Ident};

use crate::error::{JbDeriveError, JbDeriveResult};

fn gather_fields(input: &DataStruct) -> JbDeriveResult<Vec<Ident>> {
    input.fields
         .iter()
         .map(|field| {
             let Some(ref ident) = field.ident else {
                 return Err(JbDeriveError::UnnamedFieldNotSupported);
             };
             Ok(ident.clone())
         })
         .collect()
}

fn emit_assignment(ident: &Ident) -> proc_macro2::TokenStream {
    let ident_str = ident.to_string();
    quote! { json.insert(String::from(#ident_str), self.#ident.into_json()); }
}

pub fn into_json_impl(input: TokenStream) -> JbDeriveResult<TokenStream> {
    let input: DeriveInput = syn::parse(input)?;

    let struct_ident = input.ident.clone();
    let Data::Struct(struct_info) = input.data else {
        return Err(JbDeriveError::EnumNotSupported);
    };

    let fields = gather_fields(&struct_info)?;
    let assignments = fields.iter().map(emit_assignment).collect::<Vec<_>>();

    let token_stream = quote! {
        impl IntoJson for #struct_ident {
            fn into_json(self) -> jb::json::Value {
                let mut json: std::collections::HashMap<String, jb::json::Value> = std::collections::HashMap::new();
                #(#assignments)*
                jb::json::Value::Dict(json)
            }
        }
    };

    Ok(token_stream.into())
}
