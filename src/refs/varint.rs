use crate::consts::SIGN_BIT;
use crate::varint::VarInt;
use buffer_reader::BufferReader;
use std::io::{Error, ErrorKind};

/// A reference to a slice of bytes that represent a variable sized integer.
#[derive(Copy, Clone, PartialEq)]
pub struct VarIntRef<'a> {
    buffer: &'a [u8],
}

impl<'a> VarIntRef<'a> {
    /// Assumes the buffer is only the varint bytes. Does no checking, at the moment.
    pub fn new(buffer: &'a [u8]) -> std::io::Result<Self> {
        if buffer.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "size buffer cannot be of length 0",
            ));
        }

        Ok(VarIntRef { buffer })
    }
    pub(crate) unsafe fn new_unchecked(buffer: &'a [u8]) -> Self {
        VarIntRef { buffer }
    }
    /// Assumes the reader is at the start of a varint. Reads the sign bit of each byte and advances
    /// until the end of the varint and passes back a reference to the bytes as a `VarIntRef`
    pub fn from_reader(br: &BufferReader<'a>) -> std::io::Result<Self> {
        // Get the bytes that represent the size of the text buffer and decode the size.
        let mut count = 0;

        loop {
            let byte = br.peek_byte(count)?;
            count += 1;

            if byte & SIGN_BIT == 0 {
                break;
            }
        }
        VarIntRef::new(br.read_bytes(count)?)
    }
    /// Decodes a varint from the provided bytes
    pub fn decode(&self) -> usize {
        crate::varint::decode(self.buffer) as usize
    }
    /// Decodes a varint from the provided bytes. Uses the largest int primitive available.
    pub fn decode_lossless(&self) -> u128 {
        crate::varint::decode(self.buffer)
    }
    pub fn get_buffer(&self) -> &[u8] {
        self.buffer
    }
    pub fn size_of(&self) -> usize {
        self.buffer.len()
    }
    pub fn to_owned(&self) -> VarInt {
        unsafe { VarInt::from_buffer_unchecked(self.buffer) }
    }
}
