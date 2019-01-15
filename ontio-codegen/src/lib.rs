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
}
