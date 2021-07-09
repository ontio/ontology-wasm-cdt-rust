pub use ontio_codegen::base58;
pub use ontio_codegen::contract;
pub use ontio_codegen::event;
pub use ontio_codegen::keccak256;

#[cfg(test)]
mod tests {
    use crate as ontio_std;
    use ontio_std::abi::{Decoder, Encoder};
    use ontio_std::types::to_hex_string;

    #[derive(Encoder, Decoder)]
    struct Oep4 {
        from: u32,
        to: u32,
        amt: u32,
    }

    #[derive(Encoder, Decoder)]
    enum Token {
        Oep4(Oep4),
    }

    use ontio_std::prelude::*;

    #[ontio_std::macros::contract]
    trait TestContract {
        fn mut_self(&mut self, owner: Address) -> bool;
        fn ref_self(&self) -> String;
        fn multi_param(&mut self, from: Address, to: Address, amount: U128) -> bool;
        fn ref_param(&mut self, owner: &Address) -> bool;
        fn slice_param(&mut self, addrs: &[Address]) -> bool;
        fn mut_param(&mut self, owner: &mut Address) -> bool;
        fn mut_slice_param(&mut self, owner: &mut [Address]) -> bool;
        fn str_param(&mut self, owner: &str) -> bool;

        #[event]
        fn Event(&self, from: Address, to: Address, amount: U128) {}
        #[event]
        fn RefParam(&self, from: &Address, to: Address, amount: U128) {}
        #[event]
        fn SliceParam(&self, from: &[Address]) {}
    }

    #[test]
    fn base58() {
        const _ADDR: Address = ontio_std::macros::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhPbdYzM");
    }

    #[test]
    fn keccak256_test() {
        let data: H256 = ontio_std::macros::keccak256!("transfer(address,uint256)");
        assert_eq!(
            to_hex_string(data.as_bytes()),
            "a9059cbb2ab09eb219583f4a59a5d0623ade346d962bcd4e46b11da047c9049b"
        );
    }

    mod notify {
        use crate as ontio_std;
        use ontio_std::types::{Address, U128};

        #[ontio_std::macros::event]
        fn transfer(from: &Address, to: &Address, amount: U128) {}

        #[ontio_std::macros::event]
        fn transfer_name(from: &Address) {}

        #[ontio_std::macros::event(name=transfer_test)]
        fn transfer_name2(from: &Address) {}
    }

    #[test]
    fn event() {}
}
