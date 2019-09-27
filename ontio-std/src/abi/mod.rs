mod codec;
mod event_builder;
mod neo_param_builder;
mod neo_parameter_codec;
mod sink;
mod source;

pub use self::sink::Sink;
pub use self::source::Source;
use crate::prelude::*;
pub use event_builder::EventBuilder;
pub use neo_param_builder::NeoParamBuilder;
pub use neo_param_builder::NeoParamParser;
pub use neo_parameter_codec::NeoParamDecoder;
pub use neo_parameter_codec::NeoParamEncoder;

pub use ontio_derive_codec::*;

#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    IrregularData,
    InvalidUtf8,
    TypeInconsistency,
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
