mod runtime;

use self::runtime::setup_runtime;
pub use self::runtime::Runtime;
use self::runtime::RuntimeInner;
use crate::types::Address;
use std::cell::RefCell;
use std::rc::Rc;
use std::iter::Iterator;
use crate::abi::{Encoder, Sink};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct RuntimeHandle {
   inner: Rc<RefCell<RuntimeInner>>,
}

impl RuntimeHandle {
    pub fn storage_put<K: AsRef<[u8]>, T:Encoder>(&self, key: K, val: T) -> &Self {
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

    pub fn timestamp(&self, time: u64) ->&Self {
        self.inner.borrow_mut().timestamp = time;
        self
    }

    pub fn block_height(&self, height: u64) ->&Self {
        self.inner.borrow_mut().block_height = height;
        self
    }

    pub fn address(&self, addr: &Address) ->&Self {
        self.inner.borrow_mut().self_addr = addr.clone();
        self
    }

    pub fn caller(&self, caller: &Address) ->&Self {
        self.inner.borrow_mut().caller = caller.clone();
        self
    }

    pub fn witness<T : AsRef<Address>, I:IntoIterator<Item = T>>(&self, addr: I) -> &Self {
        self.inner.borrow_mut().witness = addr.into_iter().map(|a| a.as_ref().clone()).collect();
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
