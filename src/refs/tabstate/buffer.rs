use crate::refs::varint::VarIntRef;
use buffer_reader::BufferReader;
use widestring::WideStr;
#[derive(Copy, Clone)]
pub struct TabStateBufferRef<'a> {
    buffer_len: VarIntRef<'a>,
    buffer: &'a WideStr,
}

impl<'a> TabStateBufferRef<'a> {
    pub fn new(buffer_len: VarIntRef<'a>, buffer: &'a WideStr) -> Self {
        TabStateBufferRef { buffer_len, buffer }
    }
    pub fn from_reader(br: &BufferReader<'a>) -> std::io::Result<Self> {
        // Length comes first
        let buffer_len = VarIntRef::from_reader(br)?;
        let decoded_size = buffer_len.decode();
        // Then we read the bytes and convert it to a `WideStr`
        let str_bytes = br.read_slice_t(decoded_size)?;
        let buffer = WideStr::from_slice(str_bytes);
        Ok(Self::new(buffer_len, buffer))
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
