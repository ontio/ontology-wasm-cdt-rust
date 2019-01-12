mod codec;
mod sink;
mod source;

pub use self::sink::Sink;
pub use self::source::Source;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    IrregularData,
    InvalidUtf8,
}

pub trait AbiCodec: Sized {
    fn decode(source: &mut Source) -> Result<Self, Error>;

    fn encode(self, sink: &mut Sink);
}
