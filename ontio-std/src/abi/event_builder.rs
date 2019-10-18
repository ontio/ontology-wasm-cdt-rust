use super::Sink;
use crate::prelude::*;
use crate::runtime;
use byteorder::{ByteOrder, LittleEndian};

const DEFAULT_CAP: usize = 128;
const TYPE_BYTEARRAY: u8 = 0x00;
const TYPE_STRING: u8 = 0x01;
const TYPE_ADDRESS: u8 = 0x02;
const TYPE_BOOL: u8 = 0x03;
const TYPE_INT: u8 = 0x04;
const TYPE_H256: u8 = 0x05;

const TYPE_LIST: u8 = 0x10;
///Entity used to push events in a contract.
pub struct EventBuilder {
    sink: Sink,
    num_entry: u32,
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
    /// # fn main(){
    ///   let mut eb = EventBuilder::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        let mut eb = EventBuilder { sink: Sink::new(DEFAULT_CAP), num_entry: 0 };
        eb.sink.write_bytes(b"evt\0");
        eb.sink.write_byte(TYPE_LIST);
        eb.sink.write_u32(eb.num_entry);

        eb
    }

    ///Push &str type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # fn main() {
    ///   EventBuilder::new().string("notify").notify();
    /// # }
    ///```
    pub fn string(mut self, method: &str) -> Self {
        self.sink.write_byte(TYPE_STRING);
        self.sink.write_u32(method.len() as u32);
        self.sink.write_bytes(method.as_bytes());
        self.num_entry += 1;

        self
    }
    ///Push bytearray type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # fn main() {
    ///   EventBuilder::new().bytearray("notify".as_bytes()).notify();
    /// # }
    ///```
    pub fn bytearray(mut self, bytes: &[u8]) -> Self {
        self.sink.write_byte(TYPE_BYTEARRAY);
        self.sink.write_u32(bytes.len() as u32);
        self.sink.write_bytes(bytes);
        self.num_entry += 1;

        self
    }
    ///Push Address type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # fn main() {
    ///   let addr = Address::repeat_byte(1u8);
    ///   EventBuilder::new().address(addr).notify();
    /// }
    ///```
    pub fn address(mut self, address: &Address) -> Self {
        self.sink.write_byte(TYPE_ADDRESS);
        self.sink.write_bytes(address.as_bytes());
        self.num_entry += 1;

        self
    }
    ///Push U128 type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # use ontio_std::types::U128;
    /// # fn main() {
    ///   EventBuilder::new().number(123 as U128).notify();
    /// # }
    ///```
    pub fn number(mut self, amount: U128) -> Self {
        self.sink.write_byte(TYPE_INT);
        self.sink.write_u128(amount);
        self.num_entry += 1;

        self
    }
    ///Push bool type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # use ontio_std::types::U128;
    /// # fn main() {
    ///   EventBuilder::new().number(123 as U128).notify();
    /// # }
    ///```
    pub fn bool(mut self, b: bool) -> Self {
        self.sink.write_byte(TYPE_BOOL);
        self.sink.write_bool(b);
        self.num_entry += 1;

        self
    }
    ///Push H256 type event in contract
    ///# Example
    ///```no_run
    /// # use ontio_std::abi::EventBuilder;
    /// # use ontio_std::runtime;
    /// # fn main() {
    ///   let hash = runtime::sha256("test");
    ///   EventBuilder::new().h256(hash).notify();
    /// # }
    ///```
    pub fn h256(mut self, hash: H256) -> Self {
        self.sink.write_byte(TYPE_H256);
        self.sink.write_bytes(hash.as_ref());
        self.num_entry += 1;

        self
    }

    ///This function will call the notify interface in the runtime module to push the event out.
    pub fn notify(self) {
        let num_entry = self.num_entry;
        let mut buf = self.sink.into();
        LittleEndian::write_u32(&mut buf[5..9], num_entry);

        runtime::notify(&buf);
    }
}
