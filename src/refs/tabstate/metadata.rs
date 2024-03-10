use buffer_reader::BufferReader;
use crate::enums::{CarriageType, Encoding};
use crate::refs::varint::VarIntRef;

#[derive(Copy, Clone)]
pub struct TabStateMetadata<'a> {
    pub encoding: &'a Encoding,
    pub return_carriage: &'a CarriageType,
    pub filetime: VarIntRef<'a>,
    pub content_hash: &'a [u8; 0x20],
    pub unk: &'a u8,
}

impl<'a> TabStateMetadata<'a> {
    pub fn new(
        encoding: &'a Encoding,
        return_carriage: &'a CarriageType,
        filetime: VarIntRef<'a>,
        content_hash: &'a [u8; 0x20],
        unk: &'a u8,
    ) -> Self {
        Self {
            encoding,
            return_carriage,
            filetime,
            content_hash,
            unk,
        }
    }
    pub fn from_reader(br: &BufferReader<'a>) -> std::io::Result<Self> {
        let encoding = br.read_t()?;
        let return_carriage = br.read_t()?;
        let filetime = VarIntRef::from_reader(br)?;
        let content_hash= br.read_t()?;
        let unk= br.read_t()?;

        Ok(Self::new(encoding, return_carriage, filetime, content_hash, unk))
    }
}