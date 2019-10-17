use super::Encoder;
use crate::prelude::*;
use crate::types::Address;
use byteorder::{ByteOrder, LittleEndian};

use super::source::varuint_encode_size;
///Encoding different types of data into byte array.
pub struct Sink {
    buf: Vec<u8>,
}

impl Sink {
    ///Create a new sink entity, Specify initial capacity.In the process of serialization, the length of the parameter will be serialized first, and then the content of the parameter will be serialized
    ///
    ///# Example
    ///```
    ///let mut sink = Sink::new(0);
    /// sink.write("123");
    ///assert_eq!(sink.bytes(),[3,49,50,51]);
    ///```
    ///
    pub fn new(cap: usize) -> Self {
        Sink { buf: Vec::with_capacity(cap) }
    }

    ///All data types that implement the encode interface can be serialized by calling the write method
    ///# Example
    ///```
    ///let mut sink = Sink::new(0);
    ///let addr = Address::repeat_byte(1u8);
    ///sink.write(addr);
    ///sink.write("123");
    ///sink.write(123 as U128);
    ///```
    pub fn write<T: Encoder>(&mut self, val: T) {
        val.encode(self)
    }

    pub(crate) fn write_byte(&mut self, b: u8) {
        self.buf.push(b)
    }

    pub(crate) fn write_bool(&mut self, b: bool) {
        if b {
            self.write_byte(1)
        } else {
            self.write_byte(0)
        }
    }
    pub(crate) fn write_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data)
    }

    pub(crate) fn write_u16(&mut self, val: u16) {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, val);
        self.write_bytes(&buf)
    }

    pub(crate) fn write_u32(&mut self, val: u32) {
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, val);
        self.write_bytes(&buf)
    }

    pub(crate) fn write_u64(&mut self, val: u64) {
        let mut buf = [0; 8];
        LittleEndian::write_u64(&mut buf, val);
        self.write_bytes(&buf)
    }

    pub(crate) fn write_u128(&mut self, val: u128) {
        self.write_bytes(&val.to_le_bytes())
    }

    #[allow(unused)]
    pub(crate) fn write_i128(&mut self, val: i128) {
        self.write_bytes(&val.to_le_bytes())
    }

    pub(crate) fn write_varuint(&mut self, val: u64) {
        if val < 0xFD {
            self.write_byte(val as u8);
        } else if val < 0xFFFF {
            self.write_byte(0xFD);
            self.write_u16(val as u16);
        } else if val <= 0xFFFF_FFFF {
            self.write_byte(0xFE);
            self.write_u32(val as u32);
        } else {
            self.write_byte(0xFF);
            self.write_u64(val);
        }
    }

    ///When the contract needs to call a method in the native contract, it needs to use this method to serialize the address of the native contract
    ///# Example
    /// ```
    ///use ostd::macros::base58;
    ///const ONT_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");
    ///let mut sink = Sink::new(0);
    ///sink.write_native_address(&ONT_CONTRACT_ADDRESS);
    /// ```
    pub fn write_native_address(&mut self, address: &Address) {
        self.write_byte(20);
        self.write(address);
    }
    ///When the contract needs to call a method in the neovm contract, it needs to use this method to serialize the address of the neovm contract
    ///# Example
    /// ```
    ///use ostd::macros::base58;
    ///const oep4_address: Address = base58!("AbtTQJYKfQxq4UdygDsbLVjE8uRrJ2H3tP");
    ///let mut sink = Sink::new(0);
    ///sink.write_native_address(&oep4_address);
    /// ```
    pub fn write_neovm_address(&mut self, address: &Address) {
        self.write_native_address(address)
    }

    pub fn write_native_varuint(&mut self, val: u64) {
        self.write_byte(varuint_encode_size(val) as u8);
        self.write_varuint(val);
    }

    ///Used to get the serialized result in bytearray format
    /// # Example
    /// ```
    /// use ostd::macros::base58;
    /// const ONT_CONTRACT_ADDRESS: Address = base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");
    /// let mut sink = Sink::new(0);
    /// sink.write_native_address(&ONT_CONTRACT_ADDRESS);
    /// assert_eq!(sink.bytes(),[20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
    /// ```
    pub fn bytes(&self) -> &[u8] {
        &self.buf
    }

    pub fn into(self) -> Vec<u8> {
        self.buf
    }
}
