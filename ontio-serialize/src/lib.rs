#![recursion_limit = "256"]
#![feature(proc_macro_hygiene)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, Fields, Lit};

#[proc_macro_derive(Encoder)]
pub fn derive_encoder(item: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(item).unwrap();
    let name = &ast.ident; //struct name
    let expanded = match ast.data {
        Data::Struct(DataStruct { ref fields, .. }) => {
            if let Fields::Named(ref fields_name) = fields {
                let get_selfs: Vec<_> = fields_name
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.as_ref().unwrap(); // 字段名字
                        quote! {
                            &self.#field_name
                        }
                    })
                    .collect();
                let implemented_encoder = quote! {
                    impl ontio_std::abi::Encoder for #name {
                        fn encode(&self, sink: &mut ontio_std::abi::Sink) {
                             sink.write((#(#get_selfs),*));
                        }
                    }
                };
                implemented_encoder
            } else {
                panic!("not struct");
            }
        }
        _ => panic!("not support"),
    };
    expanded.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
