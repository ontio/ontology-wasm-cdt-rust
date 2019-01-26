use proc_macro2::Span;
use quote::quote;

pub fn quote(item: syn::Item) -> proc_macro2::TokenStream {
    match item {
        syn::Item::Trait(item_trait) => {
            let contract = Contract::from_item_trait(item_trait);
            let disp = generate_dispacher(&contract);
            let trait_and_event = generate_event(&contract);
            quote! {
                #trait_and_event
                #disp
            }
        }
        _ => {
            panic!("`#[contract]` can only be used on a trait");
        }
    }
}

#[derive(Debug)]
pub(crate) struct Contract {
    name: syn::Ident,
    fields: Vec<ContractField>,
}

impl Contract {
    pub(crate) fn from_item_trait(item_trait: syn::ItemTrait) -> Self {
        Contract {
            name: item_trait.ident,
            fields: item_trait
                .items
                .into_iter()
                .map(ContractField::from_trait_item)
                .collect(),
        }
    }
}

#[derive(Debug)]
enum ContractField {
    Action(ContractAction),
    Event(ContractEvent),
    Unhandle(syn::TraitItem),
}

fn is_event(method: &syn::TraitItemMethod) -> bool {
    method.attrs.iter().any(|attr| {
        if attr.style == syn::AttrStyle::Outer {
            attr.path
                .is_ident(syn::Ident::new("event", Span::call_site()))
        } else {
            false
        }
    })
}

impl ContractField {
    fn from_trait_item(item: syn::TraitItem) -> Self {
        match item {
            syn::TraitItem::Method(method) => {
                if is_event(&method) {
                    ContractField::Event(ContractEvent::from_trait_method(method))
                } else {
                    ContractField::Action(ContractAction::from_trait_method(method))
                }
            }
            item => ContractField::Unhandle(item),
        }
    }
}

#[derive(Debug)]
struct ContractAction {
    name: syn::Ident,
    params: Vec<(syn::Pat, syn::Type)>,
    ret: Option<syn::Type>,
    method: syn::TraitItemMethod,
}

impl ContractAction {
    fn from_trait_method(method: syn::TraitItemMethod) -> Self {
        let mut m = method.clone();
        m.attrs = Vec::new();

        let params = method
            .sig
            .decl
            .inputs
            .into_iter()
            .filter_map(|arg| match arg {
                syn::FnArg::SelfRef(_) | syn::FnArg::SelfValue(_) => None,
                syn::FnArg::Captured(capt) => Some((capt.pat, capt.ty)),
                _ => panic!("unsupported FnArg type"),
            })
            .collect();
        let ret = match method.sig.decl.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(*ty),
        };

        ContractAction {
            name: method.sig.ident,
            params: params,
            ret: ret,
            method: m,
        }
    }
}

#[derive(Debug)]
struct ContractEvent {
    name: syn::Ident,
    method_sig: syn::MethodSig,
    params: Vec<(syn::Pat, syn::Type)>,
    default: Option<syn::Block>,
}

impl ContractEvent {
    fn from_trait_method(method: syn::TraitItemMethod) -> Self {
        let params = method
            .sig
            .decl
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                &syn::FnArg::SelfRef(_) | &syn::FnArg::SelfValue(_) => None,
                &syn::FnArg::Captured(ref capt) => Some((capt.pat.clone(), capt.ty.clone())),
                _ => panic!("unsupported FnArg type"),
            })
            .collect();
        ContractEvent {
            name: method.sig.ident.clone(),
            method_sig: method.sig,
            params: params,
            default: method.default,
        }
    }
}

fn generate_dispacher(contract: &Contract) -> proc_macro2::TokenStream {
    let actions: Vec<proc_macro2::TokenStream> = contract.fields.iter().filter_map(|field| {
        match field {
            &ContractField::Action(ref action) => {
                let action_name = &action.name;
                let action_literal = syn::LitStr::new(&action_name.to_string(), proc_macro2::Span::call_site());
                let args = action.params.iter().map(|&(_, ref ty)| {
                    match ty {
                        syn::Type::Reference(refer) => {
                            let mutability = refer.mutability.as_ref();
                            let real = *refer.elem.clone();
                            match real {
                                syn::Type::Slice(slice) => {
                                    let slice_elem = &slice.elem;
                                    match mutability {
                                        Some(_) => quote! { source.read::<Vec<#slice_elem>>().expect(arg_decode_err).as_mut_slice() },
                                        None => quote! { source.read::<Vec<#slice_elem>>().expect(arg_decode_err).as_slice() },
                                    }
                                }
                                real => quote! { &#mutability source.read::<#real>().expect(arg_decode_err) },
                            }
                        }
                        ty => {
                            quote! { source.read::<#ty>().expect(arg_decode_err) }
                        }
                    }
                });
                match action.ret {
                    Some(_) => {
                        Some(quote!{
                            #action_literal => {
                                let res = contract_instance.#action_name(#(#args),*);
                                let mut sink = ontio_std::abi::Sink::new(16);
                                sink.write(res);
                                sink.into()
                            }
                        })
                    }
                    None => {
                        Some(quote!{
                            #action_literal => {
                                contract_instance.#action_name(#(source.read::<#args>().expect(arg_decode_err)),*);
                                Vec::new()
                            }
                        })
                    }
                }
            }
            _ => None,
        }
    }).collect();

    let contract_name = &contract.name;

    let dispatcher_name = syn::Ident::new(&format!("{}Dispatcher",contract_name), Span::call_site());

    quote! {
        pub struct #dispatcher_name<T:#contract_name> {
            pub(crate) contract_instance: T,
        }
        impl<T: #contract_name> #dispatcher_name<T> {
            pub fn new(cont: T) -> Self {
                Self{contract_instance: cont}
            }

            pub fn instance(&self) -> &T {
                &self.contract_instance
            }
        }

        impl<T: #contract_name> ontio_std::abi::Dispatcher for #dispatcher_name<T> {
            fn dispatch(&mut self, payload: &[u8]) -> ontio_std::Vec<u8> {
                let contract_instance = &mut self.contract_instance;
                // todo: avoid bytes copy
                let mut source = ontio_std::abi::Source::new(payload.to_vec());
                let action = source.read::<String>().unwrap();
                let arg_decode_err = "failed to decode argument";
                match action.as_str() {
                    #(#actions,)*
                    _ => panic!("invoke unsupported method"),
                }
            }
        }
    }
}

fn generate_event(contract: &Contract) -> proc_macro2::TokenStream {
    let events: Vec<proc_macro2::TokenStream> = contract
        .fields
        .iter()
        .map(|field| match field {
            &ContractField::Event(ref event) => {
                let event_sig = &event.method_sig;
                let event_body = match &event.default {
//                    Some(body) => quote! { #body },
//                    None => {
                    //disable default implementation
                    _ => {
                        let args_type = event.params.iter().map(|&(_, ref ty)| quote! { #ty });
                        let args_name = event.params.iter().map(|&(ref pat, _)| quote! { #pat });
                        quote! { {
                            let mut sink = ontio_std::abi::Sink::new(16);
                            #(sink.write::<#args_type>(#args_name);)*
                            ontio_std::runtime::notify(&sink.into());
                        } }
                    }
                };
                quote! {
                    #[allow(non_snake_case)]
                    #event_sig
                    #event_body
                }
            }
            &ContractField::Action(ref action) => {
                let method = &action.method;
                quote! { #method }
            }
            &ContractField::Unhandle(ref item) => quote! { #item },
        })
        .collect();

    let contract_name = &contract.name;
    quote! {
        pub trait #contract_name {
            #(#events)*
        }

    }
}


