mod runtime;

pub use self::runtime::setup_runtime;
pub use self::runtime::Runtime;

use self::runtime::RuntimeImpl;
use crate::types::Address;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::abi::{Encoder, Sink};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Default)]
pub struct RuntimeBuilder {
    storage: HashMap<Vec<u8>, Vec<u8>>,
    timestamp: u64,
    block_height: u64,
    caller: Address,
    witness: Vec<Address>,
}

impl RuntimeBuilder {
    pub fn storage_put<K: AsRef<[u8]>, T:Encoder>(mut self, key: K, val: T) ->Self {
        let mut sink = Sink::new(12);
        sink.write(val);
        self.storage.insert(key.as_ref().to_vec(), sink.into());
        self
    }

    pub fn timestamp(mut self, time: u64) ->Self {
        self.timestamp = time;
        self
    }

    pub fn block_height(mut self, height: u64) ->Self {
        self.block_height = height;
        self
    }

    pub fn address(mut self, addr: &Address) ->Self {
        self.caller = addr.clone();
        self
    }

    pub fn caller(mut self, caller: &Address) ->Self {
        self.caller= caller.clone();
        self
    }

    pub fn append_witness(mut self, addr: &Address) -> Self {
        self.witness.push(addr.clone());
        self
    }

    pub fn build(self) -> Box<Runtime> {
        let rt = RuntimeImpl {
            storage: RefCell::new(self.storage),
            timestamp: self.timestamp,
            block_height: self.block_height,
            caller: self.caller,
            witness: self.witness,
            notify: RefCell::new(Vec::new()),
        };

        Box::new(rt)
    }
}
