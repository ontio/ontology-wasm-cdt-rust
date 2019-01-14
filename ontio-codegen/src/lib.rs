#![recursion_limit = "128"]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Item};

mod contract;
use self::contract::Contract;

#[proc_macro_attribute]
pub fn contract(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = syn::parse(input).unwrap();
    let stream = contract::quote(item);

    println!("generated token stream {}", stream);
    stream.into()
}

//pub fn contract_from(item: Item) -> Contract {
//    match item {
//        Item::Trait(item_trait) => Contract::from_item_trait(item_trait),
//        _ => panic!("`#[contract]` can only be used on a trait"),
//    }
//}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
