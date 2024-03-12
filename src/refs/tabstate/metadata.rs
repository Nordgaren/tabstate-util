use crate::consts::{CARRIAGE_TYPES, ENCODINGS};
use crate::enums::{CarriageType, Encoding};
use crate::refs::tabstate::buffer::TabStateBufferRef;
use crate::refs::varint::VarIntRef;
use buffer_reader::BufferReader;
use std::io::{Error, ErrorKind};
use widestring::WideStr;

#[derive(Copy, Clone)]
pub struct TabStateMetadata<'a> {
    /// File path as a wide string.
    file_path: TabStateBufferRef<'a>,
    /// The full size in chars of the text buffer on disk. This includes carriage returns, which are
    /// not always represented in the TabState text buffer.
    full_buffer_size: VarIntRef<'a>,
    pub encoding: &'a Encoding,
    pub carriage_type: &'a CarriageType,
    pub filetime: VarIntRef<'a>,
    pub content_hash: &'a [u8; 0x20],
    pub unk: &'a u8,
    pub unk2: &'a u8,
}

impl<'a> TabStateMetadata<'a> {
    pub fn new(
        file_path: TabStateBufferRef<'a>,
        full_buffer_size: VarIntRef<'a>,
        encoding: &'a Encoding,
        carriage_type: &'a CarriageType,
        filetime: VarIntRef<'a>,
        content_hash: &'a [u8; 0x20],
        unk: &'a u8,
        unk2: &'a u8,
    ) -> Self {
        Self {
            file_path,
            full_buffer_size,
            encoding,
            carriage_type,
            filetime,
            content_hash,
            unk,
            unk2,
        }
    }
    pub fn from_reader(br: &mut BufferReader<'a>) -> std::io::Result<Self> {
        // Get the file path.
        let file_path = TabStateBufferRef::from_reader(br)?;

        // Size of the buffer in the saved file with character size adjustments (carriage return, etc)
        let full_buffer_size = VarIntRef::from_reader(br)?;

        // The metadata structure starts with the encoding
        let encoding = br.read_t::<Encoding>()?;
        if !ENCODINGS.contains(&encoding.as_value()) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown encoding Expected one of: {ENCODINGS:?}. Got: {:X}",
                    encoding
                ),
            ));
        }
        // Then the return carriage type
        let return_carriage = br.read_t::<CarriageType>()?;
        if !CARRIAGE_TYPES.contains(&return_carriage.as_value()) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown file variant. Expected one of: {CARRIAGE_TYPES:?}. Got: {:X}",
                    return_carriage
                ),
            ));
        }

        let filetime = VarIntRef::from_reader(br)?;
        let content_hash = br.read_t()?;
        let unk = br.read_t()?;
        let unk2 = br.read_t()?;

        Ok(Self::new(
            file_path,
            full_buffer_size,
            encoding,
            return_carriage,
            filetime,
            content_hash,
            unk,
            unk2,
        ))
    }
    /// Get a reference to the file path len VarInt that represents the size in chars of the text file
    /// path
    pub fn get_file_path_len(&'a self) -> VarIntRef<'a> {
        self.file_path.get_buffer_len()
    }
    /// Get a reference to the path of the file this TabState represents. Unsaved files do not have
    /// a path.
    pub fn get_path(&self) -> &'a WideStr {
        self.file_path.get_buffer()
    }
    /// Get a reference to the full buffer size VarInt that represents the size in charsof the text
    /// file on disk, if available.
    pub fn get_full_buffer_size(&'a self) -> VarIntRef<'a> {
        self.full_buffer_size
    }
    pub fn get_encoding(&'a self) -> &'a Encoding {
        self.encoding
    }
    pub fn get_carriage_type(&'a self) -> &'a CarriageType {
        self.carriage_type
    }
    pub fn get_filetime(&'a self) -> VarIntRef<'a> {
        self.filetime
    }
    pub fn get_content_hash(&'a self) -> &'a [u8; 0x20] {
        self.content_hash
    }
    pub fn get_unk(&'a self) -> &'a u8 {
        self.unk
    }
    pub fn get_unk2(&'a self) -> &'a u8 {
        self.unk2
    }
}
