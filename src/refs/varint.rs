use crate::consts::SIGN_BIT;
use crate::varint::VarInt;
use buffer_reader::BufferReader;

/// A reference to a slice of bytes that represent a variable sized integer.
#[derive(Copy, Clone, PartialEq)]
pub struct VarIntRef<'a> {
    buffer: &'a [u8],
}

impl<'a> VarIntRef<'a> {
    /// Assumes the provided buffer is only the varint bytes. Returns an error if the provided buffer
    /// is invalid, which includes empty buffer, leading bytes not being signed, or last byte being
    /// signed
    pub fn new(buffer: &'a [u8]) -> std::io::Result<Self> {
        Ok(Self { buffer: crate::varint::validate_buffer(buffer)? })
    }
    /// Assumes the provided buffer is only the varint bytes.
    ///
    /// # Safety
    ///
    /// Does not check that the buffer is valid.
    #[inline(always)]
    pub unsafe fn new_unchecked(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }
    /// Assumes the reader is at the start of a varint. Reads the sign bit of each byte and advances
    /// until the end of the varint and passes back a reference to the bytes as a `VarIntRef`
    pub fn from_reader(br: &mut BufferReader<'a>) -> std::io::Result<Self> {
        // Get the bytes that represent the size of the text buffer and decode the size. Buffer must
        // be at least size 1, so we start the count at 1, and only increment if the sign bit is set.
        let mut count = 0;

        loop {
            let byte = br.peek_byte(count)?;
            count += 1;

            if byte & SIGN_BIT == 0 {
                break;
            }
        }

        Ok(Self { buffer: br.read_bytes(count)? })
    }
    /// Decodes a varint from the provided bytes
    #[inline(always)]
    pub fn decode(&self) -> usize {
        crate::varint::decode(self.buffer) as usize
    }
    /// Decodes a varint from the provided bytes. Uses the largest int primitive available.
    #[inline(always)]
    pub fn decode_lossless(&self) -> u128 {
        crate::varint::decode(self.buffer)
    }
    #[inline(always)]
    pub fn get_buffer(&self) -> &[u8] {
        self.buffer
    }
    #[inline(always)]
    pub fn size_of(&self) -> usize {
        self.buffer.len()
    }
    #[inline(always)]
    pub fn to_owned(&self) -> VarInt {
        unsafe { VarInt::from_buffer_unchecked(self.buffer) }
    }
}
