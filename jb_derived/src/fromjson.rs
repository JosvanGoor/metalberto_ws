use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Ident, Type};

use crate::error::{JbDeriveError, JbDeriveResult};

struct Field {
    ident:     Ident,
    path:      Type,
    is_option: bool,
}

// MARK: Parsing
fn type_path(ty: &Type) -> JbDeriveResult<String> {
    match ty {
        Type::Path(type_path) => {
            Ok(type_path.path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<String>>().join("::"))
        }
        _ => Err(JbDeriveError::FailedToGetTypePath),
    }
}

fn is_option(path: &str) -> bool {
    matches!(path, "Option" | "option::Option" | "std::option::Option")
}

fn gather_fields(input: &DataStruct) -> JbDeriveResult<Vec<Field>> {
    input.fields
         .iter()
         .map(|field| {
             let Some(ref ident) = field.ident else {
                 return Err(JbDeriveError::UnnamedFieldNotSupported);
             };
             let path = type_path(&field.ty)?;
             let is_option = is_option(&path);
             Ok(Field { ident: ident.clone(),
                        path: field.ty.clone(),
                        is_option })
         })
         .collect()
}

fn create_option_path(path: &Type) -> proc_macro2::TokenStream {
    let string = quote!{#path}.to_string();
    let (option, t) = string.split_once("<").expect("Option type incomplete?");
    

    // quote! { #option::<#t }.into
    todo!()
}

fn emit_assignment(field: &Field) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    let ident_str = field.ident.to_string();
    let path = &field.path;

    if field.is_option {
        quote! { #ident: jb::json::helpers::get_or_none(&json, #ident_str)?, }
    } else {
        quote! { #ident: jb::json::helpers::get_or_error(&json, #ident_str)?, }
    }

    //(*json.get(#ident_str).ok_or(jb::json::JsonMappingError::FieldError(#ident_str.into()))?).from_json()?
}

pub fn from_json_impl(input: TokenStream) -> JbDeriveResult<TokenStream> {
    let input: DeriveInput = syn::parse(input)?;

    let struct_ident = input.ident.clone();
    let Data::Struct(struct_info) = input.data else {
        return Err(JbDeriveError::EnumNotSupported);
    };

    let fields = gather_fields(&struct_info)?;
    let assignments = fields.iter().map(emit_assignment).collect::<Vec<_>>();

    let token_stream = quote! {
        impl FromJson for #struct_ident {
            fn from_json(value: jb::json::Value) -> std::result::Result<Self, jb::json::JsonMappingError> {
                let jb::json::Value::Dict(json) = value else {
                    return Err(jb::json::JsonMappingError::TypeMismatch);
                };

                Ok(Self {
                    #(#assignments)*
                })
            }
        }
    };

    println!("{}", token_stream);

    Ok(token_stream.into())
}
