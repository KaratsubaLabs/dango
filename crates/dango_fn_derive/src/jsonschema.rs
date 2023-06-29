use proc_macro::TokenStream;
use quote::quote;
use serde_json::{json, Value};
use syn::Expr;

pub fn string(description: Expr) -> TokenStream {
    quote! {
        json! {
            {
                "type": "string",
                "description": #description
            }
        }
    }
    .into()
}
