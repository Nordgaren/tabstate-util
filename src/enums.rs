use bytemuck::{AnyBitPattern, Zeroable};
use std::fmt::{Display, Formatter, UpperHex};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Encoding {
    ANSI = 1,
    UTF16LE = 2,
    UTF16BE = 3,
    UTF8BOM = 4,
    UTF8 = 5,
}
impl Encoding {
    pub fn as_value(&self) -> u8 {
        *self as u8
    }
}
impl Display for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_value())
    }
}
impl UpperHex for Encoding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_value())
    }
}
unsafe impl Zeroable for Encoding {}
unsafe impl AnyBitPattern for Encoding {}
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CarriageType {
    Unix = 1,
    CRLF = 3,
}
impl CarriageType {
    pub fn as_value(&self) -> u8 {
        *self as u8
    }
}
impl Display for CarriageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_value())
    }
}
impl UpperHex for CarriageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_value())
    }
}

unsafe impl Zeroable for CarriageType {}

unsafe impl AnyBitPattern for CarriageType {}
