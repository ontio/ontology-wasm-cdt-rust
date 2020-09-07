use super::Sink;
use crate::prelude::*;
use crate::runtime;

pub(crate) const TYPE_BYTEARRAY: u8 = 0x00;
pub(crate) const TYPE_STRING: u8 = 0x01;
pub(crate) const TYPE_ADDRESS: u8 = 0x02;
pub(crate) const TYPE_BOOL: u8 = 0x03;
pub(crate) const TYPE_INT: u8 = 0x04;
pub(crate) const TYPE_H256: u8 = 0x05;
pub(crate) const TYPE_LIST: u8 = 0x10;

///Entity used to push events in a contract.
#[must_use = "this `EventBuilder` should call notify to take effect"]
pub struct EventBuilder {
    common: VmValueBuilderCommon,
}

impl Default for EventBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBuilder {
    ///Create a new eventbuilder instance to push &str, bytearray, Address, U128, bool, H256 type data in the contract.
    ///# Example
    /// ```no_run
    /// # use ontio_std::abi::EventBuilder;
    ///   let mut eb = EventBuilder::new();
    /// ```
    pub fn new() -> Self {
        let mut eb = EventBuilder { common: VmValueBuilderCommon::new() };
        eb.common.sink.write_bytes(b"evt\0");
        eb.common.sink.write_byte(TYPE_LIST);
        eb.common.sink.write_u32(eb.common.num_entry);

        eb
    }

    ///Push &str type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    ///   EventBuilder::new().string("notify").notify();
    ///```
    pub fn string(mut self, method: &str) -> Self {
        self.common.string(method);
        self
    }

    ///Push bytearray type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    ///   EventBuilder::new().bytearray("notify".as_bytes()).notify();
    ///```
    pub fn bytearray(mut self, bytes: &[u8]) -> Self {
        self.common.bytearray(bytes);
        self
    }

    ///Push Address type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # use ontio_std::types::Address;
    ///   let addr = Address::repeat_byte(1u8);
    ///   EventBuilder::new().address(&addr).notify();
    ///```
    pub fn address(mut self, address: &Address) -> Self {
        self.common.address(address);
        self
    }

    ///Push U128 type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # use ontio_std::types::U128;
    ///   EventBuilder::new().number(123 as U128).notify();
    ///```
    pub fn number(mut self, amount: U128) -> Self {
        self.common.number(amount);
        self
    }

    ///Push bool type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # use ontio_std::types::U128;
    ///   EventBuilder::new().number(123 as U128).notify();
    ///```
    pub fn bool(mut self, b: bool) -> Self {
        self.common.bool(b);
        self
    }

    ///Push H256 type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # use ontio_std::runtime;
    ///   let hash = runtime::sha256("test");
    ///   EventBuilder::new().h256(&hash).notify();
    ///```
    pub fn h256(mut self, hash: &H256) -> Self {
        self.common.h256(hash);
        self
    }

    pub fn notify(self) {
        let num_entry = self.common.num_entry;
        let mut buf = self.common.sink.into();
        buf[5..9].copy_from_slice(&num_entry.to_le_bytes());
        runtime::notify(&buf);
    }
}

pub struct VmValueBuilderCommon {
    pub(crate) sink: Sink,
    pub(crate) num_entry: u32,
}

impl VmValueBuilderCommon {
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

    pub fn h256(&mut self, hash: &H256) {
        self.sink.write_byte(TYPE_H256);
        self.sink.write_bytes(hash.as_bytes());
        self.num_entry += 1;
    }
}

// compile-fails

/// ```compile_fail
/// #[deny(unused_must_use)]
/// {
///     EventBuilder::new().bool(true);
/// }
/// ```
fn _event_builder_must_use() {}
