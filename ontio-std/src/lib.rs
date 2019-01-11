#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]
#![feature(alloc)]
#![cfg_attr(not(feature = "std"), no_std)]

cfg_if::cfg_if! {
    if #[cfg(not(feature = "std"))] {
        extern crate wee_alloc;
        // Use `wee_alloc` as the global allocator.
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
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
pub use alloc::string::String;
pub use alloc::vec::Vec;
pub use alloc::{format, vec};

pub mod console;
pub mod runtime;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
