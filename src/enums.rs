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
