mod generate_collection;

use proc_macro::TokenStream;
use quote::quote;

use quote::format_ident;
use syn::{punctuated::Punctuated, *};

use crate::generate_collection::generate_plugin_collection_impl;

/// Please see the [`plugin_collection`](https://github.com/heydocode/typed_ecs/blob/main/examples/plugin_collection.rs) for more details on the usage of this macro.
#[proc_macro]
pub fn generate_collection(input: TokenStream) -> TokenStream {
    let input_clone = input.clone();
    let types = parse_macro_input!(
        input with Punctuated::<Type, Token![,]>::parse_terminated
    );

    let mut quote_fields = Vec::new();
    let mut fields = Vec::new();
    let mut types_s = Vec::new();
    for ty in &types {
        if let Type::Path(type_path) = ty {
            let segment = type_path.path.segments.last().unwrap();
            let ident = &segment.ident;
            let ident_str = ident.to_string();
            let field_name = format_ident!("{}", ident_str.to_lowercase());
            fields.push(field_name.clone());
            quote_fields.push(quote! {
                #field_name: #ty
            });
            types_s.push(ident_str);
        } else {
            panic!(
                "generate_collection! only supports simple path types\n\nReceived tokens:\n{}",
                input_clone
            );
        }
    }

    let fields: Vec<syn::Ident> = fields.iter().map(|s| format_ident!("{}", s)).collect();

    let types: Vec<syn::Ident> = types_s.iter().map(|s| format_ident!("{}", s)).collect();

    let expanded = generate_plugin_collection_impl(fields, types, quote_fields);

    TokenStream::from(expanded)
}
