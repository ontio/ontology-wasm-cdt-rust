#[ontio_codegen::contract]
trait Token {
    fn transfer(&mut self, from: u32, to: u32, value: u32) -> bool;
    fn balance_of(&mut self, owner: u32) -> u32;
    fn name(&mut self) -> String {
        "TestToken".to_string()
    }

    #[event]
    fn Transfer(&mut self, from: u32, to: u32, value: u32);
}

#[test]
fn works() {
    //    wrapped_function();
    panic!("heheh")
}

