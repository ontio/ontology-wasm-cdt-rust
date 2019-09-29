use super::Error;
use super::Sink;
use super::{Decoder, Encoder, VmValueBuilder, VmValueDecoder, VmValueEncoder, VmValueParser};

use crate::abi::Source;
use crate::prelude::*;
use crate::types::{Address, H256};
use byteorder::{ByteOrder, LittleEndian};

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

impl<'a> Decoder<'a> for u128 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        source.read_u128()
    }
}

impl<'a> Decoder<'a> for i128 {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
        Ok(source.read_u128()? as i128)
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

impl Encoder for u128 {
    fn encode(&self, sink: &mut Sink) {
        sink.write_bytes(&self.to_le_bytes())
    }
}

impl Encoder for i128 {
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

macro_rules! impl_abi_codec_fixed_array {
    () => {};
    ($num:expr) => {
        impl<'a> Decoder<'a> for [u8; $num] {
            fn decode(source: &mut Source<'a>) -> Result<Self, Error> {
                let mut array = [0;$num];
                source.read_into(&mut array)?;
                Ok(array)
            }
        }

        impl Encoder for [u8; $num] {
            fn encode(&self, sink: &mut Sink) {
                sink.write_bytes(self)
            }
        }
    } ;
    ($num:expr, $($tail:expr),*) => {
        impl_abi_codec_fixed_array!($num);
        impl_abi_codec_fixed_array!($($tail),*);
     };
}

impl_abi_codec_fixed_array!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32
);

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
                let mut count = 0u32;
                #[allow(non_snake_case)]
                let ($($item,)*) = self;
                $($item;count +=1;)*
                _builder.common.sink.write_u32(count);
                $(_builder.write($item);)*
            }
        }
    }
}
