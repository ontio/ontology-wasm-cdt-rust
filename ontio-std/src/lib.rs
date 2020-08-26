#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]
#![feature(alloc_prelude)]
#![feature(slice_concat_ext)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(exclusive_range_pattern)]
#![feature(proc_macro_hygiene)]
#![feature(panic_info_message)]

//#![feature(trace_macros)]

cfg_if::cfg_if! {
    if #[cfg(all(not(feature = "std"), feature = "bump-alloc"))] {
        use ontio_bump_alloc::BumpAlloc;
        #[global_allocator]
        static ALLOC: BumpAlloc = BumpAlloc::new();
    } else if #[cfg(not(feature = "std"))] {
        extern crate wee_alloc;
        // Use `wee_alloc` as the global allocator.
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        use prelude::*;
        /// Overrides the default panic_fmt
        #[no_mangle]
        #[panic_handler]
        pub fn panic_fmt(info: &core::panic::PanicInfo) -> ! {
            let msg = info.message().map(|msg| format!("{}", msg)).unwrap_or_default();
            let (file, line) = if let Some(loc) = info.location() {
                (loc.file(), loc.line())
            } else {
                ("", 0)
            };


            let panic_msg = format!("{} at {}:{}", msg, file, line);
            runtime::panic(&panic_msg)
        }

        #[lang = "eh_personality"]
        extern "C" fn eh_personality() {}

        /// Overrides the default oom
        #[lang = "oom"]
        #[no_mangle]
        pub extern fn oom(_: core::alloc::Layout) -> ! {
            unsafe { core::intrinsics::abort() }
        }
    }

}

extern crate alloc;
///The prelude module provides common data types in the contract, and introduces some functions in the rust core library.
pub mod prelude {
    pub use crate::contract::TransferParam;
    pub use crate::types::{Address, H256, I128, U128};
    pub use alloc::boxed::Box;
    pub use alloc::prelude::*;
    pub use alloc::str;
    pub use alloc::string::{self, String, ToString};
    pub use alloc::vec::Vec;
    pub use alloc::{format, vec};
    pub use core::cmp;
    pub use core::prelude::v1::*;
}

///The abi module provides serialization and deserialization methods for different data types in the contract
pub mod abi;
///The console module provides the debug function, which is used to print the log information in the contract and facilitate the debugging of the contract.
pub mod console;
///The contract module provides the method to call the asset ont, ong on the ontology chain.
pub mod contract;
///The database module provides the interface to save the data in the contract to the chain and query the data from the chain.
pub mod database;
///The runtime module provides an interface to interact with the chain in the contract
pub mod runtime;
///The types module provides common data types such as address, U128, hash, etc.
pub mod types;

pub mod utils;
///Macro module provides common macro functions, such as base58! To convert base58 encoded address to address type
pub mod macros {
    pub use ontio_codegen::base58;
    pub use ontio_codegen::contract;
    pub use ontio_codegen::event;
}

#[cfg(feature = "mock")]
pub mod mock;
