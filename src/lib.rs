mod consts;
mod unsaved;

use crate::consts::*;
use buffer_reader::BufferReader;
use std::io::{Error, ErrorKind};
use widestring::WideStr;

/// A structure that parses the Notepad buffer data.
pub struct NPBufferReader<'a> {
    buffer: &'a [u8],
}

/// A structure tht holds references to the data in a Notepad buffer.
#[allow(unused)]
pub struct NPRefs<'a> {
    file_path: Option<&'a WideStr>,
    some_metadata: Option<&'a [u8; SIZE_OF_METADATA_STRUCTURE]>,
    text_buffer: &'a WideStr,
    footer: &'a [u8; FOOTER_SIZE],
}

impl<'a> NPRefs<'a> {
    /// Returns a new `NPRefs` object containing the provided refs.
    pub fn new(
        file_path: Option<&'a WideStr>,
        some_metadata: Option<&'a [u8; SIZE_OF_METADATA_STRUCTURE]>,
        text_buffer: &'a WideStr,
        footer: &'a [u8; FOOTER_SIZE],
    ) -> NPRefs<'a> {
        Self {
            file_path,
            some_metadata,
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
}

impl<'a> NPBufferReader<'a> {
    /// Returns a new `NPBufferReader` that contains the provided buffer.
    pub fn new(buffer: &'a [u8]) -> std::io::Result<Self> {
        if buffer.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Buffer is empty."));
        }

        Ok(Self { buffer })
    }
    /// Get references to the individual parts of the Notepad buffer, like the filepath and the text
    /// buffer, as well as some unknown metadata.
    pub fn get_refs(&self) -> std::io::Result<NPRefs<'a>> {
        let br = BufferReader::new(self.buffer);

        let magic = br.read_bytes(3)?;

        if magic != b"NP\0" {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Magic bytes invalid. Should be \"NP\" and a null byte. Read: {} raw: {magic:?}",
                    unsafe { std::str::from_utf8_unchecked(magic) }
                ),
            ));
        }

        let file_state = br.read_byte()?;
        if file_state == FILE_STATE_SAVED {
            return self.read_saved_buffer(br);
        } else if file_state == FILE_STATE_UNSAVED {
            return self.read_unsaved_buffer(br);
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid file state. Expected 0 or 1. Got: {file_state}"),
        ))
    }
    /// Reads a Notepad tab buffer that is saved to disk, and has a filepath and the text buffer.
    /// Unsaved buffers are currently unsupported.
    fn read_saved_buffer(&self, br: BufferReader<'a>) -> std::io::Result<NPRefs<'a>> {
        // Get the file path.
        let path_len = br.read_byte()?;
        let str_bytes = br.read_bytes(path_len as usize * 2)?;
        let file_path =
            Some(unsafe { WideStr::from_ptr(str_bytes.as_ptr() as *const u16, path_len as usize) });

        // Get the first marker, which denotes the start of the metadata structure. This might be two
        // different fields, or incidental. I am not sure.
        let marker_one_location = br.find_bytes(&FIRST_MARKER_BYTES).ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("Could not find marker bytes: {FIRST_MARKER_BYTES:02X?}"),
        ))?;

        // Read first size and possibly some metadata which is currently unknown.
        br.read_bytes(marker_one_location)?;

        // Get the main metadata object (?)
        let some_metadata = Some(br.read_t()?);

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

        // Find the third marker, which denotes the end of the two unknown sizes. I have noticed these
        // sizes are sometimes the same as the buffer and sometimes not the same. They might also be
        // different sizes (as in bytes) than the main text buffer size. They can also both be 0 for
        // some reason.
        let marker_three_location = br.find_bytes(&SIZE_END_MARKER).ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("Could not find marker bytes: {THIRD_MARKER_BYTES:02X?}"),
        ))?;

        // The third marker is after 2 encoded sizes, which should be the same size as the size of the
        // text buffer. This might break, in some scenarios, as sometimes the size is different. I have
        // to check it out a bit more. When the two previous sizes are 0, the value of `marker_three_location`
        // is 2, and the text buffer size should also be 2. I am not sure how this happens, so I am
        // sure this is going to fail sometime.
        let size_of_encoded_size = if marker_three_location > 2 {
            marker_three_location / 2
        } else {
            marker_three_location
        };

        // Advance over the third marker we found.
        br.read_bytes(marker_three_location + SIZE_END_MARKER.len())?;

        // Get the bytes that represent the size of the text buffer and decode the size.
        let size_bytes = br.read_bytes(size_of_encoded_size)?;
        let buffer_size = read_cursed_size_format(size_bytes)?;

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

        Ok(NPRefs::new(file_path, some_metadata, text_buffer, footer))
    }
}

/// Decodes the buffer as a size that wraps at 127. Then the count starts at 0x80. It's basically wrapping
/// as `i8::MAX`, but the carry bytes all have the sign bit set. I wonder if this is for them to decode
/// in order?
fn read_cursed_size_format(size_buffer: &[u8]) -> std::io::Result<usize> {
    if size_buffer.len() > 2 {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Bold of you to think I know a good algorithm to decode more than 2 bytes of this \
            crap. It's a miracle I got this far. I curse you, Microsoft! 10,000 years!",
        ));
    }

    // Each byte except the last one in the size buffer has the sign bit set. It might be an indicator
    // that the byte is a carry over from the next byte, and there is probably a formula to calculate
    // the size value using each carry byte, but I am not a mathematician, so idk off the top of my head.
    let first_byte = get_real_value(size_buffer[0] as usize);
    if size_buffer.len() == 1 {
        return Ok(first_byte);
    }

    // Since we are only doing 2 bytes, we hard code the last one for now. We need to multiply the
    // max value by this number and then add the first bytes real value to that, as well as the value
    // of the iterator itself. Yea, idk.
    let iter = get_real_value(size_buffer[1] as usize);
    let initial_val = MAX_VAL * iter;
    let size = initial_val + iter + first_byte;

    Ok(size)
}

#[inline(always)]
fn get_real_value(value: usize) -> usize {
    value & MAX_VAL
}

#[cfg(test)]
mod tests {
    use crate::consts::UNSUPPORTED_MESSAGE;
    use crate::NPBufferReader;

    const BUFFER_PATH: &str = concat!(
    env!("localappdata"),
    r"\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState\",
    "0c07e304-0604-4438-941d-0977da045fd9.bin" // You should be able to just change this file name.
    );

    /// Should not panic
    #[test]
    fn read_tabstate() {
        let buffer = std::fs::read(BUFFER_PATH).unwrap();
        let np = NPBufferReader::new(&buffer[..]).expect("Could not create ne NPBufferReader");
        let _ = np.get_refs().unwrap();

        //println!("{:?}", refs.get_path().unwrap_or_default());
        //println!("{:?}", refs.get_buffer());
    }

    /// Prints out error message if something abnormal happens.
    #[test]
    fn read_tabstate_folder() {
        let env = env!("localappdata");
        let files = std::fs::read_dir(&format!(r"{env}\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState")).unwrap();
        for file in files {
            let path = match file {
                Ok(p) => p,
                Err(_) => continue,
            };

            let buffer = std::fs::read(path.path()).unwrap();
            let np = match NPBufferReader::new(&buffer[..]) {
                Ok(np) => np,
                Err(_) => continue,
            };

            let error = match np.get_refs() {
                Ok(_) => continue,
                Err(e) => e,
            };
            let e_string = error.to_string();
            match &e_string[..] {
                UNSUPPORTED_MESSAGE => continue,
                _ => {
                    println!("{path:?}");
                    println!("{error}");
                }
            }
        }
    }
}
