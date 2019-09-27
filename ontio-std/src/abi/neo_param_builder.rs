use super::Error;
use super::{NeoParamDecoder, NeoParamEncoder, Sink, Source};
use crate::prelude::*;
use crate::runtime;

const DEFAULT_CAP: usize = 128;
const TYPE_BYTEARRAY: u8 = 0x00;
const TYPE_STRING: u8 = 0x01;
const TYPE_ADDRESS: u8 = 0x02;
const TYPE_BOOL: u8 = 0x03;
const TYPE_INT: u8 = 0x04;
const TYPE_H256: u8 = 0x05;

const TYPE_LIST: u8 = 0x10;

pub struct NeoParamBuilder {
    sink: Sink,
}

impl NeoParamBuilder {
    pub fn new() -> Self {
        let mut eb = NeoParamBuilder { sink: Sink::new(DEFAULT_CAP) };
        eb.sink.write_byte(0u8);
        eb
    }

    pub fn write<T: NeoParamEncoder>(&mut self, val: T) {
        val.serialize(self)
    }

    pub fn string(&mut self, method: &str) {
        self.sink.write_byte(TYPE_STRING);
        self.sink.write_u32(method.len() as u32);
        self.sink.write_bytes(method.as_bytes());
    }

    pub fn bytearray(&mut self, bytes: &[u8]) {
        self.sink.write_byte(TYPE_BYTEARRAY);
        self.sink.write_u32(bytes.len() as u32);
        self.sink.write_bytes(bytes);
    }

    pub fn address(&mut self, address: &Address) {
        self.sink.write_byte(TYPE_ADDRESS);
        self.sink.write_bytes(address.as_bytes());
    }

    pub fn number(&mut self, amount: U128) {
        self.sink.write_byte(TYPE_INT);
        self.sink.write_u128(amount);
    }

    pub fn bool(&mut self, b: bool) {
        self.sink.write_byte(TYPE_BOOL);
        self.sink.write_bool(b);
    }

    pub fn h256(&mut self, hash: H256) {
        self.sink.write_byte(TYPE_H256);
        self.sink.write_bytes(hash.as_ref());
    }

    pub fn bytes(&self) -> &[u8] {
        self.sink.bytes()
    }
}

pub struct NeoParamParser<'a> {
    source: Source<'a>,
}

impl<'a> NeoParamParser<'a> {
    pub fn new(bs: &'a [u8]) -> Self {
        Self { source: Source::new(bs) }
    }
    pub fn read<T: NeoParamDecoder<'a>>(&mut self) -> Result<T, Error> {
        T::deserialize(self)
    }

    pub fn string(&mut self) -> Result<&'a str, Error> {
        let ty = self.source.read_byte()?;
        if ty != TYPE_STRING {
            return Err(Error::TypeInconsistency);
        }
        let l = self.source.read_u32()?;
        let buf = self.source.next_bytes(l as usize)?;
        str::from_utf8(buf).map_err(|_| Error::InvalidUtf8)
    }

    pub fn bytearray(&mut self) -> Result<&'a [u8], Error> {
        let ty = self.source.read_byte()?;
        if ty != TYPE_BYTEARRAY {
            return Err(Error::TypeInconsistency);
        }
        let l = self.source.read_u32()?;
        self.source.next_bytes(l as usize)
    }

    pub fn address(&mut self) -> Result<&'a Address, Error> {
        let ty = self.source.read_byte()?;
        if ty != TYPE_ADDRESS {
            return Err(Error::TypeInconsistency);
        }
        self.source.read_address()
    }

    pub fn number(&mut self) -> Result<u128, Error> {
        let ty = self.source.read_byte()?;
        if ty != TYPE_INT {
            return Err(Error::TypeInconsistency);
        }
        self.source.read_u128()
    }

    pub fn bool(&mut self) -> Result<bool, Error> {
        let ty = self.source.read_byte()?;
        if ty != TYPE_BOOL {
            return Err(Error::TypeInconsistency);
        }
        self.source.read_bool()
    }

    pub fn h256(&mut self) -> Result<&'a H256, Error> {
        let ty = self.source.read_byte()?;
        if ty != TYPE_H256 {
            return Err(Error::TypeInconsistency);
        }
        self.source.read_h256()
    }
}
