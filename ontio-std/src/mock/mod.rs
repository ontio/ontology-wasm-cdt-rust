mod runtime;

pub use self::runtime::setup_runtime;
pub use self::runtime::Runtime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
