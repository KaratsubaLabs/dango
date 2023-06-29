use proc_macro::TokenStream;
use quote::quote;
use serde_json::{json, Value};
use syn::Expr;

pub fn string(description: Expr) -> proc_macro2::TokenStream {
    quote! {
        json! {
            {
                "type": "string",
                "description": #description
            }
        }
    }
}
