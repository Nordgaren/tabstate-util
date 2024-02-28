#[allow(unused)]
pub const FOOTER_SIZE: usize = 0x5;
pub const FILE_STATE_SAVED: u8 = 1;
pub const FILE_STATE_UNSAVED: u8 = 0;
pub const FIRST_MARKER_BYTE: [u8; 1] = [0x05];
pub const FIRST_MARKER_VARIANTS: [u8; 2] = [0x01, 0x03];
#[allow(unused)]
pub const METADATA_STRUCTURE_SIZE: usize = 0x2B;
pub const SECOND_MARKER_BYTES: [u8; 2] = [0x00, 0x01];
pub const SIZE_END_MARKER: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
pub const MAX_VAL: u8 = 0x7F;
pub const SIGN_BIT: u8 = 0x80;
pub const SIZE_START_MARKER: [u8; 1] = [0x01];
pub const UNSUPPORTED_MESSAGE: &str = "Buffer file has unknown size. The TabState buffer doesn't get the size of the buffer until Notepad has been \"closed\". Currently unsupported";
