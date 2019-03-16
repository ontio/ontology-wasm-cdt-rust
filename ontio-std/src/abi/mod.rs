mod codec;
mod sink;
mod source;
mod zero_copy_source;

pub use self::sink::Sink;
pub use self::source::Source;
pub use self::zero_copy_source::ZeroCopySource;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    IrregularData,
    InvalidUtf8,
}

pub trait Decoder: Sized {
    fn decode(source: &mut Source) -> Result<Self, Error>;
}

pub trait Encoder {
    fn encode(&self, sink: &mut Sink);
}

pub trait Dispatcher {
    fn dispatch(&mut self, payload: &[u8]) -> crate::Vec<u8>;
}
