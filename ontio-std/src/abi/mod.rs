mod codec;
mod sink;
mod source;
mod vm_value_builder;
mod vm_value_codec;

pub use self::sink::Sink;
pub use self::source::Source;
use crate::prelude::*;
pub(crate) mod event_builder;
pub use event_builder::EventBuilder;
pub(crate) use event_builder::VmValueBuilderCommon;
pub use vm_value_builder::{VmValueBuilder, VmValueParser};
pub use vm_value_codec::VmValueDecoder;
pub use vm_value_codec::VmValueEncoder;

pub use ontio_derive_codec::*;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    IrregularData,
    InvalidUtf8,
    TypeInconsistency,
    LengthInconsistency,
}

pub trait Encoder {
    fn encode(&self, sink: &mut Sink);
}

pub trait Dispatcher {
    fn dispatch(&mut self, payload: &[u8]) -> Vec<u8>;
}

#[doc(hidden)]
pub trait Decoder<'a>: Sized {
    fn decode(source: &mut Source<'a>) -> Result<Self, Error>;
}
