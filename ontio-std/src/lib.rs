#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]
#![feature(alloc_prelude)]
#![feature(slice_concat_ext)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(exclusive_range_pattern)]
#![feature(proc_macro_hygiene)]

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
        /// Overrides the default panic_fmt
        #[no_mangle]
        #[panic_handler]
        pub fn panic_fmt(_info: &core::panic::PanicInfo) -> ! {
            unsafe { core::intrinsics::abort() }
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

pub use alloc::boxed::Box;
pub use alloc::str;
pub use alloc::string::{self, String};
pub use alloc::vec::Vec;
pub use alloc::{format, vec};

pub mod prelude {
    pub use crate::types::{Address, H256, U128, U256};
    pub use alloc::prelude::*;
    pub use alloc::str;
    pub use alloc::string::{String, ToString};
    pub use alloc::vec::Vec;
    pub use alloc::{format, vec};
    pub use core::prelude::v1::*;
}

pub use core::cmp;

pub mod abi;
pub mod console;
pub mod contract;
pub mod database;
pub mod runtime;
pub mod types;
pub mod abi_codegen {
    pub use ontio_codegen::contract;
}

pub use ontio_codegen::base58;

#[cfg(feature = "mock")]
pub mod mock;
