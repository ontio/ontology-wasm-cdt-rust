use crate::types::{Address, H256};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

/// Mock of contract execution runtime
#[derive(Default)]
pub struct Runtime{
    pub(crate) inner: Rc<RefCell<RuntimeInner>>,
}

#[derive(Default)]
pub(crate) struct RuntimeInner{
    pub(crate) storage: HashMap<Vec<u8>, Vec<u8>>,
    pub(crate) timestamp: u64,
    pub(crate) block_height: u64,
    pub(crate) caller: Address,
    pub(crate) entry_address: Address,
    pub(crate) self_addr:Address,
    pub(crate) block_hash: H256,
    pub(crate) tx_hash: H256,
    pub(crate) witness: Vec<Address>,
    pub(crate) notify: Vec<Vec<u8>>,
}

impl Runtime {
    fn storage_write(&self, key: &[u8], val: &[u8]) {
        self.inner.borrow_mut().storage.insert(key.into(), val.to_vec());
    }

    fn storage_read(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.inner.borrow().storage.get(key).map(|val| val.to_vec())
    }

    fn storage_delete(&self, key: &[u8]) {
        self.inner.borrow_mut().storage.remove(key);
    }

    fn timestamp(&self) -> u64 {
        self.inner.borrow().timestamp
    }

    fn block_height(&self) -> u64 {
        self.inner.borrow().block_height
    }

    fn address(&self) -> Address {
        self.inner.borrow().self_addr.clone()
    }

    fn caller(&self) -> Address {
        self.inner.borrow().caller.clone()
    }

    fn check_witness(&self, addr: &Address) -> bool {
        self.inner.borrow().witness.iter().position(|wit| wit == addr).is_some()
    }

    fn entry_address(&self) -> Address {
        self.inner.borrow().entry_address.clone()
    }

    fn current_blockhash(&self) -> H256 {
        self.inner.borrow().block_hash.clone()
    }

    fn current_txhash(&self) -> H256 {
        self.inner.borrow().tx_hash.clone()
    }

    fn notify(&self, msg: &[u8]) {
        self.inner.borrow_mut().notify.push(msg.to_vec());
    }
}

thread_local!(static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::default()));

pub fn setup_runtime(runtime: Runtime) {
    RUNTIME.with(|r| *r.borrow_mut() = runtime);
}

mod env {
    use super::*;
    use std::slice;
    use std::ptr;
    use std::u32;
    use std::cmp;

    #[no_mangle]
    pub unsafe extern "C" fn timestamp() -> u64 {
        RUNTIME.with(|r| r.borrow().timestamp())
    }

    #[no_mangle]
    pub unsafe extern "C" fn block_height() -> u64 {
        RUNTIME.with(|r| r.borrow().block_height())
    }

    #[no_mangle]
    pub unsafe extern "C" fn self_address(dest: *mut u8) {
        RUNTIME.with(|r| {
            let addr = r.borrow().address();
             ptr::copy(addr.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn caller_address(dest: *mut u8) {
        RUNTIME.with(|r| {
            let caller = r.borrow().caller();
            ptr::copy(caller.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn entry_address(dest: *mut u8) {
        RUNTIME.with(|r| {
            let entry = r.borrow().entry_address();
            ptr::copy(entry.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn current_blockhash(dest: *mut u8) {
        RUNTIME.with(|r| {
            let block_hash = r.borrow().current_blockhash();
            ptr::copy(block_hash.as_ptr(), dest, H256::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn current_txhash(dest: *mut u8) {
        RUNTIME.with(|r| {
            let tx_hash = r.borrow().current_txhash();
            ptr::copy(tx_hash.as_ptr(), dest, H256::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn check_witness(addr: *const u8) -> bool {
        let address = Address::from_slice(slice::from_raw_parts(addr, 20));
        RUNTIME.with(|r| r.borrow().check_witness(&address))
    }

    #[no_mangle]
    pub unsafe fn storage_read(key: *const u8, klen: u32, val: *mut u8, vlen: u32, offset: u32) -> u32 {
        let offset = offset as usize;
        let key = slice::from_raw_parts(key, klen as usize);
        let v = RUNTIME.with(|r| r.borrow().storage_read(key));
        match v {
            None => u32::MAX,
            Some(v) => {
                ptr::copy(v.as_slice()[offset..].as_ptr(), val, cmp::min(vlen as usize, v.len() - offset));
                v.len() as u32
            }
        }
    }

    #[no_mangle]
    pub unsafe fn storage_write(key: *const u8, klen: u32, val: *const u8, vlen: u32) {
        let key = slice::from_raw_parts(key, klen as usize);
        let val = slice::from_raw_parts(val, vlen as usize);
        RUNTIME.with(|r| r.borrow().storage_write(key, val));
    }

    #[no_mangle]
    pub unsafe fn storage_delete(key: *const u8, klen: u32) {
        let key = slice::from_raw_parts(key, klen as usize);
        RUNTIME.with(|r| r.borrow().storage_delete(key));
    }

    #[no_mangle]
    pub unsafe fn notify(ptr: *const u8, len: u32) {
        let msg = slice::from_raw_parts(ptr, len as usize);
        RUNTIME.with(|r| r.borrow().notify(msg));
    }
}
