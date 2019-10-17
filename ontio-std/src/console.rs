mod env {
    extern "C" {
        pub fn ontio_debug(data: *const u8, len: u32);
    }
}

///Used to print the debug information in the contract, which can be seen in the log of the ontology node
/// # Example
/// ```
/// # use ontio_std::console;
/// # fn main() {
///    console::debug("test");
/// # }
/// ```
pub fn debug(msg: &str) {
    unsafe {
        env::ontio_debug(msg.as_ptr(), msg.len() as u32);
    }
}
