use super::types::{Address, H256};
use super::{vec, Vec};

mod env {
    extern "C" {
        pub fn timestamp() -> u64;
        pub fn blockheight() -> u64;
        pub fn selfaddress(dest: *mut u8);
        pub fn calleraddress(dest: *mut u8);
        pub fn checkwitness(addr: *const u8) -> u32;
        pub fn ret(ptr: *const u8, len: u32) -> !;
        pub fn notify(ptr: *const u8, len: u32);
        pub fn input_length() -> u32;
        pub fn get_input(dst: *mut u8);
        pub fn call_contract(addr: *const u8, input_ptr: *const u8, input_len: u32) -> i32;
        pub fn call_output_length() -> u32;
        pub fn get_call_output(dst: *mut u8);
        pub fn current_block_hash(dest: *mut u8);
        pub fn current_tx_hash(dest: *mut u8);

        pub fn storage_read(key: *const u8, klen: u32, val: *mut u8, vlen: u32, offset: u32) -> u32;
        pub fn storage_write(key: *const u8, klen: u32, val: *const u8, vlen: u32);
        pub fn storage_delete(key: *const u8, klen: u32);
    }
}

//todo : return result
pub fn call_contract(addr: &Address, input: &[u8]) -> Option<Vec<u8>> {
    let addr: &[u8] =addr.as_ref();
    let res = unsafe {
        env::call_contract(addr.as_ptr(), input.as_ptr(), input.len() as u32)
    };
    if res < 0 {
        return None;
    }
    let size =  unsafe {
        env::call_output_length()
    };

    let mut output = vec![0u8; size as usize];
    if size != 0 {
        let value = &mut output[..];
        unsafe {
            env::get_call_output(value.as_mut_ptr());
        }
    }

    Some(output)
}

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
    const INITIAL:usize = 32;
    let mut val = vec![0; INITIAL];
    let size = unsafe {
        env::storage_read(key.as_ptr(), key.len() as u32, val.as_mut_ptr(), val.len() as u32, 0)
    };

    if size == core::u32::MAX {
        return None
    }
    let size = size as usize;
    val.resize(size, 0);
    if size > INITIAL {
        let value = &mut val[INITIAL..];
        debug_assert!(value.len() == size - INITIAL);
        unsafe {
            env::storage_read(key.as_ptr(), key.len() as u32, value.as_mut_ptr(), value.len() as u32, INITIAL as u32)
        };
    }

    Some(val)
}

/// Get timestamp in current block
pub fn timestamp() -> u64 {
    unsafe { env::timestamp() }
}

/// Get current block height
pub fn block_height() -> u64 {
    unsafe { env::blockheight() }
}

/// Get the address of current executing contract
pub fn address() -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::selfaddress(addr.as_mut().as_mut_ptr());
    }

    addr
}
///return Caller's contract address
pub fn caller() -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::calleraddress(addr.as_mut().as_mut_ptr());
    }
    addr
}
///return current block hash
pub fn current_block_hash() -> H256 {
    let mut blockhash = H256::zero();
    unsafe {
        env::current_block_hash(blockhash.as_mut().as_mut_ptr());
    }
    blockhash
}
///return current tx hash
pub fn current_tx_hash() -> H256 {
    let mut txhash = H256::zero();
    unsafe {
        env::current_tx_hash(txhash.as_mut().as_mut_ptr());
    }
    txhash
}
///Check signature
pub fn check_witness(addr: &Address) -> bool {
    unsafe { env::checkwitness(addr.as_ptr()) != 0 }
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
