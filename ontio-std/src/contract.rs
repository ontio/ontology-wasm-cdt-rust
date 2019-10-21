use crate::prelude::*;

///Used when a transaction contains transfers between multiple addresses.
pub struct TransferParam {
    ///
    pub from: Address,
    pub to: Address,
    pub amount: U128,
}

///This module provides the operation API related to ont assets, such as balanceof, transfer, etc.
pub mod ont {
    use crate::macros::base58;
    use crate::prelude::*;

    const ONT_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");

    ///Transfer method of ont assets, Transfer ont assets from the from address to the to address
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::ont;
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input= input();
    ///     let mut source = Source::new(&input);
    ///     let (from, to, amount) = source.read().unwrap();
    ///     ont::transfer(from,to, amount);
    /// # }
    /// ```
    pub fn transfer(from: &Address, to: &Address, val: U128) -> bool {
        let state = [TransferParam { from: *from, to: *to, amount: val }];
        super::util::transfer_inner(&ONT_CONTRACT_ADDRESS, state.as_ref())
    }
    ///transfer_multi method of ont assets,Multiple transfers in one transaction
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ont,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # use ontio_std::types::{Address, U128};
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let trs: Vec<(Address,Address,U128)> = source.read().unwrap();
    ///     let mut ts = Vec::<TransferParam>::new();
    ///     for tr in trs.iter() {
    ///         let trans = TransferParam{
    ///            from:tr.0,
    ///            to:tr.1,
    ///            amount:tr.2,
    ///         };
    ///         ts.push(trans)
    ///     }
    ///     ont::transfer_multi(ts.as_slice());
    /// # }
    /// ```
    pub fn transfer_multi(transfer: &[TransferParam]) -> bool {
        super::util::transfer_inner(&ONT_CONTRACT_ADDRESS, transfer)
    }
    ///from-address can allow to-address to transfer a certain amount of assets from  from-address.
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ont,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let (from,to,amount) = source.read().unwrap();
    ///     ont::approve(from, to, amount);
    /// # }
    /// ```
    pub fn approve(from: &Address, to: &Address, amount: U128) -> bool {
        super::util::approve_inner(&ONT_CONTRACT_ADDRESS, from, to, amount)
    }
    ///Query the balance of ont assets
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ont,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let addr = source.read().unwrap();
    ///     ont::balance_of(addr);
    /// # }
    /// ```
    pub fn balance_of(address: &Address) -> U128 {
        super::util::balance_of_inner(&ONT_CONTRACT_ADDRESS, &address)
    }
    ///This method is used in conjunction with the approve method to query the number of approve
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ont,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input= input();
    ///     let mut source = Source::new(&input);
    ///     let (from, to) = source.read().unwrap();
    ///     ont::allowance(from,to);
    /// # }
    /// ```
    pub fn allowance(from: &Address, to: &Address) -> U128 {
        super::util::allowance_inner(&ONT_CONTRACT_ADDRESS, from, to)
    }
    ///Spender transfers a certain amount of ont from from-address to to-address
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ont,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input= input();
    ///     let mut source = Source::new(&input);
    ///     let (spender, from, to, amount) = source.read().unwrap();
    ///     ont::transfer_from(spender, from, to, amount);
    /// # }
    /// ```
    pub fn transfer_from(sender: &Address, from: &Address, to: &Address, amount: U128) -> bool {
        super::util::transfer_from_inner(&ONT_CONTRACT_ADDRESS, sender, from, to, amount)
    }
}

///This module provides the operation API related to ong assets, such as balanceof, transfer, etc.
pub mod ong {
    use crate::prelude::*;

    use crate::macros::base58;
    use crate::types::{Address, U128};

    const ONG_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhfRZMHJ");

    ///Transfer method of ong assets, Transfer ont assets from the from address to the to address
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::ong;
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let (from, to, amount) = source.read().unwrap();
    ///     ong::transfer(from,to, amount);
    /// # }
    /// ```
    pub fn transfer(from: &Address, to: &Address, val: U128) -> bool {
        let state = [TransferParam { from: *from, to: *to, amount: val }];
        super::util::transfer_inner(&ONG_CONTRACT_ADDRESS, state.as_ref())
    }
    ///transfer_multi method of ong assets,Multiple transfers in one transaction
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ong,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # use ontio_std::types::{Address,U128};
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let trs: Vec<(Address,Address,U128)> = source.read().unwrap();
    ///     let mut transfers = Vec::<TransferParam>::new();
    ///     for tr in trs.iter() {
    ///         transfers.push(TransferParam{
    ///             from:tr.0,
    ///             to:tr.1,
    ///             amount:tr.2,
    ///         })
    ///     }
    ///     ong::transfer_multi(transfers.as_slice());
    /// # }
    /// ```
    pub fn transfer_multi(transfer: &[super::TransferParam]) -> bool {
        super::util::transfer_inner(&ONG_CONTRACT_ADDRESS, transfer)
    }
    ///Query the balance of ong assets
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ong,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let addr = source.read().unwrap();
    ///     ong::balance_of(addr);
    /// # }
    /// ```
    pub fn balance_of(address: &Address) -> U128 {
        super::util::balance_of_inner(&ONG_CONTRACT_ADDRESS, &address)
    }
    ///from-address can allow to-address to transfer a certain amount of assets from  from-address.
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ong,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let (from,to,amount) = source.read().unwrap();
    ///     ong::approve(from, to, amount);
    /// # }
    /// ```
    pub fn approve(from: &Address, to: &Address, amount: U128) -> bool {
        super::util::approve_inner(&ONG_CONTRACT_ADDRESS, from, to, amount)
    }
    ///This method is used in conjunction with the approve method to query the number of approve
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ong};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let (from, to) = source.read().unwrap();
    ///     ong::allowance(from,to);
    /// # }
    /// ```
    pub fn allowance(from: &Address, to: &Address) -> U128 {
        super::util::allowance_inner(&ONG_CONTRACT_ADDRESS, from, to)
    }
    ///Spender transfers a certain amount of ong from from-address to to-address
    /// # Example
    /// ```no_run
    /// # use ontio_std::contract::{ong,TransferParam};
    /// # use ontio_std::abi::{Sink, Source};
    /// # use ontio_std::runtime::input;
    /// # fn main() {
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let (spender, from, to, amount) = source.read().unwrap();
    ///     ong::transfer_from(spender, from, to, amount);
    /// # }
    /// ```
    pub fn transfer_from(sender: &Address, from: &Address, to: &Address, amount: U128) -> bool {
        super::util::transfer_from_inner(&ONG_CONTRACT_ADDRESS, sender, from, to, amount)
    }
}

pub(crate) mod util {
    use super::super::abi::Sink;
    use super::super::runtime;
    use super::super::types::{u128_from_neo_bytes, u128_to_neo_bytes, Address, U128};

    const VERSION: u8 = 0;
    pub(crate) fn transfer_inner(
        contract_address: &Address, transfer: &[super::TransferParam],
    ) -> bool {
        let mut sink = Sink::new(64);
        sink.write_native_varuint(transfer.len() as u64);

        for state in transfer.iter() {
            sink.write_native_address(&state.from);
            sink.write_native_address(&state.to);
            sink.write(u128_to_neo_bytes(state.amount));
        }
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("transfer");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if !data.is_empty() {
                return true;
            }
        }
        false
    }

    pub(crate) fn approve_inner(
        contract_address: &Address, from: &Address, to: &Address, amount: U128,
    ) -> bool {
        let mut sink = Sink::new(64);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(u128_to_neo_bytes(amount));
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("approve");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if !data.is_empty() {
                return true;
            }
        }
        false
    }

    pub(crate) fn transfer_from_inner(
        contract_address: &Address, sender: &Address, from: &Address, to: &Address, amount: U128,
    ) -> bool {
        let mut sink = Sink::new(64);
        sink.write_native_address(sender);
        sink.write_native_address(from);
        sink.write_native_address(to);
        sink.write(u128_to_neo_bytes(amount));
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("transferFrom");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if !data.is_empty() {
                return true;
            }
        }
        false
    }

    pub(crate) fn allowance_inner(
        contract_address: &Address, from: &Address, to: &Address,
    ) -> U128 {
        let mut sink = Sink::new(64);
        sink.write_native_address(from);
        sink.write_native_address(to);
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("allowance");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if !data.is_empty() {
                return u128_from_neo_bytes(&data);
            }
        }
        0
    }

    pub(crate) fn balance_of_inner(contract_address: &Address, address: &Address) -> U128 {
        let mut sink = Sink::new(64);
        sink.write_native_address(address);
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("balanceOf");
        sink_param.write(sink.bytes());
        let res = runtime::call_contract(contract_address, sink_param.bytes());
        if let Some(data) = res {
            if !data.is_empty() {
                return u128_from_neo_bytes(&data);
            }
        }
        0
    }
}
