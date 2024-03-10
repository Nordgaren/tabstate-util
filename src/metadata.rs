use bytemuck::{AnyBitPattern, Zeroable};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Encoding {
    ANSI = 1,
    UTF16LE = 2,
    UTF16BE = 3,
    UTF8BOM = 4,
    UTF8 = 5,
}
unsafe impl Zeroable for Encoding {}
unsafe impl AnyBitPattern for Encoding {}
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CarriageType {
    Unix = 1,
    CRLF = 3,
}

unsafe impl Zeroable for CarriageType {}

unsafe impl AnyBitPattern for CarriageType {}
/// I am pretty sure there is a metadata structure of this size
#[repr(C)]
#[derive(Debug, Copy, Clone, AnyBitPattern)]
pub struct TabStateMetadata {
    pub encoding: Encoding,
    pub return_carriage: CarriageType,
    pub filetime_as_varint: [u8; 0x9],
    pub content_hash: [u8; 0x20],
    pub unk: [u8; 0x1],
}
pub const METADATA_STRUCTURE_SIZE: usize = 0x2C;

const _: () = assert!(std::mem::size_of::<TabStateMetadata>() == METADATA_STRUCTURE_SIZE);
