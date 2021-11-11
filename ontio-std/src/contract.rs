use crate::prelude::*;

pub mod governance {
    use crate::abi::{Encoder, Sink, Source};
    use crate::macros::base58;
    use crate::prelude::*;
    use crate::runtime::call_contract;
    use crate::types::u128_to_neo_bytes;

    const VERSION: u8 = 0;
    pub const GOV_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbviEH9ugK");

    pub struct AuthorizeForPeerParam<'a> {
        addr: &'a Address,
        peer_pubkey_list: &'a [&'a str],
        pos_list: &'a [u32],
    }

    impl<'a> AuthorizeForPeerParam<'a> {
        pub fn new(addr: &'a Address, pos_list: &'a [u32], peer_pub_key: &'a [&str]) -> Self {
            AuthorizeForPeerParam { addr, peer_pubkey_list: peer_pub_key, pos_list }
        }
    }

    impl<'a> Encoder for AuthorizeForPeerParam<'a> {
        fn encode(&self, sink: &mut Sink) {
            sink.write_var_bytes(&self.addr.as_bytes());
            sink.write_native_varuint(self.peer_pubkey_list.len() as u64);
            for &pk in self.peer_pubkey_list.iter() {
                sink.write(pk);
            }
            sink.write_native_varuint(self.pos_list.len() as u64);
            for pos in self.pos_list.iter() {
                sink.write(u128_to_neo_bytes(U128::new(*pos as u128)));
            }
        }
    }

    pub fn authorize_for_peer(addr: &Address, amt: U128, peer_pub_key: &str) -> bool {
        let mut sink = Sink::new(64);
        let pos_list = vec![amt.raw() as u32];
        let peer_pub_key = &[peer_pub_key];
        let a = AuthorizeForPeerParam::new(addr, pos_list.as_slice(), peer_pub_key);
        sink.write(a);

        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("authorizeForPeer");
        sink_param.write(sink.bytes());

        let output = call_contract(&GOV_CONTRACT_ADDRESS, sink_param.bytes());
        let mut source = Source::new(output.as_slice());
        source.read_bool().unwrap_or_default()
    }

    //Authorize for a node by depositing ONT in this governance contract, used by contracts
    pub fn authorize_for_peer_transfer_from(addr: &Address, amt: U128, peer_pub_key: &str) -> bool {
        let mut sink = Sink::new(64);
        let pos_list = &[amt.raw() as u32];
        let peer_pub_key = &[peer_pub_key];
        let a = AuthorizeForPeerParam::new(addr, pos_list, peer_pub_key);
        sink.write(a);
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("authorizeForPeerTransferFrom");
        sink_param.write(sink.bytes());

        let output = call_contract(&GOV_CONTRACT_ADDRESS, sink_param.bytes());
        let mut source = Source::new(output.as_slice());
        source.read_bool().unwrap_or_default()
    }

    //UnAuthorize for a node by redeeming ONT from this governance contract
    pub fn un_authorize_for_peer(addr: &Address, amt: U128, peer_pub_key: &str) -> bool {
        let mut sink = Sink::new(64);
        let pos_list = vec![amt.raw() as u32];
        let peer_pub_key = &[peer_pub_key];
        let a = AuthorizeForPeerParam::new(addr, pos_list.as_slice(), peer_pub_key);
        sink.write(a);

        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("unAuthorizeForPeer");
        sink_param.write(sink.bytes());

        let output = call_contract(&GOV_CONTRACT_ADDRESS, sink_param.bytes());
        let mut source = Source::new(output.as_slice());
        source.read_bool().unwrap_or_default()
    }

    struct WithdrawParam<'a> {
        addr: &'a Address,
        peer_pubkey_list: &'a [&'a str],
        withdraw_list: &'a [u32],
    }
    impl<'a> Encoder for WithdrawParam<'a> {
        fn encode(&self, sink: &mut Sink) {
            sink.write_var_bytes(self.addr.as_bytes());
            sink.write_var_bytes(
                u128_to_neo_bytes(U128::new(self.peer_pubkey_list.len() as u128)).as_slice(),
            );
            self.peer_pubkey_list.iter().for_each(|peer_pubkey| sink.write(peer_pubkey));
            sink.write_var_bytes(
                u128_to_neo_bytes(U128::new(self.withdraw_list.len() as u128)).as_slice(),
            );
            self.withdraw_list.iter().for_each(|withdraw| {
                sink.write_var_bytes(u128_to_neo_bytes(U128::new(*withdraw as u128)).as_slice())
            });
        }
    }

    //Withdraw unfreezed ONT deposited in this governance contract.
    pub fn withdraw(addr: &Address, amt: U128, peer_pub_key: &str) -> bool {
        let mut sink = Sink::new(80);
        sink.write(WithdrawParam {
            addr,
            peer_pubkey_list: &[peer_pub_key],
            withdraw_list: &[amt.raw() as u32],
        });
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("withdraw");
        sink_param.write(sink.bytes());

        let output = call_contract(&GOV_CONTRACT_ADDRESS, sink_param.bytes());
        let mut source = Source::new(output.as_slice());
        source.read_bool().unwrap_or_default()
    }

    struct WithdrawOngParam<'a> {
        addr: &'a Address,
    }
    impl<'a> Encoder for WithdrawOngParam<'a> {
        fn encode(&self, sink: &mut Sink) {
            sink.write_var_bytes(self.addr.as_bytes())
        }
    }

    //Withdraw unbounded ONG according to deposit ONT in this governance contract
    pub fn withdraw_ong(addr: &Address) -> bool {
        let mut sink = Sink::new(64);
        sink.write(WithdrawOngParam { addr });
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("withdrawOng");
        sink_param.write(sink.bytes());

        let output = call_contract(&GOV_CONTRACT_ADDRESS, sink_param.bytes());
        let mut source = Source::new(output.as_slice());
        source.read_bool().unwrap_or_default()
    }

    #[test]
    pub fn test_struct() {
        let data = [
            0x1b, 0x25, 0xfb, 0x79, 0xe6, 0x1d, 0x58, 0x60, 0x97, 0xef, 0xd9, 0xee, 0x89, 0xa5,
            0xab, 0xbf, 0x22, 0x27, 0xa2, 0xba,
        ];
        let ap = AuthorizeForPeerParam {
            addr: &Address::new(data),
            peer_pubkey_list: &[&"test".to_string()],
            pos_list: &[128u32],
        };
        let mut sink = Sink::new(64);
        sink.write(ap);
        println!("AuthorizeForPeerParam:{}", hexutil::to_hex(sink.bytes()));

        let wp = WithdrawParam {
            addr: &Address::repeat_byte(2),
            peer_pubkey_list: &["test"],
            withdraw_list: &[128u32],
        };
        let mut sink = Sink::new(64);
        sink.write(wp);
        println!("WithdrawParam:{}", hexutil::to_hex(sink.bytes()));

        let wp = WithdrawOngParam { addr: &Address::repeat_byte(2) };
        let mut sink = Sink::new(64);
        sink.write(wp);
        println!("WithdrawOngParam:{}", hexutil::to_hex(sink.bytes()));
    }
}

pub mod neo {
    use crate::prelude::*;

    pub fn call_contract<T: crate::abi::VmValueEncoder>(
        contract_address: &Address, param: T,
    ) -> Vec<u8> {
        let mut builder = crate::abi::VmValueBuilder::new();
        param.serialize(&mut builder);
        crate::runtime::call_contract(contract_address, &builder.bytes())
    }
}

pub mod eth {
    use crate::abi::Sink;
    use crate::macros::base58;
    use crate::prelude::*;
    use crate::runtime;
    use crate::types::Address;

    const EVM_INVOKE_NAME: &str = "evmInvoke";
    const VERSION: u8 = 0;
    const SYSTEM_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbwC9m2yJG");

    pub fn evm_invoke(caller: &Address, target: &Address, input: &[u8]) -> Vec<u8> {
        let mut sink = Sink::new(input.len() + 20 + 20 + 16);
        sink.write(caller.as_bytes());
        sink.write(target.as_bytes());
        sink.write(input);
        let mut sink_param = Sink::new(sink.bytes().len() + 32);
        sink_param.write(VERSION);
        sink_param.write(EVM_INVOKE_NAME);
        sink_param.write(sink.bytes());
        runtime::call_contract(&SYSTEM_CONTRACT_ADDRESS, sink_param.bytes())
    }
}

pub mod ontid {
    use super::super::types::u128_to_neo_bytes;
    use crate::abi::{Decoder, Encoder, Error, Sink, Source};
    use crate::macros::base58;
    use crate::prelude::*;
    use crate::runtime;

    const VERSION: u8 = 0;
    const ONTID_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6Ubvho7BUwN");

    pub struct DDOAttribute {
        key: Vec<u8>,
        value: Vec<u8>,
        value_type: Vec<u8>,
    }

    impl Encoder for DDOAttribute {
        fn encode(&self, sink: &mut Sink) {
            sink.write(self.key.as_slice());
            sink.write(self.value.as_slice());
            sink.write(self.value_type.as_slice());
        }
    }

    impl<'a> Decoder<'a> for DDOAttribute {
        fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
            let key: &[u8] = source.read().unwrap();
            let value: &[u8] = source.read().unwrap();
            let value_type: &[u8] = source.read().unwrap();
            Ok(DDOAttribute {
                key: key.to_vec(),
                value: value.to_vec(),
                value_type: value_type.to_vec(),
            })
        }
    }

    pub struct Signer {
        id: Vec<u8>,
        index: u32,
    }

    impl Encoder for Signer {
        fn encode(&self, sink: &mut Sink) {
            sink.write(self.id.as_slice());
            sink.write(self.index);
        }
    }

    impl<'a> Decoder<'a> for Signer {
        fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
            let id: &[u8] = source.read().unwrap();
            let index: u32 = source.read().unwrap();
            Ok(Signer { id: id.to_vec(), index })
        }
    }

    pub struct Group {
        pub members: Vec<Vec<u8>>,
        pub threshold: u32,
    }

    impl Encoder for Group {
        fn encode(&self, sink: &mut Sink) {
            sink.write(self.members.as_slice());
            sink.write(self.threshold);
        }
    }

    impl<'a> Decoder<'a> for Group {
        fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
            let members: Vec<Vec<u8>> = source.read().unwrap();
            let threshold: u32 = source.read().unwrap();
            Ok(Group { members, threshold })
        }
    }

    fn serialize_group(group: &Group) -> Vec<u8> {
        let mut sink = Sink::new(64);
        sink.write_var_bytes(u128_to_neo_bytes(U128::new(group.members.len() as u128)).as_slice());
        for mem in group.members.iter() {
            sink.write_var_bytes(mem);
        }
        sink.write_var_bytes(u128_to_neo_bytes(U128::new(group.threshold as u128)).as_slice());
        sink.bytes().to_vec()
    }

    fn serialize_signers(signers: &[Signer]) -> Vec<u8> {
        let mut sink = Sink::new(64);
        sink.write_var_bytes(u128_to_neo_bytes(U128::new(signers.len() as u128)).as_slice());
        for signer in signers.iter() {
            sink.write_var_bytes(signer.id.as_slice());
            sink.write_var_bytes(u128_to_neo_bytes(U128::new(signer.index as u128)).as_slice());
        }
        sink.bytes().to_vec()
    }

    pub fn reg_id_with_controller(ont_id: &[u8], group: &Group, signers: &[Signer]) -> bool {
        let mut sink = Sink::new(32);
        sink.write(ont_id);
        sink.write(serialize_group(group));
        sink.write(serialize_signers(signers));
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("regIDWithController");
        sink_param.write(sink.bytes());
        let output = runtime::call_contract(&ONTID_CONTRACT_ADDRESS, sink_param.bytes());

        !output.is_empty() && output[0] == 1u8
    }

    pub fn add_attributes_by_controller(
        ont_id: &[u8], attributes: &[DDOAttribute], signers: &[Signer],
    ) -> bool {
        let mut sink = Sink::new(32);
        sink.write(ont_id);
        sink.write_native_varuint(attributes.len() as u64);
        for attr in attributes.iter() {
            sink.write(attr.key.as_slice());
            sink.write(attr.value_type.as_slice());
            sink.write(attr.value.as_slice());
        }
        sink.write(serialize_signers(signers));
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("addAttributesByController");
        sink_param.write(sink.bytes());
        let output = runtime::call_contract(&ONTID_CONTRACT_ADDRESS, sink_param.bytes());

        !output.is_empty() && output[0] == 1u8
    }

    pub fn verify_signature(ont_id: &[u8], index: U128) -> bool {
        verify_sig_inner("verifySignature", ont_id, index)
    }

    fn verify_sig_inner(method: &str, ont_id: &[u8], index: U128) -> bool {
        let mut sink = Sink::new(32);
        sink.write(ont_id);
        sink.write(u128_to_neo_bytes(index));
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write(method);
        sink_param.write(sink.bytes());
        let output = runtime::call_contract(&ONTID_CONTRACT_ADDRESS, sink_param.bytes());

        !output.is_empty() && output[0] == 1u8
    }
}

pub mod wasm {
    use crate::abi::Sink;
    use crate::prelude::*;

    pub fn call_contract<T: crate::abi::Encoder>(contract_address: &Address, param: T) -> Vec<u8> {
        let mut sink = Sink::new(16);
        sink.write(param);
        crate::runtime::call_contract(contract_address, sink.bytes())
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
        super::util::balance_of_inner(&ONT_CONTRACT_ADDRESS, address)
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
        super::util::balance_of_inner(&ONG_CONTRACT_ADDRESS, address)
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
        let output = runtime::call_contract(contract_address, sink_param.bytes());

        !output.is_empty() && output[0] == 1u8
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
        let output = runtime::call_contract(contract_address, sink_param.bytes());

        !output.is_empty() && output[0] == 1u8
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
        let output = runtime::call_contract(contract_address, sink_param.bytes());

        !output.is_empty() && output[0] == 1u8
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
        let output = runtime::call_contract(contract_address, sink_param.bytes());
        if !output.is_empty() {
            return u128_from_neo_bytes(&output);
        }
        U128::new(0)
    }

    pub(crate) fn balance_of_inner(contract_address: &Address, address: &Address) -> U128 {
        let mut sink = Sink::new(64);
        sink.write_native_address(address);
        let mut sink_param = Sink::new(64);
        sink_param.write(VERSION);
        sink_param.write("balanceOf");
        sink_param.write(sink.bytes());
        let output = runtime::call_contract(contract_address, sink_param.bytes());
        if !output.is_empty() {
            return u128_from_neo_bytes(&output);
        }
        U128::new(0)
    }
}
