use crate::header::State;

pub const FILE_STATE_SAVED: u8 = State::Saved as u8;
pub const FILE_STATE_UNSAVED: u8 = State::Unsaved as u8;
pub const ENCODINGS: [u8; 5] = [0x01, 0x02, 0x03, 0x04, 0x05];
pub const CARRIAGE_TYPES: [u8; 2] = [0x01, 0x03];
pub const CURSOR_END_MARKER: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
pub const MAX_VAL: u8 = 0x7F;
pub const SIGN_BIT: u8 = 0x80;
pub const CURSOR_START_MARKER: u8 = 0x01;
