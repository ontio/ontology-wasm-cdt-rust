#![no_std]

extern crate ontio_std as ostd;

use ostd::Box;
use ostd::format;

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
#[no_mangle]
pub fn add_4(a:i32, b:i32) ->i32 {
	a + b + 4
}

#[inline(never)]
#[no_mangle]
fn add_5(a:i32, b:i32) ->i32 {
	a + b + 4
}

#[no_mangle]
pub fn add(a:i32, b:i32) ->i32 {
	add_5(a, b)
}

#[no_mangle]
pub fn boxed(a:i32) ->i32 {
    let addr = ostd::runtime::address();
    ostd::runtime::check_witness(&addr);
    ostd::console::debug(&format!("hahh{:?}", addr));
    let b = Box::new(a);
    *b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
