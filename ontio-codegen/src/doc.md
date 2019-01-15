`
#[test]
fn smoke() {
    let origin = quote! {
        trait Token {
            fn transfer(&mut self, from:u32, to:u32, value:u32) -> bool;
            fn balance_of(&self, owner:u32) -> u32;
            fn name(&self) -> String {
                "TestToken".to_string()
            }

            #[event]
            fn Transfer(&mut self, from:u32, to:u32, value:u32);
        }
    }
    .into();
    let expected = quote! {
        pub trait Token {
            fn transfer(&mut self, from: u32, to: u32, value: u32) -> bool;
            fn balance_of(&self, owner: u32) -> u32;
            fn name(&self) -> String {
                "TestToken".to_string()
            }
            fn Transfer(&mut self, from: u32, to: u32, value: u32) {
                let mut sink = ontio_std::abi::Sink::new(16);
                sink.write::<u32>(from);
                sink.write::<u32>(to);
                sink.write::<u32>(value);
                ontio_std::runtime::notify(&sink.into());
            }
        }
        pub struct Dispatcher<T: Token> {
            pub(crate) contract_instance: T,
        }
        impl<T: Token> Dispatcher<T> {
            pub fn new(cont: T) -> Self {
                Self {
                    contract_instance: cont,
                }
            }
            pub fn instance(&self) -> &T {
                &self.contract_instance
            }
        }
        impl<T: Token> ontio_std::abi::Dispatcher for Dispatcher<T> {
            fn dispatch(&mut self, payload: &[u8]) -> Vec<u8> {
                let contract_instance = &mut self.contract_instance;
                let mut source = ontio_std::abi::Source::new(payload.to_vec());
                let action = source.read::<String>().unwrap();
                let arg_decode_err = "failed to decode argument";
                match action.as_str() {
                    "transfer" => {
                        let res = contract_instance.transfer(
                            source.read::<u32>().expect(arg_decode_err),
                            source.read::<u32>().expect(arg_decode_err),
                            source.read::<u32>().expect(arg_decode_err),
                        );
                        let mut sink = ontio_std::abi::Sink::new(16);
                        sink.write(res);
                        sink.into()
                    }
                    "balance_of" => {
                        let res = contract_instance.balance_of(source.read::<u32>().expect(arg_decode_err));
                        let mut sink = ontio_std::abi::Sink::new(16);
                        sink.write(res);
                        sink.into()
                    }
                    "name" => {
                        let res = contract_instance.name();
                        let mut sink = ontio_std::abi::Sink::new(16);
                        sink.write(res);
                        sink.into()
                    }
                    _ => panic!("invoke unsupported method"),
                }
            }
        }
    };

    let item: syn::Item = syn::parse(origin).unwrap();
    let result = quote(item);

    assert_eq!(result.to_string(), expected.to_string());
}
`
