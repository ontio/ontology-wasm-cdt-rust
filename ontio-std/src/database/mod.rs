mod list;

pub use self::list::ListStore;

use super::abi::{Decoder, Encoder, Sink, Source};
use super::prelude::*;
use super::runtime;

#[track_caller]
pub fn get<K: AsRef<[u8]>, T>(key: K) -> Option<T>
where
    for<'a> T: Decoder<'a> + 'static,
{
    let val = runtime::storage_read(key.as_ref())?;
    let mut source = Source::new(&val);
    Some(source.read().unwrap())
}

pub fn put<K: AsRef<[u8]>, T: Encoder>(key: K, val: T) {
    let mut sink = Sink::new(12);
    sink.write(val);
    runtime::storage_write(key.as_ref(), sink.bytes());
}

pub fn delete<K: AsRef<[u8]>>(key: K) {
    runtime::storage_delete(key.as_ref());
}
