use super::event_builder::{
    TYPE_ADDRESS, TYPE_BOOL, TYPE_BYTEARRAY, TYPE_H256, TYPE_INT, TYPE_STRING,
};
use super::Error;
use super::Source;
use super::{VmValueBuilderCommon, VmValueDecoder, VmValueEncoder};
use crate::abi::event_builder::TYPE_LIST;
use crate::prelude::*;
use fixed_hash::static_assertions::_core::ops::{Deref, DerefMut};

pub struct VmValueBuilder {
    pub(crate) common: VmValueBuilderCommon,
}

impl Default for VmValueBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct NestedVmValueBuilder<'a> {
    origin: &'a mut VmValueBuilder,
    current: VmValueBuilderCommon,
}

impl<'a> NestedVmValueBuilder<'_> {
    pub fn finish(self) {
        let mut buf = self.current.sink.into();
        buf[1..5].copy_from_slice(&self.current.num_entry.to_le_bytes());
        self.origin.common.sink.write_bytes(&buf);
    }
}

impl<'a> Deref for NestedVmValueBuilder<'a> {
    type Target = VmValueBuilderCommon;

    fn deref(&self) -> &Self::Target {
        &self.current
    }
}

impl<'a> DerefMut for NestedVmValueBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.current
    }
}

impl VmValueBuilder {
    pub fn new() -> Self {
        let common = VmValueBuilderCommon::new();
        let mut builder = VmValueBuilder { common };
        builder.common.sink.write_byte(0u8); // verison
        builder.common.sink.write_byte(TYPE_LIST); // list type
        builder.common.sink.write_u32(builder.common.num_entry); // occupy length
        builder
    }

    pub fn write<T: VmValueEncoder>(&mut self, val: T) {
        val.serialize(self)
    }

    pub fn string(&mut self, method: &str) {
        self.common.string(method);
    }

    pub fn bytearray(&mut self, bytes: &[u8]) {
        self.common.bytearray(bytes);
    }

    pub fn address(&mut self, address: &Address) {
        self.common.address(address);
    }

    pub fn number(&mut self, amount: U128) {
        self.common.number(amount);
    }

    pub fn list(&mut self) -> NestedVmValueBuilder {
        let mut nested = VmValueBuilderCommon::new();
        nested.sink.write_byte(TYPE_LIST); // list type
        nested.sink.write_u32(0); // occupy length

        NestedVmValueBuilder { origin: self, current: nested }
    }

    pub fn bool(&mut self, b: bool) {
        self.common.bool(b);
    }

    pub fn h256(&mut self, hash: &H256) {
        self.common.h256(hash);
    }

    pub fn bytes(self) -> Vec<u8> {
        let num_entry = self.common.num_entry;
        let mut buf = self.common.sink.into();
        buf[2..6].copy_from_slice(&num_entry.to_le_bytes());
        buf
    }
}

pub struct VmValueParser<'a> {
    pub source: Source<'a>,
}

impl<'a> VmValueParser<'a> {
    pub fn new(bs: &'a [u8]) -> Self {
        let mut source = Source::new(bs);
        let _version = source.read_byte(); //version
        Self { source }
    }

    pub fn read<T: VmValueDecoder<'a>>(&mut self) -> Result<T, Error> {
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
        if ty != TYPE_BYTEARRAY || ty == TYPE_STRING {
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
        match ty {
            TYPE_BOOL => self.source.read_bool(),
            TYPE_INT => Ok(!self.source.read_u128()?.is_zero()),
            _ => Err(Error::TypeInconsistency)
        }
    }

    pub fn h256(&mut self) -> Result<&'a H256, Error> {
        let ty = self.source.read_byte()?;
        if ty != TYPE_H256 {
            return Err(Error::TypeInconsistency);
        }
        self.source.read_h256()
    }
}
