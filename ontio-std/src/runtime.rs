use super::types::{Addr, Address, H256};
use super::{vec, Vec};

mod env {
    extern "C" {
        pub fn timestamp() -> u64;
        pub fn block_height() -> u32;
        pub fn self_address(dest: *mut u8);
        pub fn caller_address(dest: *mut u8);
        pub fn entry_address(dest: *mut u8);
        pub fn check_witness(addr: *const u8) -> u32;
        pub fn ret(ptr: *const u8, len: u32) -> !;
        pub fn notify(ptr: *const u8, len: u32);
        pub fn input_length() -> u32;
        pub fn get_input(dst: *mut u8);
        pub fn call_contract(addr: *const u8, input_ptr: *const u8, input_len: u32) -> i32;
        pub fn call_output_length() -> u32;
        pub fn get_call_output(dst: *mut u8);
        pub fn current_blockhash(blockhash: *const u8) -> u32;
        pub fn current_txhash(txhash: *const u8) -> u32;
        pub fn contract_migrate(
            code: *const u8,
            code_len: u32,
            vm_type: u32,
            name_ptr: *const u8,
            name_len: u32,
            ver_ptr: *const u8,
            ver_len: u32,
            author_ptr: *const u8,
            author_len: u32,
            email_ptr: *const u8,
            email_len: u32,
            desc_ptr: *const u8,
            desc_len: u32,
            new_address_ptr: *mut u8,
        ) -> i32;
        //        pub fn contract_delete();
        pub fn storage_read(key: *const u8, klen: u32, val: *mut u8, vlen: u32, offset: u32)
            -> u32;
        pub fn storage_write(key: *const u8, klen: u32, val: *const u8, vlen: u32);
        pub fn storage_delete(key: *const u8, klen: u32);
    }
}

//todo : return result
pub fn call_contract<T: AsRef<Addr>>(addr: &T, input: &[u8]) -> Option<Vec<u8>> {
    let addr: &[u8] = addr.as_ref().as_ref();
    let res = unsafe { env::call_contract(addr.as_ptr(), input.as_ptr(), input.len() as u32) };
    if res < 0 {
        return None;
    }
    let size = unsafe { env::call_output_length() };

    let mut output = vec![0u8; size as usize];
    if size != 0 {
        let value = &mut output[..];
        unsafe {
            env::get_call_output(value.as_mut_ptr());
        }
    }

    Some(output)
}
///contract migrate
pub fn contract_migrate(
    code: &[u8],
    vm_type: u32,
    name: &str,
    version: &str,
    author: &str,
    email: &str,
    desc: &str,
) -> Option<Address> {
    let mut addr: Address = Address::zero();
    let res = unsafe {
        env::contract_migrate(
            code.as_ptr(),
            code.len() as u32,
            vm_type,
            name.as_ptr(),
            name.len() as u32,
            version.as_ptr(),
            version.len() as u32,
            author.as_ptr(),
            author.len() as u32,
            email.as_ptr(),
            email.len() as u32,
            desc.as_ptr(),
            desc.len() as u32,
            addr.as_mut().as_mut_ptr(),
        )
    };
    if res < 0 {
        return None;
    }
    Some(addr)
}
//pub fn contract_delete() {
//    unsafe {
//        env::contract_delete();
//    }
//}

pub fn storage_write(key: &[u8], val: &[u8]) {
    unsafe {
        env::storage_write(key.as_ptr(), key.len() as u32, val.as_ptr(), val.len() as u32);
    }
}

pub fn storage_delete(key: &[u8]) {
    unsafe {
        env::storage_delete(key.as_ptr(), key.len() as u32);
    }
}

pub fn storage_read(key: &[u8]) -> Option<Vec<u8>> {
    const INITIAL: usize = 32;
    let mut val = vec![0; INITIAL];
    let size = unsafe {
        env::storage_read(key.as_ptr(), key.len() as u32, val.as_mut_ptr(), val.len() as u32, 0)
    };

    if size == core::u32::MAX {
        return None;
    }
    let size = size as usize;
    val.resize(size, 0);
    if size > INITIAL {
        let value = &mut val[INITIAL..];
        debug_assert!(value.len() == size - INITIAL);
        unsafe {
            env::storage_read(
                key.as_ptr(),
                key.len() as u32,
                value.as_mut_ptr(),
                value.len() as u32,
                INITIAL as u32,
            )
        };
    }

    Some(val)
}

/// Get timestamp in current block
pub fn timestamp() -> u64 {
    unsafe { env::timestamp() }
}

/// Get current block height
pub fn block_height() -> u32 {
    unsafe { env::block_height() }
}

/// Get the address of current executing contract
pub fn address() -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::self_address(addr.as_mut().as_mut_ptr());
    }

    addr
}
///return Caller's contract address
pub fn caller() -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::caller_address(addr.as_mut().as_mut_ptr());
    }
    addr
}
/// return the entry address
pub fn entry_address() -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::entry_address(addr.as_mut().as_mut_ptr());
    }
    addr
}
///return current block hash
pub fn current_blockhash() -> H256 {
    let temp: [u8; 32] = [0; 32];
    let block_hash = H256::new(temp);
    unsafe {
        env::current_blockhash(block_hash.as_ptr());
    }
    block_hash
}
///return current tx hash
pub fn current_txhash() -> H256 {
    let temp: [u8; 32] = [0; 32];
    let tx_hash = H256::new(temp);
    unsafe {
        env::current_txhash(tx_hash.as_ptr());
    }
    tx_hash
}
///Check signature
pub fn check_witness<T: AsRef<Addr>>(addr: T) -> bool {
    unsafe { env::check_witness(addr.as_ref().as_ptr()) != 0 }
}

/// Get input data from transaction or caller contract
pub fn input() -> Vec<u8> {
    let len = unsafe { env::input_length() };

    if len == 0 {
        Vec::new()
    } else {
        let mut data = super::vec![0;len as usize];
        unsafe {
            env::get_input(data.as_mut_ptr());
        }
        data
    }
}

/// return the result of execution and exit contract execution
pub fn ret(data: &[u8]) -> ! {
    unsafe {
        env::ret(data.as_ptr(), data.len() as u32);
    }
}
///Save event
pub fn notify(data: &[u8]) {
    unsafe {
        env::notify(data.as_ptr(), data.len() as u32);
    }
}
