pub const FOOTER_SIZE: usize = 0x5;
pub const FILE_STATE_SAVED: u8 = 1;
pub const FIRST_MARKER_BYTES: [u8; 2] = [0x05, 0x01];
pub const SIZE_OF_METADATA_STRUCTURE: usize = 0x2B;
pub const SECOND_MARKER_BYTES: [u8; 2] = [0x00, 0x01];
pub const THIRD_MARKER_BYTES: [u8; 4] = [0x01, 0x00, 0x00, 0x00];
pub const MAX_VAL: usize = 0x7F;
pub const UNSUPPORTED_MESSAGE: &str = "Unsaved files are currently not supported by this reader.";
