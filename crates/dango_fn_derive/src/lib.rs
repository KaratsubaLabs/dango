extern crate proc_macro;

use std::any::{Any, TypeId};

use proc_macro::TokenStream;
use quote::quote;
use serde_json::{json, Value};
use syn::{parse::Parse, parse_macro_input, Expr, Fields, Ident, Item, ItemStruct, Meta};

mod jsonschema;

#[proc_macro_derive(Function)]
pub fn function(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as Item);

    if let Item::Struct(item) = parsed_input {
        impl_function(item)
    } else {
        quote! {
            compile_error!("not used on struct")
        }
        .into()
    }
}

fn impl_function(item: ItemStruct) -> TokenStream {
    let struct_name = &item.ident;

    match item.fields {
        Fields::Named(ref inner) => {
            let fields: Vec<(String, TokenStream)> = inner
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().unwrap().to_string();

                    // filter all doc comments and concat them?
                    let description = field
                        .attrs
                        .iter()
                        .find_map(|attr| {
                            if attr.path().is_ident("doc") {
                                let doc: Expr = attr.parse_args().unwrap();
                                Some(doc)
                            } else {
                                None
                            }
                        })
                        .expect("Expected description");

                    let schema = if field.type_id() == TypeId::of::<String>() {
                        jsonschema::string(description)
                    } else {
                        quote! { compile_error!("Unexpected Type") }.into()
                    };

                    (field_name, schema)
                })
                .collect();

            let properties = quote! {
                Map::from_iter()
            };

            quote! {
                json! {
                    "name": struct_name,
                    "description": ,
                    "parameters": {
                        "type": "object",
                        "properties": {
                        }
                    }
                }
            }
        },
        _ => quote! {
            compile_error!("Currently only supports named structs")
        },
    }
    .into()
}
