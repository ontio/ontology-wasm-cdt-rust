mod env {
    extern "C" {
        pub fn ontio_debug(data: *const u8, len: u32);
    }
}

pub fn debug(msg: &str) {
    unsafe {
        env::ontio_debug(msg.as_ptr(), msg.len() as u32);
    }
}
