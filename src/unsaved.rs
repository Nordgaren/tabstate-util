use std::io::{Error, ErrorKind};
use buffer_reader::BufferReader;
use widestring::WideStr;
use crate::consts::{SIZE_START_MARKER, SIZE_END_MARKER, UNSUPPORTED_MESSAGE, SIGN_BIT};
use crate::{NPBufferReader, NPRefs, decode_varint};

impl<'a> NPBufferReader<'a> {
    /// Reads a Notepad tab buffer that is not saved to disk, and does not have a filepath. Currently
    /// unsupported if Notepad has not been closed since the tab was opened.
    pub(crate) fn read_unsaved_buffer(&self, br: BufferReader<'a>) -> std::io::Result<NPRefs<'a>> {
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

        // See saved buffer function for more details
        let marker_three_location = br.find_bytes(&SIZE_END_MARKER).ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("Could not find marker bytes: {SIZE_END_MARKER:02X?}"),
        ))?;

        // Advance over the third marker we found.
        br.read_bytes(marker_three_location + SIZE_END_MARKER.len())?;

        // Get the bytes that represent the size of the text buffer and decode the size.
        let mut count = 0;

        loop {
            let byte = br.peek_byte(count)?;
            count += 1;

            if byte & SIGN_BIT == 0 {
                break;
            }
        }

        let size_bytes = br.read_bytes(count)?;
        let buffer_size = decode_varint(size_bytes)?;
        if buffer_size == 0 {
            return Err(Error::new(ErrorKind::Unsupported, UNSUPPORTED_MESSAGE));
        }

        // The text buffer should be right after the final size buffer we just read.
        let text_buffer = br.read_bytes(buffer_size * 2)?;
        let text_buffer =
            unsafe { WideStr::from_ptr(text_buffer.as_ptr() as *const u16, buffer_size) };
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


        Ok(NPRefs::new(None, None, text_buffer, footer))
    }
}