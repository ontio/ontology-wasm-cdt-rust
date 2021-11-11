use super::Error;
use super::Sink;
use super::{Decoder, Encoder, VmValueBuilder, VmValueDecoder, VmValueEncoder, VmValueParser};

use crate::abi::Source;
use crate::prelude::*;
use crate::types::{Address, H256};

impl<'a> Decoder<'a> for u8 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_byte()
    }
}

impl<'a> Decoder<'a> for &'a Address {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_address()
    }
}

impl<'a> Decoder<'a> for Address {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        let mut addr = Address::zero();
        source.read_into(addr.as_mut())?;
        Ok(addr)
    }
}

impl<'a> Decoder<'a> for &'a [u8] {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_bytes()
    }
}

impl<'a, T: Decoder<'a>> Decoder<'a> for Vec<T> {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        let len = source.read_varuint()?;
        let mut value = Vec::with_capacity(cmp::min(len, 1024) as usize);
        for _i in 0..len {
            value.push(source.read::<T>()?);
        }

        Ok(value)
    }
}

impl<'a, T: Decoder<'a>> Decoder<'a> for Option<T> {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        let is_val: bool = source.read()?;
        if is_val {
            Ok(Some(source.read()?))
        } else {
            Ok(None)
        }
    }
}

impl<'a> Decoder<'a> for &'a str {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        let buf = source.read_bytes()?;
        str::from_utf8(buf).map_err(|_| Error::InvalidUtf8)
    }
}

impl<'a> Decoder<'a> for String {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        let s: &str = source.read()?;
        Ok(s.to_string())
    }
}

impl<'a> Decoder<'a> for u16 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u16()
    }
}

impl<'a> Decoder<'a> for u32 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u32()
    }
}

impl<'a> Decoder<'a> for u64 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u64()
    }
}

impl<'a> Decoder<'a> for bool {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_bool()
    }
}

impl<'a> Decoder<'a> for &'a H256 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_h256()
    }
}

impl<'a> Decoder<'a> for H256 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_h256().map(H256::clone)
    }
}

impl<'a> Decoder<'a> for U128 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u128()
    }
}

impl<'a> Decoder<'a> for I128 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        Ok(source.read_u128()?.to_i128())
    }
}

impl Encoder for u8 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_byte(*self)
    }
}

impl Encoder for u16 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_u16(*self)
    }
}

impl Encoder for u32 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_u32(*self)
    }
}

impl Encoder for U128 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(&self.to_le_bytes())
    }
}

impl Encoder for I128 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(&self.to_le_bytes())
    }
}

impl Encoder for u64 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_u64(*self)
    }
}

impl Encoder for bool {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bool(*self)
    }
}

impl Encoder for Address {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(self.as_ref())
    }
}

impl Encoder for H256 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(self.as_ref())
    }
}

impl<T: Encoder> Encoder for Vec<T> {
    fn encode(&self, sink: &mut Sink) {
        self.as_slice().encode(sink);
    }
}

impl<T> Encoder for &[T]
where
    T: Encoder,
{
    fn encode(&self, sink: &mut Sink) {
        sink.write_varuint(self.len() as u64);
        for item in *self {
            sink.write(item);
        }
    }
}

impl<T: Encoder> Encoder for Option<T> {
    fn encode(&self, sink: &mut Sink) {
        if let Some(val) = self {
            sink.write(true);
            sink.write(val);
        } else {
            sink.write(false);
        }
    }
}

impl Encoder for &str {
    fn encode(&self, sink: &mut Sink) {
        sink.write_varuint(self.len() as u64);
        sink.write_bytes(self.as_bytes());
    }
}

impl Encoder for String {
    fn encode(&self, sink: &mut Sink) {
        self.as_str().encode(sink)
    }
}

impl<T: Encoder> Encoder for &T {
    fn encode(&self, sink: &mut Sink) {
        (*self).encode(sink)
    }
}

impl<T: Encoder> Encoder for &mut T {
    fn encode(&self, sink: &mut Sink) {
        (*self as &T).encode(sink)
    }
}

impl<T: Encoder, const N: usize> Encoder for [T; N] {
    fn encode(&self, sink: &mut Sink) {
        for val in self {
            sink.write(val)
        }
    }
}

impl<'a, T: Decoder<'a> + Default + Copy, const N: usize> Decoder<'a> for [T; N] {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        let mut array = [Default::default(); N];
        for val in &mut array {
            *val = source.read()?;
        }
        Ok(array)
    }
}

/// reference:
/// 1. https://github.com/rust-lang/rust/issues/24830
/// 2. https://github.com/rust-lang/rust/blob/8f5b5f94dcdb9884737dfbc8efd893d1d70f0b14/src/libcore/hash/mod.rs#L239
/// 3. https://github.com/rust-num/num/pull/89/files
macro_rules! for_each_tuple_ {
    ($m:ident !!) => {
        $m! { }
    };
    ($m:ident !! $h:ident, $($t:ident,)*) => {
        $m! { $h $($t)* }
        for_each_tuple_! { $m !! $($t,)* }
    }
}
macro_rules! for_each_tuple {
    ($($m:tt)*) => {
        macro_rules! m { $($m)* }
        for_each_tuple_! { m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, }
    }
}

//trace_macros!(true);
for_each_tuple! {
    ($($item:ident)*) => {
        impl<'a, $($item: Decoder<'a>),*> Decoder<'a> for ($($item,)*) {
            fn decode(_source: &mut Source<'a>) -> Result<Self, Error> {
                Ok(($(_source.read::<$item>()?,)*))
            }
        }

        impl<$($item: Encoder),*> Encoder for ($($item,)*) {
            fn encode(&self, _sink: &mut Sink) {
                #[allow(non_snake_case)]
                let ($($item,)*) = self;
                $(_sink.write($item);)*
            }
        }
    }
}

//trace_macros!(true);
for_each_tuple! {
    ($($item:ident)*) => {
        impl<'a, $($item: VmValueDecoder<'a>),*> VmValueDecoder<'a> for ($($item,)*) {
            fn deserialize(_parser: &mut VmValueParser<'a>) -> Result<Self, Error> {
                let ty = _parser.source.read_byte()?;
                if ty != crate::abi::event_builder::TYPE_LIST {
                     return Err(Error::TypeInconsistency);
                }
                #[allow(unused_mut)]
                let mut count = 0u32;
                $(let _ :$item; count +=1;)*
                let l = _parser.source.read_u32()?;
                if l!= count {
                    return Err(Error::LengthInconsistency);
                }
                Ok(($(_parser.read::<$item>()?,)*))
            }
        }
        impl<$($item: VmValueEncoder),*> VmValueEncoder for ($($item,)*) {
            fn serialize(&self, _builder: &mut VmValueBuilder) {
                _builder.common.sink.write_byte(crate::abi::event_builder::TYPE_LIST);
                 #[allow(unused_mut)]
                let mut count = 0u32;
                #[allow(non_snake_case)]
                let ($($item,)*) = self;
                $(let _ = $item;count +=1;)*
                _builder.common.sink.write_u32(count);
                $(_builder.write($item);)*
            }
        }
    }
}

#[test]
fn test_array() {
    let addrs = [Address::zero(), Address::repeat_byte(1)];
    let mut sink = Sink::new(10);
    sink.write(addrs);
    let buf = sink.into();
    let addrs2: [Address; 2] = Source::new(buf.as_slice()).read().unwrap();
    assert_eq!(addrs, addrs2);
}
