use crate::consts::METADATA_STRUCTURE_SIZE;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Encoding {
    ANSI = 1,
    UTF16LE = 2,
    UTF16BE = 3,
    UTF8BOM = 4,
    UTF8 = 5,
}
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CarriageType {
    Unix = 1,
    CRLF = 3,
}
/// I am pretty sure there is a metadata structure of this size
#[repr(C)]
pub struct TabStateMetadata {
    pub encoding: Encoding,
    pub return_carriage: CarriageType,
    pub unk: [u8; 0x6],
    pub unk_two: [u8; 0x23],
}
const _: () = assert!(std::mem::size_of::<TabStateMetadata>() == METADATA_STRUCTURE_SIZE);
