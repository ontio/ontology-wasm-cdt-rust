#![recursion_limit = "256"]
#![feature(proc_macro_hygiene)]

extern crate proc_macro;
use heck::MixedCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DataStruct, Fields};

#[proc_macro_derive(Encoder)]
pub fn derive_encoder(item: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(item).unwrap();
    let name = &ast.ident; //struct name
    let expanded: proc_macro2::TokenStream = match ast.data {
        Data::Enum(DataEnum { ref variants, .. }) => {
            let get_selfs: Vec<_> = variants
                .iter()
                .map(|variant| {
                    let field_name = &variant.ident; // 字段名字
                    quote! {
                        #field_name
                    }
                })
                .collect();
            let implemented_encoder = quote! {
                  impl ontio_std::abi::Encoder for #name {
                    fn encode(&self, sink: &mut ontio_std::abi::Sink) {
                         match self {
                             #(#name::#get_selfs(temp) => {
                                sink.write(stringify!(#get_selfs));
                                sink.write(temp);
                             }), *
                         }
                    }
                  }
            };
            implemented_encoder
        }
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

#[proc_macro_derive(Decoder)]
pub fn derive_decoder(item: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(item).unwrap();
    let name = &ast.ident; //struct name
    let expanded: proc_macro2::TokenStream = match ast.data {
        Data::Enum(DataEnum { ref variants, .. }) => {
            let get_selfs: Vec<_> = variants
                .iter()
                .map(|variant| {
                    let field_name = &variant.ident; // 字段名字
                    let match_name =
                        syn::Ident::new(&field_name.to_string().to_mixed_case(), field_name.span());
                    quote! {
                        #match_name
                    }
                })
                .collect();

            let get_selfs2: Vec<_> = variants
                .iter()
                .map(|variant| {
                    let field_name = &variant.ident; // 字段名字
                    quote! {
                        #field_name
                    }
                })
                .collect();

            let implemented_decoder = quote! {
                impl<'a> ontio_std::abi::Decoder<'a> for #name {
                    fn decode(source: &mut ontio_std::abi::Source<'a>) -> Result<Self,
                    ontio_std::abi::Error> {
                    let ty:&str= source.read()?;
                        match ty {
                             #(stringify!(#get_selfs) => {
                                let temp = source.read()?;
                                Ok(#name::#get_selfs2(temp))
                             }),*
                             _ => {
                             panic!("decoder not support:{}", ty)
                             }
                        }
                    }
                }
            };
            implemented_decoder
        }
        Data::Struct(DataStruct { ref fields, .. }) => {
            if let Fields::Named(ref fields_name) = fields {
                let get_selfs: Vec<_> = fields_name
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.as_ref().unwrap(); // 字段名字
                        quote! {
                            #field_name
                        }
                    })
                    .collect();
                let implemented_decoder = quote! {
                    impl<'a> ontio_std::abi::Decoder<'a> for #name {
                        fn decode(source: &mut ontio_std::abi::Source) -> Result<Self,
                        ontio_std::abi::Error> {
                            return Ok(#name {
                            #(#get_selfs: source.read()?),*
                            })
                        }
                    }
                };
                implemented_decoder
            } else {
                panic!("not struct");
            }
        }
        _ => panic!("not support"),
    };

    expanded.into()
}
