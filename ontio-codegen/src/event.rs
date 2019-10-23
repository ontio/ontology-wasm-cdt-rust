use quote::quote;
use quote::ToTokens;
use syn::FnArg;

pub fn quote(method_name: String, func: &syn::ItemFn) -> proc_macro2::TokenStream {
    let name = func.sig.ident.clone();
    let inputs = func.sig.inputs.iter().map(|v| {
        quote! { #v }
    });
    // Extract parameters, which may be multiple
    let params: Vec<_> = func
        .sig
        .inputs
        .iter()
        .filter_map(|i| {
            match i {
                // https://docs.rs/syn/1.0.1/syn/struct.PatType.html
                FnArg::Typed(ref val) => Some((val.pat.clone(), val.ty.clone())),
                _ => unreachable!("it's not gonna happen."),
            }
        })
        .collect();
    let head = quote! {
        let mut es = ontio_std::abi::EventBuilder::new();
        es = es.string(#method_name);
    };
    let body = params.iter().map(|&(ref pat, ref ty)| {
        let mut param_type = ty.into_token_stream().to_string();
        param_type = param_type.replace(" ", "");
        match param_type.as_str() {
            "Address" | "&Address" => quote! {es = es.address(#pat)},
            "U128" => quote! {es = es.number(#pat)},
            "&str" => quote! {es = es.string(#pat)},
            "&[u8]" => quote! {es = es.bytearray(#pat)},
            "bool" => quote! {es = es.bool(#pat)},
            "H256" => quote! {es = es.h256(#pat)},
            _ => panic!("not support type"),
        }
    });

    let gen = quote! {
       pub fn #name ( #( #inputs),* ) {
           #head
           #(#body;)*
           es.notify();
       }
    };
    gen.into_token_stream()
}
