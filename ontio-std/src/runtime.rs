use crate::prelude::*;

mod env {
    extern "C" {
        pub fn ontio_timestamp() -> u64;
        pub fn ontio_block_height() -> u32;
        pub fn ontio_self_address(dest: *mut u8);
        pub fn ontio_caller_address(dest: *mut u8);
        pub fn ontio_entry_address(dest: *mut u8);
        pub fn ontio_check_witness(addr: *const u8) -> u32;
        pub fn ontio_return(ptr: *const u8, len: u32) -> !;
        pub fn ontio_panic(ptr: *const u8, len: u32) -> !;
        pub fn ontio_notify(ptr: *const u8, len: u32);
        pub fn ontio_input_length() -> u32;
        pub fn ontio_get_input(dst: *mut u8);
        pub fn ontio_call_contract(addr: *const u8, input_ptr: *const u8, input_len: u32) -> u32;
        #[allow(unused)]
        pub fn ontio_call_output_length() -> u32;
        pub fn ontio_get_call_output(dst: *mut u8);
        pub fn ontio_current_blockhash(blockhash: *const u8) -> u32;
        pub fn ontio_current_txhash(txhash: *const u8) -> u32;
        pub fn ontio_contract_migrate(
            code: *const u8, code_len: u32, vm_type: u32, name_ptr: *const u8, name_len: u32,
            ver_ptr: *const u8, ver_len: u32, author_ptr: *const u8, author_len: u32,
            email_ptr: *const u8, email_len: u32, desc_ptr: *const u8, desc_len: u32,
            new_address_ptr: *mut u8,
        ) -> i32;
        pub fn ontio_storage_read(
            key: *const u8, klen: u32, val: *mut u8, vlen: u32, offset: u32,
        ) -> u32;
        pub fn ontio_storage_write(key: *const u8, klen: u32, val: *const u8, vlen: u32);
        pub fn ontio_storage_delete(key: *const u8, klen: u32);
        pub fn ontio_sha256(data: *const u8, len: u32, val: *mut u8);
        pub fn ontio_contract_create(
            code_ptr: *const u8, code_len: u32, vm_type: u32, name_ptr: *const u8, name_len: u32,
            ver_ptr: *const u8, ver_len: u32, author_ptr: *const u8, author_len: u32,
            email_ptr: *const u8, email_len: u32, desc_ptr: *const u8, desc_len: u32,
            new_addr_ptr: *mut u8,
        ) -> u32;
        pub fn ontio_contract_destroy() -> !;
    }
}

//todo : return result
/// With this method, another contract can be called, When calling Neo contract and native contract across contracts, the parameter construction is slightly different. Please refer to the corresponding examples.
///
/// addr: Called contract address
///
/// input: Parameters required to call the target contract method
///
/// # Example
/// ```no_run
/// # use ontio_std::abi::Sink;
/// # use ontio_std::runtime;
/// # use ontio_std::types::Address;
/// let mut sink = Sink::new(16);
/// let addr = Address::repeat_byte(1u8);
/// sink.write(("balance_of".to_string(), &addr));
/// let res = runtime::call_contract(&addr, sink.bytes());
/// ```
pub fn call_contract(addr: &Address, input: &[u8]) -> Vec<u8> {
    let addr: &[u8] = addr.as_ref();
    let size =
        unsafe { env::ontio_call_contract(addr.as_ptr(), input.as_ptr(), input.len() as u32) };
    let mut output = vec![0u8; size as usize];
    if size != 0 {
        let value = &mut output[..];
        unsafe {
            env::ontio_get_call_output(value.as_mut_ptr());
        }
    }

    output
}

/// Create a new contract based on the parameters passed in
///
/// code: new contract code
///
/// vm_type: 1 for neovm, 3 for wasmvm
///
/// name: contract name
///
/// ver: contract version
///
/// author: contract author
///
/// email: contract email
///
/// desc: contract desc
///
/// return value: new contract address
///
/// # Example
/// ```no_run
/// # use ontio_std::abi::{Sink, Source};
/// # use ontio_std::runtime;
///   let input = runtime::input();
///   let mut source = Source::new(&input);
///   let (code, vm_type, name, ver, author, email, desc) = source.read().unwrap();
///   let res = runtime::contract_create(code, vm_type, name, ver, author, email, desc);
/// ```
///
pub fn contract_create(
    code: &[u8], vm_type: u32, name: &str, ver: &str, author: &str, email: &str, desc: &str,
) -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::ontio_contract_create(
            code.as_ptr(),
            code.len() as u32,
            vm_type,
            name.as_ptr(),
            name.len() as u32,
            ver.as_ptr(),
            ver.len() as u32,
            author.as_ptr(),
            author.len() as u32,
            email.as_ptr(),
            email.len() as u32,
            desc.as_ptr(),
            desc.len() as u32,
            addr.as_mut().as_mut_ptr(),
        )
    };

    addr
}

///contract migrate
///
/// code: new contract code
///
/// vm_type: 1 for neovm, 3 for wasmvm
///
/// name: contract name
///
/// ver: contract version
///
/// author: contract author
///
/// email: contract email
///
/// desc: contract desc
///
/// return value: new contract address
///
/// # Example
/// ```no_run
/// # use ontio_std::abi::{Sink, Source};
/// # use ontio_std::runtime;
/// let input = runtime::input();
/// let mut source = Source::new(&input);
/// let (code, vm_type, name, ver, author, email, desc) = source.read().unwrap();
/// let res = runtime::contract_migrate(code, vm_type, name, ver, author, email, desc);
/// ```
///
pub fn contract_migrate(
    code: &[u8], vm_type: u32, name: &str, version: &str, author: &str, email: &str, desc: &str,
) -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::ontio_contract_migrate(
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

    addr
}
///delete the contract
pub fn contract_delete() -> ! {
    unsafe {
        env::ontio_contract_destroy();
    }
}
///Save key-value as a key-value pair
///
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let key = "key";
/// let value = "value";
/// runtime::storage_write(key.as_bytes(), value.as_bytes());
/// ```
///
pub fn storage_write(key: &[u8], val: &[u8]) {
    unsafe {
        env::ontio_storage_write(key.as_ptr(), key.len() as u32, val.as_ptr(), val.len() as u32);
    }
}

///Delete key-value pairs according to the key
///
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let key = "key";
/// runtime::storage_delete(key.as_bytes());
/// ```
///
pub fn storage_delete(key: &[u8]) {
    unsafe {
        env::ontio_storage_delete(key.as_ptr(), key.len() as u32);
    }
}
///Read key-value pairs according to key.
///
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let key = "key";
/// let res = runtime::storage_read(key.as_bytes());
/// ```
///
pub fn storage_read(key: &[u8]) -> Option<Vec<u8>> {
    const INITIAL: usize = 32;
    let mut val = vec![0; INITIAL];
    let size = unsafe {
        env::ontio_storage_read(
            key.as_ptr(),
            key.len() as u32,
            val.as_mut_ptr(),
            val.len() as u32,
            0,
        )
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
            env::ontio_storage_read(
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
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let res = runtime::timestamp();
/// ```
///
pub fn timestamp() -> u64 {
    unsafe { env::ontio_timestamp() }
}

/// Get current block height
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let res = runtime::block_height();
/// ```
///
pub fn block_height() -> u32 {
    unsafe { env::ontio_block_height() }
}

/// Get the address of current executing contract
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let res = runtime::address();
/// ```
///
pub fn address() -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::ontio_self_address(addr.as_mut().as_mut_ptr());
    }

    addr
}
///return Caller's contract address
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let res = runtime::caller();
/// ```
///
pub fn caller() -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::ontio_caller_address(addr.as_mut().as_mut_ptr());
    }
    addr
}
/// return the entry address
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let res = runtime::entry_address();
/// ```
///
pub fn entry_address() -> Address {
    let mut addr: Address = Address::zero();
    unsafe {
        env::ontio_entry_address(addr.as_mut().as_mut_ptr());
    }
    addr
}
///return current block hash
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let res = runtime::current_blockhash();
/// ```
///
pub fn current_blockhash() -> H256 {
    let temp: [u8; 32] = [0; 32];
    let block_hash = H256::new(temp);
    unsafe {
        env::ontio_current_blockhash(block_hash.as_ptr());
    }
    block_hash
}
///return current tx hash
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let res = runtime::current_txhash();
/// ```
///
pub fn current_txhash() -> H256 {
    let tx_hash = H256::zero();
    unsafe {
        env::ontio_current_txhash(tx_hash.as_ptr());
    }
    tx_hash
}
/// Calculate the hash value
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// let res = runtime::sha256("test");
/// ```
///
pub fn sha256(data: impl AsRef<[u8]>) -> H256 {
    let data = data.as_ref();
    let mut hash = H256::zero();
    unsafe {
        env::ontio_sha256(data.as_ptr(), data.len() as u32, hash.as_mut_ptr());
    }
    hash
}

///Check signature
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// # use ontio_std::abi::Source;
/// let input = runtime::input();
/// let mut source = Source::new(&input);
/// let addr = source.read().unwrap();
/// let res = runtime::check_witness(addr);
/// ```
///
pub fn check_witness(addr: &Address) -> bool {
    unsafe { env::ontio_check_witness(addr.as_ptr()) != 0 }
}

/// Get input data from transaction or caller contract
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// # use ontio_std::abi::Source;
/// let input = runtime::input();
/// let mut source = Source::new(&input);
/// let addr = source.read().unwrap();
/// let res = runtime::check_witness(addr);
/// ```
///
pub fn input() -> Vec<u8> {
    let len = unsafe { env::ontio_input_length() };

    if len == 0 {
        Vec::new()
    } else {
        let mut data = vec![0; len as usize];
        unsafe {
            env::ontio_get_input(data.as_mut_ptr());
        }
        data
    }
}

/// return the result of execution and exit contract execution
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime;
/// # use ontio_std::abi::{Source, Sink};
///   let input = runtime::input();
///   let mut source = Source::new(&input);
///   let addr = source.read().unwrap();
///   let res = runtime::check_witness(addr);
///   let mut sink = Sink::new(16);
///   sink.write(res);
///   runtime::ret(sink.bytes());
/// ```
///
pub fn ret(data: &[u8]) -> ! {
    unsafe {
        env::ontio_return(data.as_ptr(), data.len() as u32);
    }
}

#[doc(hidden)]
pub fn notify(data: &[u8]) {
    unsafe {
        env::ontio_notify(data.as_ptr(), data.len() as u32);
    }
}
/// When the function is executed, all writes to the chain will be cancelled, and the error message will be returned.
///
/// # Example
///
/// ```no_run
/// # use ontio_std::runtime::panic;
///   panic("panic");
/// ```
///
pub fn panic(msg: &str) -> ! {
    unsafe {
        env::ontio_panic(msg.as_ptr(), msg.len() as u32);
    }
}
