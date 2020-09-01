#![recursion_limit = "256"]
#![feature(proc_macro_hygiene)]

extern crate proc_macro;
use heck::MixedCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Lit};

mod base58;
mod contract;
mod event;

#[proc_macro_attribute]
pub fn contract(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).unwrap();
    let stream = contract::quote(item);

    stream.into()
}

#[proc_macro_attribute]
pub fn event(metadata: TokenStream, input: TokenStream) -> TokenStream {
    match syn::parse::<syn::Item>(input).unwrap() {
        syn::Item::Fn(ref func) => {
            use quote::ToTokens;
            let mut method_name;
            if metadata.is_empty() {
                method_name = func.sig.ident.clone().into_token_stream().to_string();
            } else {
                method_name = metadata.to_string();
                method_name = method_name.replace("name", "");
                method_name = method_name.replace("=", "");
            }
            method_name = method_name.to_mixed_case();
            let stream = event::quote(method_name, func);
            stream.into()
        }
        _ => panic!("Only fn is allowed"),
    }
}

#[proc_macro]
pub fn base58(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Lit);
    let addr = match input {
        syn::Lit::Str(lit_str) => base58::decode_base58(&lit_str.value())
            .unwrap_or_else(|e| panic!("failed to parse base58 address: {}", e)),
        syn::Lit::ByteStr(lit_str) => {
            base58::decode_base58(&String::from_utf8(lit_str.value()).unwrap())
                .unwrap_or_else(|e| panic!("failed to parse base58 address: {}", e))
        }
        _ => panic!("base58! only support string literal"),
    };
    let result = quote! { Address::new([#(#addr),*])};

    result.into()
}
