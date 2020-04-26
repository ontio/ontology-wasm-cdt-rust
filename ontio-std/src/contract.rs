use crate::prelude::*;

pub mod neo {
    use crate::prelude::*;
    pub fn call_contract<T: crate::abi::VmValueEncoder>(
        contract_address: &Address, param: T,
    ) -> Option<Vec<u8>> {
        let mut builder = crate::abi::VmValueBuilder::new();
        param.serialize(&mut builder);
        crate::runtime::call_contract(contract_address, &builder.bytes())
    }
}

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
    ///   let input= input();
    ///   let mut source = Source::new(&input);
    ///   let (from, to, amount) = source.read().unwrap();
    ///   ont::transfer(from,to, amount);
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
    ///   let input = input();
    ///   let mut source = Source::new(&input);
    ///   let (from,to,amount) = source.read().unwrap();
    ///   ont::approve(from, to, amount);
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
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let addr = source.read().unwrap();
    ///     ont::balance_of(addr);
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
    ///   let input= input();
    ///   let mut source = Source::new(&input);
    ///   let (from, to) = source.read().unwrap();
    ///   ont::allowance(from,to);
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
    ///   let input= input();
    ///   let mut source = Source::new(&input);
    ///   let (spender, from, to, amount) = source.read().unwrap();
    ///   ont::transfer_from(spender, from, to, amount);
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
    ///   let input = input();
    ///   let mut source = Source::new(&input);
    ///   let (from, to, amount) = source.read().unwrap();
    ///   ong::transfer(from,to, amount);
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
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let addr = source.read().unwrap();
    ///     ong::balance_of(addr);
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
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let (from,to,amount) = source.read().unwrap();
    ///     ong::approve(from, to, amount);
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
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let (from, to) = source.read().unwrap();
    ///     ong::allowance(from,to);
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
    ///     let input = input();
    ///     let mut source = Source::new(&input);
    ///     let (spender, from, to, amount) = source.read().unwrap();
    ///     ong::transfer_from(spender, from, to, amount);
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

pub mod contract_mock {
    use crate::abi::{Decoder, Error, Source, TYPE_ADDRESS, TYPE_INT, TYPE_LIST, TYPE_STRING};
    use crate::prelude::*;
    use crate::types::{u128_from_neo_bytes, Address, U128};

    pub enum Command<'a> {
        Transfer { from: &'a Address, to: &'a Address, value: U128 },
        BalanceOf { addr: &'a Address },
    }

    impl<'a> Decoder<'a> for Command<'a> {
        fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
            let version = source.read_byte()?; //version
            assert_eq!(version, 0);
            let method = source.read().unwrap();
            match method {
                "transfer" => {
                    let param_length = source.read_varuint()?; //param length
                    assert_ne!(param_length, 0);
                    let transfer_len = source.read_native_varuint().ok().unwrap(); //transfer length
                    assert_eq!(transfer_len, 1);
                    let from = source.read_native_address().ok().unwrap();
                    let to = source.read_native_address().ok().unwrap();
                    let amt: Vec<u8> = source.read().ok().unwrap();
                    let value = u128_from_neo_bytes(amt.as_slice());
                    Ok(Command::Transfer { from, to, value })
                }
                "balance_of" => {
                    let addr = source.read_native_address().ok().unwrap();
                    Ok(Command::BalanceOf { addr })
                }
                _ => panic!(""),
            }
        }
    }

    pub enum NeoCommand<'a> {
        Transfer { from: &'a Address, to: &'a Address, value: U128 },
        BalanceOf { addr: &'a Address },
    }

    impl<'a> Decoder<'a> for NeoCommand<'a> {
        fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
            let version = source.read_byte()?; //version
            assert_eq!(version, 0);
            let list_type = source.read_byte()?; //list type
            assert_eq!(list_type, TYPE_LIST);
            let param_length: u32 = source.read()?;
            assert_ne!(param_length, 0);
            let string_type = source.read_byte()?;
            assert_eq!(string_type, TYPE_STRING);
            let str_l: u32 = source.read()?;
            assert_ne!(str_l, 0);
            let method: &[u8] = source.next_bytes(str_l as usize)?;
            match method {
                b"transfer" => {
                    let list_type = source.read_byte()?; //list type
                    assert_eq!(list_type, TYPE_LIST);
                    let transfer_len: u32 = source.read()?; //param length
                    assert_eq!(transfer_len, 1);
                    let address_type = source.read_byte()?;
                    assert_eq!(address_type, TYPE_ADDRESS);
                    let from: &Address = source.read()?;
                    let address_type = source.read_byte()?;
                    assert_eq!(address_type, TYPE_ADDRESS);
                    let to: &Address = source.read()?;
                    let int_type = source.read_byte()?;
                    assert_eq!(int_type, TYPE_INT);
                    let value: U128 = source.read()?;
                    Ok(NeoCommand::Transfer { from, to, value })
                }
                b"balance_of" => {
                    let list_type = source.read_byte()?; //list type
                    assert_eq!(list_type, TYPE_LIST);
                    let param_len: u32 = source.read()?; //param length
                    assert_eq!(param_len, 1);
                    let address_type = source.read_byte()?; // param type
                    assert_eq!(address_type, TYPE_ADDRESS);
                    let addr: &Address = source.read()?;
                    Ok(NeoCommand::BalanceOf { addr })
                }
                _ => panic!(""),
            }
        }
    }
}
