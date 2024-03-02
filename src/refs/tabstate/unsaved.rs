use crate::consts::{SIZE_END_MARKER, UNSAVED_SIZE_START_MARKER};
use crate::header::Header;
use crate::refs::tabstate::{TabStateCursor, TabStateRefs};
use crate::refs::varint::VarIntRef;
use crate::util;
use buffer_reader::BufferReader;
use std::io::{Error, ErrorKind};

impl<'a> TabStateRefs<'a> {
    /// Reads a Notepad tab buffer that is not saved to disk, and does not have a filepath. Currently
    /// unsupported if Notepad has not been closed since the tab was opened.
    pub(crate) fn read_unsaved_buffer(
        br: BufferReader<'a>,
        header: &'a Header,
    ) -> std::io::Result<TabStateRefs<'a>> {
        // Read the unsaved marker, and make sure it's what's expected.
        let marker = br.read_byte()?;
        if marker != UNSAVED_SIZE_START_MARKER {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown marker encountered. Expected: 0x{UNSAVED_SIZE_START_MARKER:02X} Got: 0x{marker:02X}.",
                ),
            ));
        }

        // After the first marker should be two more VarInt. These represent the cursor start and end
        // point for selection. They will be equal if there is no selection.
        let cursor_start = VarIntRef::from_reader(&br)?;
        let cursor_end = VarIntRef::from_reader(&br)?;

        // Read the third marker, which denotes the end of the two cursor start points.
        let marker_three = br.read_bytes(SIZE_END_MARKER.len())?;

        if marker_three != SIZE_END_MARKER {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Could not find marker bytes: {SIZE_END_MARKER:02X?}"),
            ));
        };

        // Get the VarInt for the text buffer size in UTF-16.
        let buffer_size = VarIntRef::from_reader(&br)?;
        let decoded_size = buffer_size.decode();
        if decoded_size == 0 {
            return Err(Error::new(ErrorKind::Unsupported, "Buffer file has unknown size. The TabState buffer doesn't get the size of the buffer until Notepad has been \"closed\". Currently unsupported"));
        }

        // The text buffer should be right after the VarInt we just read. Double the size of bytes
        // to read, since the size is in UTF-16 chars
        let text_buffer = br.read_bytes(decoded_size * 2)?;
        let text_buffer = util::wide_string_from_buffer(text_buffer, decoded_size);

        let footer = br.read_t()?;

        // Check that there are no bytes remaining in the buffer. If there are, print out the bytes
        // and how many.
        if !br.is_empty() {
            eprintln!(
                "Please report on GH issues: Bytes still remaining in the buffer: {}\n",
                br.len(),
            );
            println!("Please send me your buffer file, as well, so I can see what is wrong!")
        }

        Ok(TabStateRefs::new(
            header,
            None,
            TabStateCursor::new(cursor_start, cursor_end),
            buffer_size,
            text_buffer,
            footer,
        ))
    }
}
