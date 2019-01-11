#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(allocator_api)]
#![feature(alloc)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(not(feature = "std"))]
extern crate wee_alloc;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
#[cfg(not(feature = "std"))]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Overrides the default panic_fmt
#[no_mangle]
#[panic_handler]
#[cfg(not(feature = "std"))]
pub fn panic_fmt(_info: &core::panic::PanicInfo) -> ! {
	unsafe { core::intrinsics::abort() }
}

#[lang = "eh_personality"]
#[cfg(not(feature = "std"))]
extern "C" fn eh_personality() {}

/// Overrides the default oom
#[lang = "oom"]
#[cfg(not(feature = "std"))]
#[no_mangle]
pub extern fn oom(_: core::alloc::Layout) -> ! {
	unsafe { core::intrinsics::abort() }
}

pub use alloc::boxed::Box;
pub use alloc::vec::Vec;
pub use alloc::string::String;
pub use alloc::str;
pub use alloc::{vec, format};

pub mod types;
pub mod runtime;
pub mod console;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
