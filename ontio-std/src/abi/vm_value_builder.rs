use super::Error;
use super::{
    Source, VmValueBuilderCommon, VmValueDecoder, VmValueEncoder, TYPE_ADDRESS, TYPE_BOOL,
    TYPE_BYTEARRAY, TYPE_H256, TYPE_INT, TYPE_STRING,
};
use crate::prelude::*;

pub struct VmValueBuilder {
    pub(crate) common: VmValueBuilderCommon,
}

impl Default for VmValueBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl VmValueBuilder {
    pub fn new() -> Self {
        let common = VmValueBuilderCommon::new();
        let mut builder = VmValueBuilder { common };
        builder.common.sink.write_byte(0u8);
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

    pub fn bool(&mut self, b: bool) {
        self.common.bool(b);
    }

    pub fn h256(&mut self, hash: H256) {
        self.common.h256(hash);
    }

    pub fn bytes(self) -> Vec<u8> {
        self.common.sink.into()
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
        if ty == TYPE_BOOL {
            return self.source.read_bool();
        } else if ty == TYPE_INT {
            let res = self.source.read_u128()?;
            if res != 0 {
                return Ok(true);
            } else {
                return Ok(false);
            }
        }
        Err(Error::TypeInconsistency)
    }

    pub fn h256(&mut self) -> Result<&'a H256, Error> {
        let ty = self.source.read_byte()?;
        if ty != TYPE_H256 {
            return Err(Error::TypeInconsistency);
        }
        self.source.read_h256()
    }
}
