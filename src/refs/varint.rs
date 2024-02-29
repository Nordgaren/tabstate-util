use std::io::{Error, ErrorKind};
use buffer_reader::BufferReader;
use crate::consts::{MAX_VAL, SIGN_BIT};

pub struct VarIntRef<'a> {
    buffer: &'a [u8],
}

impl<'a> VarIntRef<'a> {
    /// Assumes the buffer is only the varint bytes. Does no checking, at the moment.
    pub fn new(buffer: &'a [u8]) -> Self {
        VarIntRef { buffer }
    }
    /// Assumes the reader is at the start of a varint. Reads the sign bit of each byte and advances
    /// until the end of the varint and passes back a reference to the bytes as a `VarIntRef`
    pub fn from_reader(br: &BufferReader<'a>) -> std::io::Result<VarIntRef<'a>> {
        // Get the bytes that represent the size of the text buffer and decode the size.
        let mut count = 0;

        loop {
            let byte = br.peek_byte(count)?;
            count += 1;

            if byte & SIGN_BIT == 0 {
                break;
            }
        }

        Ok(VarIntRef::new(br.read_bytes(count)?))
    }
    /// Decodes a varint from the provided bytes
    pub fn decode(&self) -> std::io::Result<usize> {
        let size_buffer =self.buffer;
        if size_buffer.is_empty() {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "size buffer cannot be of length 0",
            ));
        }

        let mut size = 0;
        // We strip the sign bit off and bit shift the value to the right by 7 * i (since each byte only holds
        // 7 bits of data and this is little endian, so the byte furthest to the left is the least significant byte.)
        for i in 0..size_buffer.len() {
            let num = (size_buffer[i] & MAX_VAL) as usize;
            size |= num << (7 * i);
        }

        Ok(size)
    }
}