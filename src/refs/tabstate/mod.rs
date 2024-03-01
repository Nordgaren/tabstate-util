#![doc = "TabState references to each part of a TabState file. This is generic, so some parts are optional"]

use std::io::{Error, ErrorKind};
use buffer_reader::BufferReader;
use widestring::WideStr;
use crate::consts::{FILE_STATE_SAVED, FILE_STATE_UNSAVED, ENCODINGS, CARRIAGE_TYPES, SECOND_MARKER_BYTES, SIZE_END_MARKER};
use crate::footer::TabStateFooter;
use crate::metadata::TabStateMetadata;
use crate::refs::varint::VarIntRef;
use crate::util;

pub mod unsaved;

/// A structure tht holds references to the data in a Notepad buffer.
#[allow(unused)]
pub struct TabStateRefs<'a> {
    file_path: Option<&'a WideStr>,
    full_buffer_size: Option<VarIntRef<'a>>,
    some_metadata: Option<&'a TabStateMetadata>,
    cursor_start: VarIntRef<'a>,
    cursor_end: VarIntRef<'a>,
    buffer_size: VarIntRef<'a>,
    text_buffer: &'a WideStr,
    footer: &'a TabStateFooter,
}

impl<'a> TabStateRefs<'a> {
    /// Returns a new `TabStateRefs` object containing the provided refs.
    pub fn new(
        file_path: Option<&'a WideStr>,
        full_buffer_size: Option<VarIntRef<'a>>,
        some_metadata: Option<&'a TabStateMetadata>,
        cursor_start: VarIntRef<'a>,
        cursor_end: VarIntRef<'a>,
        buffer_size: VarIntRef<'a>,
        text_buffer: &'a WideStr,
        footer: &'a TabStateFooter,
    ) -> TabStateRefs<'a> {
        Self {
            file_path,
            full_buffer_size,
            some_metadata,
            cursor_start,
            cursor_end,
            buffer_size,
            text_buffer,
            footer,
        }
    }
    /// Returns the path of the file this TabState represents. Unsaved files do not have a path.
    pub fn get_path(&self) -> Option<&'a WideStr> {
        self.file_path
    }
    /// Get the main text buffer for the file.
    pub fn get_buffer(&self) -> &'a WideStr {
        self.text_buffer
    }
    pub fn from_buffer(buffer: &'a [u8]) -> std::io::Result<Self> {
        let br = BufferReader::new(buffer);

        let magic = br.read_bytes(3)?;

        // I know that the magic is technically just NP, and that the third byte can change, but if
        // it isn't I am going to return an error, anyway, so let's just check it here.
        if magic != b"NP\0" {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Magic bytes invalid. Should be \"NP\" and a null byte. Read: \"{}\" raw: {magic:?}",
                    unsafe { std::str::from_utf8_unchecked(magic) }
                ),
            ));
        }

        let file_state = br.read_byte()?;
        if file_state == FILE_STATE_SAVED {
            return Self::read_saved_buffer(br);
        } else if file_state == FILE_STATE_UNSAVED {
            return Self::read_unsaved_buffer(br);
        }

        Err(Error::new(
            ErrorKind::Unsupported,
            format!(
                "File has no data. File state should be 1 or 0. There are likely \
            this many bytes left in the buffer {file_state} + 5 bytes for the footer {}", br.len() + 1
            ),
        ))
    }
    /// Reads a Notepad tab buffer that is saved to disk, and has a filepath and the text buffer.
    pub fn read_saved_buffer(br: BufferReader<'a>) -> std::io::Result<Self> {
        // Get the file path.
        let path_len = br.read_byte()? as usize;
        let str_bytes = br.read_bytes(path_len * 2)?;
        let file_path = Some(util::wide_string_from_buffer(str_bytes, path_len));


        let full_buffer_size = VarIntRef::from_reader(&br)?;

        // Get the main metadata object (?)
        let some_metadata = br.read_t::<TabStateMetadata>()?;

        // The metadata structure starts with the encoding and return carraige type
        let encoding = some_metadata.encoding as u8;
        if !ENCODINGS.contains(&encoding) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown encoding Expected one of: {ENCODINGS:?}. Got: {:X}",
                    encoding
                ),
            ));
        }


        let return_carriage = some_metadata.return_carriage as u8;
        if !CARRIAGE_TYPES.contains(&return_carriage) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown file variant. Expected one of: {CARRIAGE_TYPES:?}. Got: {:X}",
                    return_carriage
                ),
            ));
        }

        // Read the second marker, and make sure it's what's expected.
        let marker_two = br.read_bytes(2)?;
        if marker_two != SECOND_MARKER_BYTES {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown marker encountered. Expected: {SECOND_MARKER_BYTES:02X?} Got: {:02X?}.",
                    marker_two
                ),
            ));
        }

        let cursor_start = VarIntRef::from_reader(&br)?;
        let cursor_end = VarIntRef::from_reader(&br)?;

        // Find the third marker, which denotes the end of the two unknown sizes. I have noticed these
        // sizes are sometimes the same as the buffer and sometimes not the same. They might also be
        // different sizes (as in bytes) than the main text buffer size. They can also both be 0 for
        // some reason.
        let marker_three = br.read_bytes(SIZE_END_MARKER.len())?;

        if marker_three != SIZE_END_MARKER {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Could not find marker bytes: {SIZE_END_MARKER:02X?}"),
            ));
        };

        // Get the VarInt from the reader so we can decode it.
        let buffer_size = VarIntRef::from_reader(&br)?;
        let decoded_size = buffer_size.decode()?;

        // The text buffer should be right after the final size buffer we just read.
        let text_buffer = br.read_bytes(decoded_size * 2)?;
        let text_buffer = util::wide_string_from_buffer(text_buffer, decoded_size);

        let footer = br.read_t()?;

        // Check that there are no bytes remaining in the buffer. If there are, print out the bytes
        // and how many.
        if !br.is_empty() {
            eprintln!(
                "Please report on GH issues: Bytes still remaining in the buffer:\n\
            remaining: {}\n\
            bytes: {:?}",
                br.len(),
                br.get_remaining()
            );
            eprintln!("Please send me your buffer file, as well, so I can see what is wrong!")
        }

        Ok(TabStateRefs::new(
            file_path,
            Some(full_buffer_size),
            Some(some_metadata),
            cursor_start,
            cursor_end,
            buffer_size,
            text_buffer,
            footer,
        ))
    }
}
