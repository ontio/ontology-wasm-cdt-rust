use super::Error;
use crate::abi::{VmValueBuilder, VmValueParser};
use crate::prelude::*;

pub trait VmValueEncoder {
    fn serialize(&self, sink: &mut VmValueBuilder);
}

impl VmValueEncoder for &str {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.string(self);
    }
}

impl VmValueEncoder for &[u8] {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.bytearray(self);
    }
}

impl VmValueEncoder for bool {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.bool(*self);
    }
}

impl VmValueEncoder for H256 {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.h256(&self);
    }
}

impl VmValueEncoder for U128 {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.number(self.clone());
    }
}

impl VmValueEncoder for Address {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        builder.address(&self);
    }
}

impl<T: VmValueEncoder> VmValueEncoder for &T {
    fn serialize(&self, builder: &mut VmValueBuilder) {
        (*self).serialize(builder)
    }
}

pub trait VmValueDecoder<'a>: Sized {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error>;
}

impl<'a> VmValueDecoder<'a> for &'a str {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.string()
    }
}

impl<'a> VmValueDecoder<'a> for &'a [u8] {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.bytearray()
    }
}

impl<'a> VmValueDecoder<'a> for bool {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.bool()
    }
}

impl<'a> VmValueDecoder<'a> for &'a H256 {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.h256()
    }
}

impl<'a> VmValueDecoder<'a> for U128 {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.number()
    }
}

impl<'a> VmValueDecoder<'a> for &'a Address {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
        parser.address()
    }
}

impl<'a, T: VmValueDecoder<'a>> VmValueDecoder<'a> for &'a T {
    fn deserialize(parser: &mut VmValueParser<'a>) -> Result<&'a T, Error> {
        parser.read()
    }
}
