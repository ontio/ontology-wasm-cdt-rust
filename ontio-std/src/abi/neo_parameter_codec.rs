use super::Error;
use crate::abi::{NeoParamBuilder, NeoParamParser};
use crate::prelude::*;

pub trait NeoParamEncoder {
    fn serialize(&self, sink: &mut NeoParamBuilder);
}

impl NeoParamEncoder for &str {
    fn serialize(&self, builder: &mut NeoParamBuilder) {
        builder.string(self);
    }
}

impl NeoParamEncoder for &[u8] {
    fn serialize(&self, builder: &mut NeoParamBuilder) {
        builder.bytearray(self);
    }
}

impl NeoParamEncoder for bool {
    fn serialize(&self, builder: &mut NeoParamBuilder) {
        builder.bool(*self);
    }
}

impl NeoParamEncoder for H256 {
    fn serialize(&self, builder: &mut NeoParamBuilder) {
        builder.h256(self.clone());
    }
}

impl NeoParamEncoder for U128 {
    fn serialize(&self, builder: &mut NeoParamBuilder) {
        builder.number(self.clone());
    }
}

impl NeoParamEncoder for Address {
    fn serialize(&self, builder: &mut NeoParamBuilder) {
        builder.address(&self);
    }
}

impl<T: NeoParamEncoder> NeoParamEncoder for &T {
    fn serialize(&self, builder: &mut NeoParamBuilder) {
        (*self).serialize(builder)
    }
}

pub trait NeoParamDecoder<'a>: Sized {
    fn deserialize(parser: &mut NeoParamParser<'a>) -> Result<Self, Error>;
}

impl<'a> NeoParamDecoder<'a> for &'a str {
    fn deserialize(parser: &mut NeoParamParser<'a>) -> Result<Self, Error> {
        parser.string()
    }
}

impl<'a> NeoParamDecoder<'a> for &'a [u8] {
    fn deserialize(parser: &mut NeoParamParser<'a>) -> Result<Self, Error> {
        parser.bytearray()
    }
}

impl<'a> NeoParamDecoder<'a> for bool {
    fn deserialize(parser: &mut NeoParamParser<'a>) -> Result<Self, Error> {
        parser.bool()
    }
}

impl<'a> NeoParamDecoder<'a> for &'a H256 {
    fn deserialize(parser: &mut NeoParamParser<'a>) -> Result<Self, Error> {
        parser.h256()
    }
}

impl<'a> NeoParamDecoder<'a> for U128 {
    fn deserialize(parser: &mut NeoParamParser<'a>) -> Result<Self, Error> {
        parser.number()
    }
}

impl<'a> NeoParamDecoder<'a> for &'a Address {
    fn deserialize(parser: &mut NeoParamParser<'a>) -> Result<Self, Error> {
        parser.address()
    }
}

impl<'a, T: NeoParamDecoder<'a>> NeoParamDecoder<'a> for &'a T {
    fn deserialize(parser: &mut NeoParamParser) -> Result<&'a T, Error> {
        parser.read()
    }
}
