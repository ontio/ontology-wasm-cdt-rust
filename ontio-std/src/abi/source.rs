use super::Decoder;
use super::Error;
use crate::Vec;
use byteorder::{ByteOrder, LittleEndian};

fn varuint_encode_size(val: u64) -> usize {
    if val < 0xfd {
        1
    } else if val <= 0xffff {
        3
    } else if val <= 0xFFFFFFFF {
        5
    } else {
        9
    }
}

pub struct Source {
    buf: Vec<u8>,
    pos: usize,
}

impl Source {
    pub fn new(data: Vec<u8>) -> Self {
        Self { buf: data, pos: 0 }
    }

    pub fn read<T: Decoder>(&mut self) -> Result<T, Error> {
        T::decode(self)
    }

    pub(crate) fn next_bytes(&mut self, len: usize) -> Result<&[u8], Error> {
        if self.buf.len() - self.pos < len {
            Err(Error::UnexpectedEOF)
        } else {
            let bytes = &self.buf.as_slice()[self.pos..self.pos + len];
            self.pos += len;
            Ok(bytes)
        }
    }

    pub(crate) fn read_into(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let bytes = self.next_bytes(buf.len())?;
        buf.copy_from_slice(bytes);
        Ok(())
    }

    pub(crate) fn read_byte(&mut self) -> Result<u8, Error> {
        if self.pos >= self.buf.len() {
            Err(Error::UnexpectedEOF)
        } else {
            let b = self.buf[self.pos];
            self.pos += 1;
            Ok(b)
        }
    }

    pub(crate) fn read_bool(&mut self) -> Result<bool, Error> {
        match self.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::IrregularData),
        }
    }

    #[allow(unused)]
    pub(crate) fn skip(&mut self, n: usize) -> Result<(), Error> {
        if self.buf.len() - self.pos < n {
            Err(Error::UnexpectedEOF)
        } else {
            self.pos += n;
            Ok(())
        }
    }

    #[allow(unused)]
    pub(crate) fn backup(&mut self, n: usize) {
        assert!(self.pos >= n);
        self.pos -= n;
    }

    pub(crate) fn read_u16(&mut self) -> Result<u16, Error> {
        Ok(LittleEndian::read_u16(self.next_bytes(2)?))
    }

    pub(crate) fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(LittleEndian::read_u32(self.next_bytes(4)?))
    }

    pub(crate) fn read_u64(&mut self) -> Result<u64, Error> {
        Ok(LittleEndian::read_u64(self.next_bytes(8)?))
    }

    pub(crate) fn read_varuint(&mut self) -> Result<u64, Error> {
        match self.read_byte()? {
            0xFD => self.read_u16().map(|v| (3, v as u64)),
            0xFE => self.read_u32().map(|v| (5, v as u64)),
            0xFF => self.read_u64().map(|v| (9, v)),
            val => Ok((1, val as u64)),
        }
        .and_then(|(len, val)| match len == varuint_encode_size(val) {
            true => Ok(val),
            false => Err(Error::IrregularData),
        })
    }
}
