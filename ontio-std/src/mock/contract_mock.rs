use super::super::prelude::*;
use crate::abi::event_builder::TYPE_LIST;
use crate::abi::event_builder::{TYPE_ADDRESS, TYPE_INT, TYPE_STRING};
use crate::abi::{Decoder, Error, Source};
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
