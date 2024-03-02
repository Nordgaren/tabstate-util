use std::io::{Error, ErrorKind};
use buffer_reader::BufferReader;
use crate::metadata::TabStateMetadata;
use crate::refs::varint::VarIntRef;
use widestring::WideStr;
use crate::consts::{CARRIAGE_TYPES, ENCODINGS};
use crate::util;

/// Represents items that are only available in TabStates that represent a file on disk.
#[derive(Copy, Clone)]
pub struct SavedRefs<'a> {
    file_path_len: VarIntRef<'a>,
    file_path: &'a WideStr,
    full_buffer_size: VarIntRef<'a>,
    metadata: &'a TabStateMetadata,
}

impl<'a> SavedRefs<'a> {
    pub fn new(
        file_path_len: VarIntRef<'a>,
        file_path: &'a WideStr,
        full_buffer_size: VarIntRef<'a>,
        metadata: &'a TabStateMetadata,
    ) -> Self {
        Self {
            file_path_len,
            file_path,
            full_buffer_size,
            metadata,
        }
    }
    pub fn from_reader(br: &BufferReader<'a>) -> std::io::Result<Self> {
        // Get the file path.
        let file_path_len = VarIntRef::from_reader(&br)?;
        let decoded_size = file_path_len.decode();
        let str_bytes = br.read_bytes(decoded_size * 2)?;
        let file_path = util::wide_string_from_buffer(str_bytes, decoded_size);

        let full_buffer_size = VarIntRef::from_reader(&br)?;

        // Get the main metadata object
        let metadata = br.read_t::<TabStateMetadata>()?;

        // The metadata structure starts with the encoding
        let encoding = metadata.encoding as u8;
        if !ENCODINGS.contains(&encoding) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown encoding Expected one of: {ENCODINGS:?}. Got: {:X}",
                    encoding
                ),
            ));
        }
        // Then the return carriage type
        let return_carriage = metadata.return_carriage as u8;
        if !CARRIAGE_TYPES.contains(&return_carriage) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown file variant. Expected one of: {CARRIAGE_TYPES:?}. Got: {:X}",
                    return_carriage
                ),
            ));
        }

        Ok(SavedRefs::new(file_path_len, file_path, full_buffer_size, metadata))
    }
    /// Get a reference to the file path len VarInt that represents the size in chars of the text file
    /// path
    pub fn get_file_path_len(&'a self) -> VarIntRef<'a> {
        self.file_path_len
    }
    /// Get a reference to the path of the file this TabState represents. Unsaved files do not have
    /// a path.
    pub fn get_path(&self) -> &'a WideStr {
        self.file_path
    }
    /// Get a reference to the full buffer size VarInt that represents the size in charsof the text
    /// file on disk, if available.
    pub fn get_full_buffer_size(&'a self) -> VarIntRef<'a> {
        self.full_buffer_size
    }
    /// Get a reference to the metadata structure, if available.
    pub fn get_metadata(&self) -> &'a TabStateMetadata {
        self.metadata
    }
}
