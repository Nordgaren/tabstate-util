use crate::refs::varint::VarIntRef;
use widestring::WideStr;
#[derive(Copy, Clone)]
pub struct TextBufferRef<'a> {
    buffer_len: VarIntRef<'a>,
    buffer: &'a WideStr,
}

impl<'a> TextBufferRef<'a> {
    pub fn new(buffer_len: VarIntRef<'a>, buffer: &'a WideStr) -> Self {
        TextBufferRef { buffer_len, buffer }
    }
    pub fn decode_buffer_len(&self) -> usize {
        self.buffer_len.decode()
    }
    pub fn get_buffer_len(&self) -> VarIntRef<'a> {
        self.buffer_len
    }
    pub fn get_buffer(&self) -> &'a WideStr {
        self.buffer
    }
}
