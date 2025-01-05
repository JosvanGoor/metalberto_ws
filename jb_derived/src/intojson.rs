use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Ident, Type};

use crate::error::{JbDeriveError, JbDeriveResult};

struct Field {
    ident:     Ident,
    path:      String,
    is_option: bool,
}

impl Field {
    fn new(ident: Ident, path: String, is_option: bool) -> Self {
        Self { ident,
               path,
               is_option }
    }
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
             Ok(Field::new(ident.clone(), path, is_option))
         })
         .collect()
}

// MARK: Emitting
fn emit_assignment(field: &Field) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    let ident_str = field.ident.to_string();
    quote! { json.insert(String::from(#ident_str), self.#ident.into_json()); }
}

// MARK: Impl
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
            fn into_json(self) -> Value {
                let mut json: HashMap<String, Value> = HashMap::new();
                #(#assignments)*
                Value::Dict(json)
            }
        }
    };

    // println!("{}", token_stream);
    Ok(token_stream.into())
}
