#![doc = "TabState references to each part of a TabState file. This is generic, so some parts are optional"]

use crate::consts::{
    FILE_STATE_SAVED, FILE_STATE_UNSAVED, CURSOR_START_MARKER,
    CURSOR_END_MARKER,
};
use crate::footer::TabStateFooter;
use crate::header::Header;
use crate::refs::tabstate::cursor::TabStateCursor;
use crate::refs::tabstate::saved::SavedStateRefs;
use crate::refs::varint::VarIntRef;
use crate::util;
use buffer_reader::BufferReader;
use std::io::{Error, ErrorKind};
use widestring::WideStr;

pub mod cursor;
pub mod saved;

/// A structure tht holds references to the data in a Notepad buffer.
#[allow(unused)]
pub struct TabStateRefs<'a> {
    header: &'a Header,
    saved_refs: Option<SavedStateRefs<'a>>,
    cursor: TabStateCursor<'a>,
    buffer_size: VarIntRef<'a>,
    text_buffer: &'a WideStr,
    footer: &'a TabStateFooter,
}

impl<'a> TabStateRefs<'a> {
    /// Returns a new `TabStateRefs` object containing the provided refs.
    pub fn new(
        header: &'a Header,
        saved_refs: Option<SavedStateRefs<'a>>,
        cursor: TabStateCursor<'a>,
        buffer_size: VarIntRef<'a>,
        text_buffer: &'a WideStr,
        footer: &'a TabStateFooter,
    ) -> TabStateRefs<'a> {
        Self {
            header,
            saved_refs,
            cursor,
            buffer_size,
            text_buffer,
            footer,
        }
    }
    pub fn get_saved_tabstate_refs(&self) -> Option<SavedStateRefs> {
        self.saved_refs
    }
    /// Get a reference to the cursor start VarInt.
    pub fn get_cursor_start(&'a self) -> VarIntRef<'a> {
        self.cursor.get_cursor_start()
    }
    /// Get a reference to the cursor end VarInt.
    pub fn get_cursor_end(&'a self) -> VarIntRef<'a> {
        self.cursor.get_cursor_end()
    }
    /// Get a reference to the main text buffer size for the TabState.
    pub fn get_buffer_size(&'a self) -> VarIntRef<'a> {
        self.buffer_size
    }
    /// Get a reference to the main text buffer for the TabState.
    pub fn get_buffer(&self) -> &'a WideStr {
        self.text_buffer
    }
    /// Get a reference to the footer for the file.
    pub fn get_footer(&self) -> &'a TabStateFooter {
        self.footer
    }
    pub fn from_buffer(buffer: &'a [u8]) -> std::io::Result<Self> {
        let br = BufferReader::new(buffer);

        let header = br.read_t::<Header>()?;

        // I know that the magic is technically just NP, and that the third byte can change, but if
        // it isn't I am going to return an error, anyway, so let's just check it here.
        if &header.magic != b"NP\0" {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Magic bytes invalid. Should be \"NP\" and a null byte. Read: \"{}\" raw: {:?}",
                    unsafe { std::str::from_utf8_unchecked(&header.magic) },
                    header.magic
                ),
            ));
        }

        // We have to match as u8s, otherwise the compiler thinks the final case is unreachable, which
        // is not true in this case, and the code will be optimized out.
        let saved_refs = match header.state as u8 {
            FILE_STATE_SAVED => Some(SavedStateRefs::from_reader(&br)?),
            FILE_STATE_UNSAVED => None,
            file_state => return Err(Error::new(
                ErrorKind::Unsupported,
                format!(
                    "File state should be 1 or 0. There are likely {} bytes left in the buffer Remaining: {}",
                    // When the file state is not 1 or 0 it indicates how many bytes are left in the
                    // file
                    file_state as usize + std::mem::size_of::<TabStateFooter>(),
                    // This includes the header, but also includes the file state byte, so we add one
                    // to the length of the remaining bytes
                    br.len() + 1
                ),
            )),
        };

        // Read the second marker, and make sure it's what's expected.
        let start_marker = br.read_byte()?;
        if start_marker != CURSOR_START_MARKER {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown marker encountered. Expected: {CURSOR_START_MARKER:02X?} Got: {:02X?}.",
                    start_marker
                ),
            ));
        }

        // After the first marker should be two more VarInt. These represent the cursor start and end
        // point for selection. They will be equal if there is no selection.
        let cursor_start = VarIntRef::from_reader(&br)?;
        let cursor_end = VarIntRef::from_reader(&br)?;

        // Read the third marker, which denotes the end of the two cursor start points.
        let end_marker = br.read_bytes(CURSOR_END_MARKER.len())?;
        if end_marker != CURSOR_END_MARKER {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Could not find marker bytes: {CURSOR_END_MARKER:02X?}"),
            ));
        };

        // Get the VarInt for the text buffer size in UTF-16.
        let buffer_size = VarIntRef::from_reader(&br)?;
        let decoded_size = buffer_size.decode();

        // The text buffer should be right after the VarInt we just read. Double the size of bytes
        // to read, since the size is in UTF-16 chars
        let text_buffer = br.read_bytes(decoded_size * 2)?;
        let text_buffer = util::wide_string_from_buffer(text_buffer, decoded_size);

        // It always ends with this footer. I am not sure if it's there if there's extra data, as there
        // sometimes is extra data. It might still be after the text buffer AND at the end of the file.
        let footer = br.read_t()?;

        // Check that there are no bytes remaining in the buffer. If there are, print out the bytes
        // and how many.
        if !br.is_empty() {
            eprintln!(
                "Please report on GH issues: Bytes still remaining in the buffer: {}\n",
                br.len(),
            );
            eprintln!("Please send me your buffer file, as well, so I can see what is wrong!")
        }

        Ok(TabStateRefs::new(
            header,
            saved_refs,
            TabStateCursor::new(cursor_start, cursor_end),
            buffer_size,
            text_buffer,
            footer,
        ))
    }
}
