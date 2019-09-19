mod codec;
mod sink;
mod source;

pub use self::sink::Sink;
pub use self::source::Source;

pub use ontio_derive_codec::*;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    IrregularData,
    InvalidUtf8,
}

pub trait Encoder {
    fn encode(&self, sink: &mut Sink);
}

pub trait Dispatcher {
    fn dispatch(&mut self, payload: &[u8]) -> crate::Vec<u8>;
}

#[doc(hidden)]
pub trait Decoder2<'a>: Sized {
    fn decode2(source: &mut Source<'a>) -> Result<Self, Error>;
}
