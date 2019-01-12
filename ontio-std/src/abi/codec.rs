use super::AbiCodec;
use super::Error;
use super::{Sink, Source};

use crate::cmp;
use crate::types::{Address, H256, U256};
use crate::{Vec, String};

impl AbiCodec for u8 {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        source.read_byte()
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_byte(self)
    }
}

impl AbiCodec for u16 {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        source.read_u16()
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_u16(self)
    }
}

impl AbiCodec for u32 {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        source.read_u32()
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_u32(self)
    }
}

impl AbiCodec for u64 {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        source.read_u64()
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_u64(self)
    }
}

impl AbiCodec for bool {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        source.read_bool()
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_bool(self)
    }
}

impl AbiCodec for Address {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        let mut addr = Address::zero();
        source.read_into(addr.as_mut())?;
        Ok(addr)
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_bytes(self.as_ref())
    }
}

impl AbiCodec for H256 {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        let mut hash = H256::zero();
        source.read_into(hash.as_mut())?;
        Ok(hash)
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_bytes(self.as_ref())
    }
}

impl AbiCodec for U256 {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        let mut buf = [0; 32];
        source.read_into(buf.as_mut())?;
        Ok( U256::from_little_endian(&buf) )
    }

    fn encode(self, sink: &mut Sink) {
        let mut buf = [0; 32];
        self.to_little_endian(&mut buf);
        sink.write_bytes(&buf)
    }
}

// TODO: implement Vec<u8> for performence when specialization is ready
impl<T: AbiCodec> AbiCodec for Vec<T> {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        let len = source.read_varuint()?;
        let mut value = Vec::with_capacity(cmp::min(len, 1024) as usize);
        for _i in 0..len {
            value.push(source.read::<T>()?);
        }

        Ok(value)
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_varuint(self.len() as u64);
        for item in self {
            sink.write(item);
        }
    }
}

impl AbiCodec for String {
    fn decode(source: &mut Source) -> Result<Self, Error> {
        let len = source.read_varuint()?;
        let bytes = source.next_bytes(len as usize)?;
        String::from_utf8(bytes.into()).map_err(|_| Error::InvalidUtf8)
    }

    fn encode(self, sink: &mut Sink) {
        sink.write_varuint(self.len() as u64);
        sink.write_bytes(self.as_bytes());
    }
}

macro_rules! impl_abi_codec_fixed_array {
    () => {};
    ($num:expr) => {
        impl AbiCodec for [u8; $num] {
            fn decode(source: &mut Source) -> Result<Self, Error> {
                let mut array = [0;$num];
                source.read_into(&mut array)?;
                Ok(array)
            }

            fn encode(self, sink: &mut Sink) {
                sink.write_bytes(&self)
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
        impl<$($item: AbiCodec),*> AbiCodec for ($($item,)*) {
            fn decode(_source: &mut Source) -> Result<Self, Error> {
                Ok(($(_source.read::<$item>()?,)*))
            }

            fn encode(self, _sink: &mut Sink) {
                #[allow(non_snake_case)]
                let ($($item,)*) = self;
                $(_sink.write($item);)*
            }
        }
    }
}
