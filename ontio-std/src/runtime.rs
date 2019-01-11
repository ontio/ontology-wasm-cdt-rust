use super::types::Address;
use super::Vec;

mod external {
    extern "C" {
        pub fn timestamp() -> u64;
        pub fn blockheight() -> u64;
        pub fn selfaddress(dest: *mut u8);
        pub fn calleraddress(dest: *mut u8);
        pub fn checkwitness(addr: *const u8) -> u32;
        pub fn ret(ptr: *const u8, len: u32) -> !;
        pub fn input_length() -> u32;
        pub fn get_input(dst: *mut u8);
    }
}

/// Get timestamp in current block
pub fn timestamp() -> u64 {
    unsafe { external::timestamp() }
}

/// Get current block height
pub fn block_height() -> u64 {
    unsafe { external::blockheight() }
}

/// Get the address of current executing contract
pub fn address() -> Address {
    let mut addr: Address = [0;20];
    unsafe {
        external::selfaddress(addr.as_mut().as_mut_ptr());
    }

    addr
}

pub fn caller() ->Address {
    let mut addr: Address = [0;20];
    unsafe {
        external::calleraddress(addr.as_mut().as_mut_ptr());
    }
    addr
}

pub fn check_witness(addr: &Address) -> bool {
    unsafe {
        external::checkwitness(addr.as_ptr()) != 0
    }
}

/// Get input data from transaction or caller contract
pub fn input() -> Vec<u8> {
    let len = unsafe { external::input_length() };

    if len == 0 {
        Vec::new()
    } else {
        let mut data = super::vec![0;len as usize];
        unsafe {
            external::get_input(data.as_mut_ptr());
        }
        data
    }
}

/// return the result of execution and exit contract execution
pub fn ret(data: &[u8]) -> ! {
    unsafe { external::ret(data.as_ptr(), data.len() as u32); }
}

