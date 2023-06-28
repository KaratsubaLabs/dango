extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Fields, Ident, Item, ItemStruct};

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
    match item.fields {
        Fields::Named(ref inner) => {
            for field in inner.named.iter() {}
            quote! {}
        },
        _ => quote! {
            compile_error!("Currently only supports named structs")
        },
    }
    .into()
}
