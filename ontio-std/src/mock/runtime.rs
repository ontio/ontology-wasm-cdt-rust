use crate::types::Address;
use std::cell::RefCell;
use std::collections::HashMap;

/// Mock of contract execution runtime
pub trait Runtime {
    fn storage_write(&self, key: &[u8], val: &[u8]);
    fn storage_read(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn storage_delete(&self, key: &[u8]) ;
    fn timestamp(&self) -> u64;
    fn block_height(&self) -> u64;
    fn address(&self) -> Address;
    fn caller(&self) -> Address;
    fn check_witness(&self, addr: &Address) -> bool;
    fn notify(&self, msg: &[u8]);
    //    fn input() -> Vec<u8> ;
    //    fn ret(data: &[u8]) -> ! ;
}

#[derive(Default)]
pub(crate) struct RuntimeImpl {
    pub(crate) storage: RefCell<HashMap<Vec<u8>, Vec<u8>>>,
    pub(crate) timestamp: u64,
    pub(crate) block_height: u64,
    pub(crate) caller: Address,
    pub(crate) witness: Vec<Address>,
    pub(crate) notify: RefCell<Vec<Vec<u8>>>,
}

impl Runtime for RuntimeImpl {
    fn storage_write(&self, key: &[u8], val: &[u8]) {
        self.storage.borrow_mut().insert(key.into(), val.to_vec());
    }

    fn storage_read(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.storage.borrow().get(key).map(|val| val.to_vec())
    }

    fn storage_delete(&self, key: &[u8]) {
        self.storage.borrow_mut().remove(key);
    }

    fn timestamp(&self) -> u64 {
        self.timestamp
    }

    fn block_height(&self) -> u64 {
        self.block_height
    }

    fn address(&self) -> Address {
        self.caller.clone()
    }

    fn caller(&self) -> Address {
        self.caller.clone()
    }

    fn check_witness(&self, addr: &Address) -> bool {
        self.witness.iter().position(|wit| wit == addr).is_some()
    }

    fn notify(&self, msg: &[u8]) {
        self.notify.borrow_mut().push(msg.to_vec());
    }
}

thread_local!(static RUNTIME: RefCell<Box<dyn Runtime>> = RefCell::new(Box::new(RuntimeImpl::default())));

pub fn setup_runtime(runtime: Box<dyn Runtime>) {
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
    pub unsafe extern "C" fn blockheight() -> u64 {
        RUNTIME.with(|r| r.borrow().block_height())
    }

    #[no_mangle]
    pub unsafe extern "C" fn selfaddress(dest: *mut u8) {
        RUNTIME.with(|r| {
            let addr = r.borrow().address();
             ptr::copy(addr.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn calleraddress(dest: *mut u8) {
        RUNTIME.with(|r| {
            let caller = r.borrow().caller();
            ptr::copy(caller.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn checkwitness(addr: *const u8) -> bool {
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

//        pub fn ret(ptr: *const u8, len: u32) -> !;
//        pub fn input_length() -> u32;
//        pub fn get_input(dst: *mut u8);
//


}
