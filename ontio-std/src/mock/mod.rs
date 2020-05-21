pub mod contract_mock;
pub use contract_mock::{Command, NeoCommand};
mod runtime;
use self::runtime::setup_runtime;
pub use self::runtime::Runtime;
use self::runtime::RuntimeInner;
use crate::abi::{Encoder, Sink};
use crate::types::{Address, H256};
use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;

pub struct RuntimeHandle {
    inner: Rc<RefCell<RuntimeInner>>,
}

impl RuntimeHandle {
    pub fn storage_put<K: AsRef<[u8]>, T: Encoder>(&self, key: K, val: T) -> &Self {
        let mut sink = Sink::new(12);
        sink.write(val);
        self.inner.borrow_mut().storage.insert(key.as_ref().to_vec(), sink.into());
        self
    }

    pub fn storage_read(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.inner.borrow().storage.get(key).map(|val| val.to_vec())
    }

    pub fn storage_delete(&self, key: &[u8]) -> &Self {
        self.inner.borrow_mut().storage.remove(key);
        self
    }

    pub fn timestamp(&self, time: u64) -> &Self {
        self.inner.borrow_mut().timestamp = time;
        self
    }

    pub fn block_height(&self, height: u64) -> &Self {
        self.inner.borrow_mut().block_height = height;
        self
    }

    pub fn address(&self, addr: &Address) -> &Self {
        self.inner.borrow_mut().self_addr = addr.clone();
        self
    }

    pub fn caller(&self, caller: &Address) -> &Self {
        self.inner.borrow_mut().caller = caller.clone();
        self
    }
    pub fn entry_address(&self, entry: &Address) -> &Self {
        self.inner.borrow_mut().entry_address = entry.clone();
        self
    }

    pub fn current_blockhash(&self, block_hash: &H256) -> &Self {
        self.inner.borrow_mut().block_hash = block_hash.clone();
        self
    }
    pub fn current_txhash(&self, tx_hash: &H256) -> &Self {
        self.inner.borrow_mut().tx_hash = tx_hash.clone();
        self
    }

    pub fn witness<T: AsRef<Address>, I: IntoIterator<Item = T>>(&self, addr: I) -> &Self {
        self.inner.borrow_mut().witness = addr.into_iter().map(|a| a.as_ref().clone()).collect();
        self
    }

    pub fn on_contract_call(
        &self, func: impl FnMut(&Address, &[u8]) -> Option<Vec<u8>> + 'static,
    ) -> &Self {
        self.inner.borrow_mut().call_contract = Some(Box::new(func));
        self
    }
}

pub fn build_runtime() -> RuntimeHandle {
    let inner = Rc::new(RefCell::new(RuntimeInner::default()));

    let rt = Runtime { inner: inner.clone() };
    setup_runtime(rt);

    let handle = RuntimeHandle { inner: inner };
    handle
}

#[test]
fn test_call_contract() {
    assert_eq!(crate::runtime::call_contract(&Address::repeat_byte(1), &[1, 2]), Some(vec![]));

    build_runtime().on_contract_call(|_addr, _data| -> Option<Vec<u8>> { Some(vec![1, 2, 3]) });
    assert_eq!(
        crate::runtime::call_contract(&Address::repeat_byte(1), &[1, 2]),
        Some(vec![1, 2, 3])
    );
}
