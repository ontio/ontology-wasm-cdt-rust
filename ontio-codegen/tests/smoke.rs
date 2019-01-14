#[ontio_codegen::contract]
trait Token {
    fn transfer(&mut self, from: u32, to: u32, value: u32);
    fn has_return(&mut self, from: u64, to: u32, value: u32) -> u64;
    fn return_tuple(&mut self, from: u64, to: u32, value: u32) -> (u32, u64);

    #[event]
    fn Transfer(&mut self, from: u32, to: u32, value: u32);
}

pub struct Dispatcher;

impl Dispatcher {
    fn transfer(&mut self, from: u32, to: u32, value: u32) {}
}

#[test]
fn works() {
    wrapped_function();
    panic!("heheh")
}

/*
pub struct Dispacher<T: Token> {
    pub(crate) contract_instance: T,
}
impl<T: Token> Dispacher<T> {
    pub fn new(cont: T) -> Self {
        Self {
            contract_instance: cont,
        }
    }
    pub fn instance(&self) -> &T {
        &self.contract_instance
    }
}
impl<T: Token> ontio_std::abi::Dispacher for Dispacher<T> {
    fn dispatch(&mut self, payload: &[u8]) -> Vec<u8> {
        let contract_instance = &mut self.contract_instance;
        let mut source = ontio_std::abi::Source::new(payload.to_vec());
        let action = source.read::<String>().unwrap();
        let arg_decode_err = "failed to decode argument";
        match action.as_str() {
            "transfer" => {
                contract_instance.transfer(
                    source.read::<u32>().expect(arg_decode_err),
                    source.read::<u32>().expect(arg_decode_err),
                    source.read::<u32>().expect(arg_decode_err),
                );
                Vec::new()
            }
            "has_return" => {
                let res = contract_instance.has_return(
                    source.read::<u64>().expect(arg_decode_err),
                    source.read::<u32>().expect(arg_decode_err),
                    source.read::<u32>().expect(arg_decode_err),
                );
                let mut sink = ontio_std::abi::Sink::new(16);
                sink.write(res);
                sink.into()
            }
            "return_tuple" => {
                let res = contract_instance.return_tuple(
                    source.read::<u64>().expect(arg_decode_err),
                    source.read::<u32>().expect(arg_decode_err),
                    source.read::<u32>().expect(arg_decode_err),
                );
                let mut sink = ontio_std::abi::Sink::new(16);
                sink.write(res);
                sink.into()
            }
            _ => panic!("invoke unsupported method"),
        }
    }
}
*/
