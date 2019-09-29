use super::Sink;
use crate::prelude::*;
use crate::runtime;
use byteorder::{ByteOrder, LittleEndian};

pub const DEFAULT_CAP: usize = 128;
pub const TYPE_BYTEARRAY: u8 = 0x00;
pub const TYPE_STRING: u8 = 0x01;
pub const TYPE_ADDRESS: u8 = 0x02;
pub const TYPE_BOOL: u8 = 0x03;
pub const TYPE_INT: u8 = 0x04;
pub const TYPE_H256: u8 = 0x05;

pub const TYPE_LIST: u8 = 0x10;

pub struct EventBuilder {
    common: NeoParamBuilderCommon,
}

impl EventBuilder {
    pub fn new() -> Self {
        let mut eb = EventBuilder { common: NeoParamBuilderCommon::new() };
        eb.common.sink.write_bytes(b"evt\0");
        eb.common.sink.write_byte(TYPE_LIST);
        eb.common.sink.write_u32(eb.common.num_entry);

        eb
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

    pub fn notify(self) {
        let num_entry = self.common.num_entry;
        let mut buf = self.common.sink.into();
        LittleEndian::write_u32(&mut buf[5..9], num_entry);
        runtime::notify(&buf);
    }
}

pub struct NeoParamBuilderCommon {
    pub(crate) sink: Sink,
    num_entry: u32,
}

impl NeoParamBuilderCommon {
    pub(crate) fn new() -> Self {
        let sink = Sink::new(12);
        Self { sink, num_entry: 0u32 }
    }
    pub fn string(&mut self, method: &str) {
        self.sink.write_byte(TYPE_STRING);
        self.sink.write_u32(method.len() as u32);
        self.sink.write_bytes(method.as_bytes());
        self.num_entry += 1;
    }

    pub fn bytearray(&mut self, bytes: &[u8]) {
        self.sink.write_byte(TYPE_BYTEARRAY);
        self.sink.write_u32(bytes.len() as u32);
        self.sink.write_bytes(bytes);
        self.num_entry += 1;
    }

    pub fn address(&mut self, address: &Address) {
        self.sink.write_byte(TYPE_ADDRESS);
        self.sink.write_bytes(address.as_bytes());
        self.num_entry += 1;
    }

    pub fn number(&mut self, amount: U128) {
        self.sink.write_byte(TYPE_INT);
        self.sink.write_u128(amount);
        self.num_entry += 1;
    }

    pub fn bool(&mut self, b: bool) {
        self.sink.write_byte(TYPE_BOOL);
        self.sink.write_bool(b);
        self.num_entry += 1;
    }

    pub fn h256(&mut self, hash: H256) {
        self.sink.write_byte(TYPE_H256);
        self.sink.write_bytes(hash.as_ref());
        self.num_entry += 1;
    }
}
