pub mod ont {
    pub struct State {
        pub from: Address,
        pub to: Address,
        pub amount: U256,
    }
    use super::super::base58;
    use super::super::types::{Address, U256};
    const ONT_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");
    pub fn transfer(transfer: &[State]) -> bool {
        super::util::transfer_inner(&ONT_CONTRACT_ADDRESS, transfer)
    }
    pub fn approve(from: &Address, to: &Address, amount: U256) -> bool {
        super::util::approve_inner(&ONT_CONTRACT_ADDRESS, from, to, amount)
    }
    pub fn balance_of(address: &Address) -> U256 {
        super::util::balance_of_inner(&ONT_CONTRACT_ADDRESS, &address)
    }
    pub fn allowance(from: &Address, to: &Address) -> U256 {
        super::util::allowance_inner(&ONT_CONTRACT_ADDRESS, from, to)
    }
    pub fn transfer_from(sender: &Address, from: &Address, to: &Address, amount: U256) -> bool {
        super::util::transfer_from_inner(&ONT_CONTRACT_ADDRESS, sender, from, to, amount)
    }
}

pub mod ong {
    use super::super::base58;
    use super::super::types::{Address, U256};
    const ONG_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhfRZMHJ");
    pub fn transfer(transfer: &[super::ont::State]) -> bool {
        super::util::transfer_inner(&ONG_CONTRACT_ADDRESS, transfer)
    }
    pub fn balance_of(address: &Address) -> U256 {
        super::util::balance_of_inner(&ONG_CONTRACT_ADDRESS, &address)
    }
    pub fn approve(from: &Address, to: &Address, amount: U256) -> bool {
        super::util::approve_inner(&ONG_CONTRACT_ADDRESS, from, to, amount)
    }
    pub fn allowance(from: &Address, to: &Address) -> U256 {
        super::util::allowance_inner(&ONG_CONTRACT_ADDRESS, from, to)
    }
    pub fn transfer_from(sender: &Address, from: &Address, to: &Address, amount: U256) -> bool {
        super::util::transfer_from_inner(&ONG_CONTRACT_ADDRESS, sender, from, to, amount)
    }
}

pub(crate) mod util {
    use super::super::abi::Sink;
    use super::super::runtime;
    use super::super::types::{to_neo_bytes, Address, U256};
    const VERSION: u8 = 0;
    pub(crate) fn transfer_inner(
        contract_address: &Address, transfer: &[super::ont::State],
    ) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_varuint(transfer.len() as u64);

        for state in transfer.iter() {
            sink.write_native_address(&state.from);
            sink.write_native_address(&state.to);
            sink.write(to_neo_bytes(state.amount));
        }
        let mut sink_param = Sink::new(16);
        sink_param.write(VERSION);
        sink_param.write("transfer");
        sink_param.write(sink.into());
        let res = runtime::call_contract(contract_address, sink_param.into().as_slice());
        if let Some(data) = res {
            if data.len() != 0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn approve_inner(
        contract_address: &Address, from: &Address, to: &Address, amount: U256,
    ) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(to_neo_bytes(amount));
        let mut sink_param = Sink::new(16);
        sink_param.write(VERSION);
        sink_param.write("approve");
        sink_param.write(sink.into());
        let res = runtime::call_contract(contract_address, sink_param.into().as_slice());
        if let Some(data) = res {
            if data.len() != 0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn transfer_from_inner(
        contract_address: &Address, sender: &Address, from: &Address, to: &Address, amount: U256,
    ) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_address(sender);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(to_neo_bytes(amount));
        let mut sink_param = Sink::new(16);
        sink_param.write(VERSION);
        sink_param.write("transferFrom");
        sink_param.write(sink.into());
        let res = runtime::call_contract(contract_address, sink_param.into().as_slice());
        if let Some(data) = res {
            if data.len() != 0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn allowance_inner(
        contract_address: &Address, from: &Address, to: &Address,
    ) -> U256 {
        let mut sink = Sink::new(0);
        sink.write_native_address(from);
        sink.write_native_address(to);
        let mut sink_param = Sink::new(0);
        sink_param.write(VERSION);
        sink_param.write("allowance");
        sink_param.write(sink.into());
        let res = runtime::call_contract(contract_address, sink_param.into().as_slice());
        if let Some(data) = res {
            if data.len() != 0 {
                return U256::from(data.as_slice());
            }
        }
        U256::zero()
    }
    pub(crate) fn balance_of_inner(contract_address: &Address, address: &Address) -> U256 {
        let mut sink = Sink::new(0);
        sink.write_native_address(address);
        let mut sink_param = Sink::new(0);
        sink_param.write(VERSION);
        sink_param.write("balanceOf");
        sink_param.write(sink.into());
        let res = runtime::call_contract(contract_address, sink_param.into().as_slice());
        if let Some(data) = res {
            if data.len() != 0 {
                return U256::from(data.as_slice());
            }
        }
        U256::zero()
    }
}
