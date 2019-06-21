#![recursion_limit = "256"]
#![feature(proc_macro_hygiene)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Lit};

mod base58;
mod contract;

#[proc_macro_attribute]
pub fn contract(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).unwrap();
    let stream = contract::quote(item);

    stream.into()
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

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use ontio_std::prelude::*;
    #[ontio_std::abi_codegen::contract]
    trait TestContract {
        fn mut_self(&mut self, owner: Address) -> bool;
        fn ref_self(&self) -> String;
        fn multi_param(&mut self, from: Address, to: Address, amount: U256) -> bool;
        fn ref_param(&mut self, owner: &Address) -> bool;
        fn slice_param(&mut self, addrs: &[Address]) -> bool;
        fn mut_param(&mut self, owner: &mut Address) -> bool;
        fn mut_slice_param(&mut self, owner: &mut [Address]) -> bool;
        fn str_param(&mut self, owner: &str) -> bool;

        #[event]
        fn Event(&self, from: Address, to: Address, amount: U256) {}
        #[event]
        fn RefParam(&self, from: &Address, to: Address, amount: U256) {}
        #[event]
        fn SliceParam(&self, from: &[Address]) {}
    }

    #[test]
    fn base58() {
        const _ADDR: Address = ontio_std::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
    }
}
