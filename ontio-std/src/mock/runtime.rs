use crate::types::{Address, H256};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use sha2::Digest;

/// Mock of contract execution runtime
#[derive(Default)]
pub struct Runtime {
    pub(crate) inner: Rc<RefCell<RuntimeInner>>,
}

#[derive(Default)]
pub(crate) struct RuntimeInner {
    pub(crate) storage: HashMap<Vec<u8>, Vec<u8>>,
    pub(crate) timestamp: u64,
    pub(crate) block_height: u64,
    pub(crate) caller: Address,
    pub(crate) entry_address: Address,
    pub(crate) self_addr: Address,
    pub(crate) block_hash: H256,
    pub(crate) tx_hash: H256,
    pub(crate) witness: Vec<Address>,
    pub(crate) notify: Vec<Vec<u8>>,
    pub(crate) call_contract: Option<Box<dyn FnMut(&Address, &[u8]) -> Vec<u8>>>,
    pub(crate) call_output: Vec<u8>,
}

impl RuntimeInner {
    fn call_contract(&mut self, addr: &Address, data: &[u8]) -> u32 {
        let call = self.call_contract.as_mut().expect("call contract callback is not set");
        self.call_output = (call)(addr, data);
        self.call_output.len() as u32
    }
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

    fn sha256(&self, data: &[u8]) -> H256 {
        let hash = sha2::Sha256::new().chain(data).finalize();
        H256::from_slice(hash.as_slice())
    }

    fn call_contract(&self, addr: &Address, data: &[u8]) -> u32 {
        self.inner.borrow_mut().call_contract(addr, data)
    }

    fn get_call_output(&self) -> Vec<u8> {
        self.inner.borrow().call_output.clone()
    }

    fn call_output_length(&self) -> u32 {
        self.inner.borrow().call_output.len() as u32
    }
}

thread_local!(static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::default()));

pub fn setup_runtime(runtime: Runtime) {
    RUNTIME.with(|r| *r.borrow_mut() = runtime);
}

mod env {
    use super::*;
    use std::cmp;
    use std::ptr;
    use std::slice;
    use std::u32;

    #[no_mangle]
    pub unsafe extern "C" fn ontio_timestamp() -> u64 {
        RUNTIME.with(|r| r.borrow().timestamp())
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_block_height() -> u64 {
        RUNTIME.with(|r| r.borrow().block_height())
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_self_address(dest: *mut u8) {
        RUNTIME.with(|r| {
            let addr = r.borrow().address();
            ptr::copy(addr.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_caller_address(dest: *mut u8) {
        RUNTIME.with(|r| {
            let caller = r.borrow().caller();
            ptr::copy(caller.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_entry_address(dest: *mut u8) {
        RUNTIME.with(|r| {
            let entry = r.borrow().entry_address();
            ptr::copy(entry.as_ptr(), dest, Address::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_current_blockhash(dest: *mut u8) {
        RUNTIME.with(|r| {
            let block_hash = r.borrow().current_blockhash();
            ptr::copy(block_hash.as_ptr(), dest, H256::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_current_txhash(dest: *mut u8) {
        RUNTIME.with(|r| {
            let tx_hash = r.borrow().current_txhash();
            ptr::copy(tx_hash.as_ptr(), dest, H256::len_bytes());
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_check_witness(addr: *const u8) -> bool {
        let address = Address::from_slice(slice::from_raw_parts(addr, 20));
        RUNTIME.with(|r| r.borrow().check_witness(&address))
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_storage_read(
        key: *const u8, klen: u32, val: *mut u8, vlen: u32, offset: u32,
    ) -> u32 {
        let offset = offset as usize;
        let key = slice::from_raw_parts(key, klen as usize);
        let v = RUNTIME.with(|r| r.borrow().storage_read(key));
        match v {
            None => u32::MAX,
            Some(v) => {
                ptr::copy(
                    v.as_slice()[offset..].as_ptr(),
                    val,
                    cmp::min(vlen as usize, v.len() - offset),
                );
                v.len() as u32
            }
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_storage_write(
        key: *const u8, klen: u32, val: *const u8, vlen: u32,
    ) {
        let key = slice::from_raw_parts(key, klen as usize);
        let val = slice::from_raw_parts(val, vlen as usize);
        RUNTIME.with(|r| r.borrow().storage_write(key, val));
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_storage_delete(key: *const u8, klen: u32) {
        let key = slice::from_raw_parts(key, klen as usize);
        RUNTIME.with(|r| r.borrow().storage_delete(key));
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_notify(ptr: *const u8, len: u32) {
        let msg = slice::from_raw_parts(ptr, len as usize);
        RUNTIME.with(|r| r.borrow().notify(msg));
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_sha256(ptr: *const u8, len: u32, h256: *mut u8) {
        let msg = slice::from_raw_parts(ptr, len as usize);
        RUNTIME.with(|r| {
            let hash = r.borrow().sha256(msg);
            ptr::copy(hash.as_ptr(), h256, 32);
        });
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_call_contract(
        addr: *const u8, input_ptr: *const u8, input_len: u32,
    ) -> u32 {
        let addr = Address::from_slice(slice::from_raw_parts(addr, 20));
        let input = slice::from_raw_parts(input_ptr, input_len as usize);
        RUNTIME.with(|r| {
            r.borrow().call_contract(&addr, input)
        })
    }

    #[no_mangle]
    pub unsafe extern "C" fn ontio_call_output_length() -> u32 {
        RUNTIME.with(|r| r.borrow().call_output_length())
    }

    #[no_mangle]
    pub fn ontio_panic(_ptr: *const u8, _len: u32) -> ! {
        panic!("ontio panic");
    }

    #[no_mangle]
    pub fn ontio_get_call_output(dst: *mut u8) {
        let output = RUNTIME.with(|r| r.borrow().get_call_output());
        unsafe {
            std::ptr::copy(output.as_ptr(), dst, output.len());
        }
    }

    #[no_mangle]
    pub fn ontio_contract_create(
        _code_ptr: *const u8, _code_len: u32, _need_storage: u32, _name_ptr: *const u8,
        _name_len: u32, _ver_ptr: *const u8, _ver_len: u32, _author_ptr: *const u8,
        _author_len: u32, _email_ptr: *const u8, _email_len: u32, _desc_ptr: *const u8,
        _desc_len: u32, _new_addr_ptr: *mut u8,
    ) -> u32 {
        unimplemented!()
    }

    #[no_mangle]
    pub fn ontio_contract_destroy() -> ! {
        unimplemented!()
    }

    #[no_mangle]
    pub fn ontio_contract_migrate(
        _code: *const u8, _code_len: u32, _vm_type: u32, _name_ptr: *const u8, _name_len: u32,
        _ver_ptr: *const u8, _ver_len: u32, _author_ptr: *const u8, _author_len: u32,
        _email_ptr: *const u8, _email_len: u32, _desc_ptr: *const u8, _desc_len: u32,
        _new_address_ptr: *mut u8,
    ) -> i32 {
        unimplemented!()
    }

    #[no_mangle]
    pub fn ontio_input_length() -> u32 {
        unimplemented!()
    }

    #[no_mangle]
    pub fn ontio_get_input(_dst: *mut u8) {
        unimplemented!()
    }

    #[no_mangle]
    pub fn ontio_return(_ptr: *const u8, _len: u32) -> ! {
        unimplemented!()
    }
}
