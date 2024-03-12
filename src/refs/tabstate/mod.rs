#![doc = "TabState references to each part of a TabState file. This is generic, so some parts are optional"]

use crate::consts::{FILE_STATE_SAVED, FILE_STATE_UNSAVED, METADATA_UNK2};
use crate::footer::TabStateFooter;
use crate::header::Header;
use crate::options::TabStateOptions;
use crate::refs::tabstate::buffer::TabStateBufferRef;
use crate::refs::tabstate::cursor::TabStateCursor;
use crate::refs::tabstate::metadata::TabStateMetadata;
use crate::refs::varint::VarIntRef;
use buffer_reader::BufferReader;
use std::io::{Error, ErrorKind};
use widestring::WideStr;

pub mod buffer;
pub mod cursor;
pub mod metadata;

/// A structure tht holds references to the data in a Notepad buffer.
#[allow(unused)]
pub struct TabStateRefs<'a> {
    header: &'a Header,
    metadata: Option<TabStateMetadata<'a>>,
    cursor: TabStateCursor<'a>,
    options: &'a TabStateOptions,
    text_buffer: TabStateBufferRef<'a>,
    footer: &'a TabStateFooter,
}

impl<'a> TabStateRefs<'a> {
    /// Returns a new `TabStateRefs` object containing the provided refs.
    pub fn new(
        header: &'a Header,
        metadata: Option<TabStateMetadata<'a>>,
        cursor: TabStateCursor<'a>,
        options: &'a TabStateOptions,
        text_buffer: TabStateBufferRef<'a>,
        footer: &'a TabStateFooter,
    ) -> TabStateRefs<'a> {
        Self {
            header,
            metadata,
            cursor,
            options,
            text_buffer,
            footer,
        }
    }
    // Returns the `SavedStateRefs` for this object, if the buffer is in a saved state.
    pub fn get_metadata(&self) -> Option<TabStateMetadata> {
        self.metadata
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
    pub fn get_buffer_len(&'a self) -> VarIntRef<'a> {
        self.text_buffer.get_buffer_len()
    }
    /// Get a reference to the main text buffer for the TabState.
    pub fn get_buffer(&self) -> &'a WideStr {
        self.text_buffer.get_buffer()
    }
    /// Get a reference to the footer for the file.
    pub fn get_footer(&self) -> &'a TabStateFooter {
        self.footer
    }
    /// Parse the TabState file from a given buffer.
    pub fn from_buffer(buffer: &'a [u8]) -> std::io::Result<Self> {
        let mut br = BufferReader::new(buffer);

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
        let metadata = match header.state as u8 {
            FILE_STATE_SAVED => Some(TabStateMetadata::from_reader(&mut br)?),
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

        // Check that the second unk in `TabStateMetaData` is 1 or not. If it isn't, we need to inspect
        // this file.
        if let Some(mdata) = metadata {
            if *mdata.get_unk2() != METADATA_UNK2 {
                return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Unknown value encountered for TabStateMetadata::unk2. Expected: 1 Got: {:02X?}.\n\
                            Pleas inspect this file and help figure out the missing value means!",
                            *mdata.get_unk2()
                        ),
                    ));
            };
        };

        // After the first marker should be two more VarInt. These represent the cursor start and end
        // point for selection. They will be equal if there is no selection.
        let cursor_start = VarIntRef::from_reader(&mut br)?;
        let cursor_end = VarIntRef::from_reader(&mut br)?;

        let options = br.read_t()?;

        // This is the main text buffer in the TabState.
        let text_buffer = TabStateBufferRef::from_reader(&mut br)?;

        // It always ends with this footer. I am not sure if it's there if there's extra data, as there
        // sometimes is extra data. It might still be after the text buffer AND at the end of the file.
        let footer = br.read_t()?;

        // Check that there are no bytes remaining in the buffer. If there are, print out how many.
        if !br.is_empty() {
            eprintln!(
                "Please report on GH issues: Bytes still remaining in the buffer: {}",
                br.len(),
            );
            eprintln!("Please send me your buffer file, as well, so I can see what is wrong!")
        }

        Ok(TabStateRefs::new(
            header,
            metadata,
            TabStateCursor::new(cursor_start, cursor_end),
            options,
            text_buffer,
            footer,
        ))
    }
}
