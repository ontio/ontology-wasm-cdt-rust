#![no_std]

extern crate ontio_std as ostd;

#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}
#[no_mangle]
pub fn add_4(a: i32, b: i32) -> i32 {
    a + b + 4
}

#[inline(never)]
#[no_mangle]
fn add_5(a: i32, b: i32) -> i32 {
    a + b + 4
}

use ostd::types::U256;

#[no_mangle]
pub fn add() -> U256 {
    let mut sink = ostd::abi::Sink::new(10);
    sink.write(1 as u32);
    let mut val: U256 = 1023.into();
    for _ in 0..200 {
        val = val * 2.into()
    }

    val
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
