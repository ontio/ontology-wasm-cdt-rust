use super::Decoder2;
use super::Error;
use byteorder::{ByteOrder, LittleEndian};

use crate::types::{Address, H256, U256};

use core::mem::transmute;

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

pub struct ZeroCopySource<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> ZeroCopySource<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { buf: data, pos: 0 }
    }

    pub(crate) fn next_bytes(&mut self, len: usize) -> Result<&'a [u8], Error> {
        if self.buf.len() - self.pos < len {
            Err(Error::UnexpectedEOF)
        } else {
            let bytes = &self.buf[self.pos..self.pos + len];
            self.pos += len;
            Ok(bytes)
        }
    }

    pub fn read_bytes(&mut self) -> Result<&'a [u8], Error> {
        let n = self.read_varuint()?;
        self.next_bytes(n as usize)
    }

    pub fn read<T: Decoder2<'a>>(&mut self) -> Result<T, Error> {
        T::decode2(self)
    }

    pub(crate) fn read_address(&mut self) -> Result<&'a Address, Error> {
        let buf = self.next_bytes(20)?;
        Ok(unsafe { transmute(buf.as_ptr()) })
    }

    pub(crate) fn read_h256(&mut self) -> Result<&'a H256, Error> {
        let buf = self.next_bytes(32)?;
        Ok(unsafe { transmute(buf.as_ptr()) })
    }

    pub fn read_u256(&mut self) -> Result<U256, Error> {
        let buf = self.next_bytes(32)?;
        Ok(U256::from_little_endian(buf))
    }

    pub(crate) fn read_into(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        let bytes = self.next_bytes(buf.len())?;
        buf.copy_from_slice(bytes);
        Ok(())
    }

    pub fn read_byte(&mut self) -> Result<u8, Error> {
        if self.pos >= self.buf.len() {
            Err(Error::UnexpectedEOF)
        } else {
            let b = self.buf[self.pos];
            self.pos += 1;
            Ok(b)
        }
    }

    pub fn read_bool(&mut self) -> Result<bool, Error> {
        match self.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::IrregularData),
        }
    }

    #[allow(unused)]
    pub fn skip(&mut self, n: usize) -> Result<(), Error> {
        if self.buf.len() - self.pos < n {
            Err(Error::UnexpectedEOF)
        } else {
            self.pos += n;
            Ok(())
        }
    }

    #[allow(unused)]
    pub fn backup(&mut self, n: usize) {
        assert!(self.pos >= n);
        self.pos -= n;
    }

    pub fn read_u16(&mut self) -> Result<u16, Error> {
        Ok(LittleEndian::read_u16(self.next_bytes(2)?))
    }

    pub fn read_u32(&mut self) -> Result<u32, Error> {
        Ok(LittleEndian::read_u32(self.next_bytes(4)?))
    }

    pub fn read_u64(&mut self) -> Result<u64, Error> {
        Ok(LittleEndian::read_u64(self.next_bytes(8)?))
    }

    pub fn read_u128(&mut self) -> Result<u128, Error> {
        Ok(LittleEndian::read_u128(self.next_bytes(16)?))
    }

    pub fn read_varuint(&mut self) -> Result<u64, Error> {
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
