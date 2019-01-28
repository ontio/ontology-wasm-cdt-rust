#![recursion_limit = "256"]

extern crate proc_macro;
use proc_macro::TokenStream;

mod contract;

#[proc_macro_attribute]
pub fn contract(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).unwrap();
    let stream = contract::quote(item);

    stream.into()
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
}
