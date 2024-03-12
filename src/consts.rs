use crate::enums::{CarriageType, Encoding};
use crate::header::State;

pub const FILE_STATE_SAVED: u8 = State::Saved as u8;
pub const FILE_STATE_UNSAVED: u8 = State::Unsaved as u8;
pub const ENCODINGS: [u8; 5] = [
    Encoding::ANSI as u8,
    Encoding::UTF16LE as u8,
    Encoding::UTF16BE as u8,
    Encoding::UTF8BOM as u8,
    Encoding::UTF8 as u8,
];
pub const CARRIAGE_TYPES: [u8; 2] = [CarriageType::Unix as u8, CarriageType::CRLF as u8];
pub const CURSOR_START_MARKER: u8 = 0x01;
pub const CURSOR_END_MARKER: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
pub const MAX_VAL: u8 = 0x7F;
pub const SIGN_BIT: u8 = 0x80;
