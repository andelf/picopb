#![no_std]

use core::str;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
/// A wire type that provides just enough information to find the length of the following value.
pub enum WireType {
    /// int32, int64, uint32, uint64, sint32, sint64, bool, enum
    Varint = 0,
    /// fixed64, sfixed64, double
    Bit64 = 1,
    /// string, bytes, embedded messages, packed repeated fields
    Bytes = 2,
    /// fixed32, sfixed32, float
    Bit32 = 5,
}

impl WireType {
    pub fn from_u8(b: u8) -> Option<Self> {
        match b {
            0 => Some(WireType::Varint),
            1 => Some(WireType::Bit64),
            2 => Some(WireType::Bytes),
            5 => Some(WireType::Bit32),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Eof,
    UnexpectedEof,
    InvalidWireType(u8),
    VarintOverflow,
    InvalidUtf8String,
}

pub struct PbReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl PbReader<'_> {
    pub fn new<'a>(buf: &'a [u8]) -> PbReader<'a> {
        PbReader { buf, pos: 0 }
    }

    pub fn peek_next_key(&self) -> Result<(u8, WireType), Error> {
        let key = self.peek_next_u8().ok_or(Error::Eof)?;
        match WireType::from_u8(key & 0b111) {
            Some(wt) => Ok((key >> 3, wt)),
            None => Err(Error::InvalidWireType(key & 0b111)),
        }
    }

    /// (field_number, wire_type)
    pub fn next_key(&mut self) -> Result<(u8, WireType), Error> {
        let key = self.next_u8().ok_or(Error::Eof)?;
        match WireType::from_u8(key & 0b111) {
            Some(wt) => Ok((key >> 3, wt)),
            None => Err(Error::InvalidWireType(key & 0b111)),
        }
    }

    pub fn next_varint(&mut self) -> Result<u64, Error> {
        let mut result = 0;
        let mut bitpos = 0;
        loop {
            let b = self.next_u8().ok_or(Error::UnexpectedEof)?;
            let tmp = ((b & 0x7f) as u64)
                .checked_shl(bitpos)
                .ok_or(Error::VarintOverflow)?;
            result += tmp;
            if b & 0x80 == 0 {
                return Ok(result);
            }
            bitpos += 7;
        }
    }

    pub fn next_bytes(&mut self) -> Result<&[u8], Error> {
        let len = self.next_varint()? as usize;
        if self.pos + len > self.buf.len() {
            Err(Error::UnexpectedEof)
        } else {
            let bytes = &self.buf[self.pos..self.pos + len];
            self.pos += len;
            Ok(bytes)
        }
    }

    pub fn next_string(&mut self) -> Result<&str, Error> {
        self.next_bytes()
            .and_then(|raw| str::from_utf8(raw).map_err(|_| Error::InvalidUtf8String))
    }

    pub fn next_embedded_message(&mut self) -> Result<PbReader<'_>, Error> {
        self.next_bytes().map(PbReader::new)
    }

    pub fn next_svarint(&mut self) -> Result<i64, Error> {
        let val = self.next_varint()?;
        Ok(varint_to_svarint(val))
    }

    fn peek_next_u8(&self) -> Option<u8> {
        if self.has_next() {
            Some(self.buf[self.pos])
        } else {
            None
        }
    }

    fn next_u8(&mut self) -> Option<u8> {
        if self.has_next() {
            let i = self.pos;
            self.pos += 1;
            Some(self.buf[i])
        } else {
            None
        }
    }

    fn has_next(&self) -> bool {
        self.pos < self.buf.len()
    }

    pub fn is_eof(&self) -> bool {
        self.pos == self.buf.len()
    }
}

#[inline]
fn varint_to_svarint(n: u64) -> i64 {
    if n & 0b1 == 1 {
        -((n >> 1) as i64 + 1)
    } else {
        (n >> 1) as i64
    }
}
