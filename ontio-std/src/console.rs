mod env {
    extern "C" {
        pub fn debug(data: *const u8, len: u32);
    }
}

pub fn debug(msg: &str) {
    unsafe {
        env::debug(msg.as_ptr(), msg.len() as u32);
    }
}
