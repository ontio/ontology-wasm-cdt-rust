use super::abi::{Sink,Source, AbiCodec};
use super::runtime;

pub fn get<T:AbiCodec>(key: &[u8]) -> Option<T> {
    runtime::storage_read(key).map(|val|{
        let mut source = Source::new(val);
        source.read().unwrap()
    })
}

pub fn put<T:AbiCodec>(key: &[u8], val: T) {
    let mut sink = Sink::new(12);
    sink.write(val);

    runtime::storage_write(key, &sink.into());
}
