
pub mod ont {
    pub struct State  {
        pub from:   Address,
        pub to:     Address,
        pub amount: U256,
    }
    use super::super::types::{Address,U256};
    use super::super::base58;
    const ONT_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");
    pub fn transfer(version:u8, transfer: &[State]) -> bool {
        super::util::transfer_inner(&ONT_CONTRACT_ADDRESS,"transfer", version, transfer)
    }
    pub fn approve(version:u8, from:&Address, to:&Address, amount:U256) -> bool {
        super::util::approve_inner(&ONT_CONTRACT_ADDRESS,"approve", version, from, to, amount)
    }
    pub fn balance_of(version:u8, address: &Address) -> U256 {
        super::util::balance_of(&ONT_CONTRACT_ADDRESS,version, &address)
    }
    pub fn allowance(version:u8, from:&Address, to: &Address) -> U256 {
        super::util::allowance(&ONT_CONTRACT_ADDRESS, version, from, to)
    }
    pub fn transfer_from(version:u8, sender:&Address, from: &Address, to: &Address, amount: U256) -> bool {
        super::util::transfer_from(&ONT_CONTRACT_ADDRESS,version, sender, from, to, amount)
    }
}

pub mod ong {
    use super::super::types::{Address,U256};
    use super::super::base58;
    const ONG_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhfRZMHJ");
    pub fn transfer(version:u8, transfer: &[super::ont::State]) -> bool {
        super::util::transfer_inner(&ONG_CONTRACT_ADDRESS,"transfer", version, transfer)
    }
    pub fn balance_of(version:u8, address: &Address) -> U256 {
        super::util::balance_of(&ONG_CONTRACT_ADDRESS,version, &address)
    }
    pub fn approve(version:u8, from: &Address, to: &Address, amount: U256) -> bool {
        super::util::approve_inner(&ONG_CONTRACT_ADDRESS,"approve", version, from, to, amount)
    }
    pub fn allowance(version:u8, from:&Address, to: &Address) -> U256 {
        super::util::allowance(&ONG_CONTRACT_ADDRESS, version, from, to)
    }
    pub fn transfer_from(version:u8, sender:&Address, from: &Address, to: &Address, amount: U256) -> bool {
        super::util::transfer_from(&ONG_CONTRACT_ADDRESS,version, sender, from, to, amount)
    }
}

pub(crate) mod util {
    use super::super::types::{Address,U256,to_neo_bytes};
    use super::super::abi::Sink;
    use super::super::runtime;
    pub(crate) fn transfer_inner(contract_address: &Address, function_name:&str, version:u8, trasnfer: &[super::ont::State]) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_varuint(trasnfer.len() as u64);

        for state in trasnfer.iter() {
            sink.write_native_address(&state.from);
            sink.write_native_address(&state.to);
            sink.write(to_neo_bytes(state.amount));
        }
        let da = sink.into();
        let mut sink_param = Sink::new(16);
        sink_param.write(version);
        sink_param.write(function_name);
        sink_param.write(da);
        let res = runtime::call_contract(contract_address,sink_param.into().as_slice());
        if res.is_some() {
            let data = res.unwrap();
            if data.len() !=0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn approve_inner(contract_address: &Address, function_name:&str, version:u8, from:&Address, to:&Address, amount:U256) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(to_neo_bytes(amount));
        let param = sink.into();
        let mut sink_param = Sink::new(16);
        sink_param.write(version);
        sink_param.write(function_name);
        sink_param.write(param);
        let res = runtime::call_contract(contract_address,sink_param.into().as_slice());
        if res.is_some() {
            let data = res.unwrap();
            if data.len() !=0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn transfer_from(contract_address: &Address, version:u8, sender:&Address, from: &Address, to: &Address, amount: U256) -> bool {
        let mut sink = Sink::new(16);
        sink.write_native_address(sender);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(to_neo_bytes(amount));
        let mut sink_param = Sink::new(16);
        sink_param.write(version);
        sink_param.write("transferFrom");
        sink_param.write(sink.into());
        let res = runtime::call_contract(contract_address,sink_param.into().as_slice());
        if res.is_some() {
            let data = res.unwrap();
            if data.len() !=0 {
                return true;
            }
        }
        false
    }

    pub(crate) fn allowance(contract_address: &Address, version:u8, from:&Address,to:&Address) -> U256 {
        let mut sink = Sink::new(0);
        sink.write_native_address(from);
        sink.write_native_address(to);
        let mut sink_param = Sink::new(0);
        sink_param.write(version);
        sink_param.write("allowance");
        sink_param.write(sink.into());
        let res = runtime::call_contract(contract_address,sink_param.into().as_slice());
        if res.is_some() {
            let data = res.unwrap();
            return U256::from(data.as_slice());
        }
        U256::zero()
    }
    pub(crate) fn balance_of(contract_address: &Address, version:u8, address: &Address) -> U256 {
        let mut sink = Sink::new(0);
        sink.write_native_address(address);
        let mut sink_param = Sink::new(0);
        sink_param.write(version);
        sink_param.write("balanceOf");
        sink_param.write(sink.into());
        let res = runtime::call_contract(contract_address,sink_param.into().as_slice());
        if res.is_some() {
            let data = res.unwrap();
            return U256::from(data.as_slice());
        }
        U256::zero()
    }
}
