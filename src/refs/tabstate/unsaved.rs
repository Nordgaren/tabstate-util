use crate::consts::{SIZE_END_MARKER, SIZE_START_MARKER};
use buffer_reader::BufferReader;
use std::io::{Error, ErrorKind};
use crate::refs::tabstate::{TabStateCursor, TabStateRefs};
use crate::refs::varint::VarIntRef;
use crate::util;

impl<'a> TabStateRefs<'a> {
    /// Reads a Notepad tab buffer that is not saved to disk, and does not have a filepath. Currently
    /// unsupported if Notepad has not been closed since the tab was opened.
    pub(crate) fn read_unsaved_buffer(br: BufferReader<'a>) -> std::io::Result<TabStateRefs<'a>> {
        // Read the unsaved marker, and make sure it's what's expected.
        let marker = br.read_bytes(1)?;
        if marker != SIZE_START_MARKER {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown marker encountered. Expected: {SIZE_START_MARKER:02X?} Got: {:02X?}.",
                    marker
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
        if decoded_size == 0 {
            return Err(Error::new(ErrorKind::Unsupported, "Buffer file has unknown size. The TabState buffer doesn't get the size of the buffer until Notepad has been \"closed\". Currently unsupported"));
        }

        // The text buffer should be right after the final size buffer we just read.
        let text_buffer = br.read_bytes(decoded_size * 2)?;
        let text_buffer = util::wide_string_from_buffer(text_buffer, decoded_size);

        let footer = br.read_t()?;

        // Check that there are no bytes remaining in the buffer. If there are, print out the bytes
        // and how many.
        if !br.is_empty() {
            println!(
                "Please report on GH issues: Bytes still remaining in the buffer:\n\
            remaining: {}\n\
            bytes: {:?}",
                br.len(),
                br.get_remaining()
            );
            println!("Please send me your buffer file, as well, so I can see what is wrong!")
        }

        Ok(TabStateRefs::new(None, TabStateCursor::new(cursor_start, cursor_end), buffer_size, text_buffer, footer))
    }
}
