mod consts;

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
    some_metadata: &'a [u8; SIZE_OF_METADATA_STRUCTURE],
    text_buffer: &'a WideStr,
    footer: &'a [u8; FOOTER_SIZE],
}
impl<'a> NPRefs<'a> {
    /// Returns a new `NPRefs` object containing the provided refs.
    pub fn new(
        file_path: Option<&'a WideStr>,
        some_metadata: &'a [u8; SIZE_OF_METADATA_STRUCTURE],
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
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }
    /// Get references to the individual parts of the Notepad buffer, like the filepath and the text
    /// buffer, as well as some unknown metadata.
    pub fn get_refs(&self) -> std::io::Result<NPRefs> {
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
        }

        self.read_unsaved_buffer(br)
    }
    /// Reads a Notepad tab buffer that is saved to disk, and has a filepath and the text buffer.
    /// Unsaved buffers are currently unsupported.
    fn read_saved_buffer(&self, br: BufferReader<'a>) -> std::io::Result<NPRefs> {
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
        let some_metadata = br.read_t()?;

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
        let marker_three_location = br.find_bytes(&THIRD_MARKER_BYTES).ok_or(Error::new(
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
        br.read_bytes(marker_three_location + THIRD_MARKER_BYTES.len())?;

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
        if br.len() != 0 {
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
    /// Reads a Notepad tab buffer that is not saved to disk, and does not have a filepath. Currently
    /// unsupported.
    fn read_unsaved_buffer(&self, _br: BufferReader) -> std::io::Result<NPRefs> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Unsaved files are currently not supported by this reader.",
        ))
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

    let first_byte = get_real_value(size_buffer[0] as usize);
    if size_buffer.len() == 1 {
        return Ok(first_byte);
    }

    let iter = get_real_value(size_buffer[1] as usize);
    let initial_val = MAX_VAL * iter;
    let size = initial_val + iter + first_byte;

    Ok(size)
}

fn get_real_value(value: usize) -> usize {
    value & MAX_VAL
}

#[cfg(test)]
mod tests {
    use crate::NPBufferReader;

    const BUFFER_PATH: &str = r"C:\Users\Nordgaren\AppData\Local\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState\1357df91-87b1-4f96-9324-920bb2aece4a.bin";
    #[test]
    fn read_tabstate() {
        let buffer = std::fs::read(BUFFER_PATH).unwrap();
        let np = NPBufferReader::new(&buffer[..]);
        let refs = np.get_refs().unwrap();

        println!("{:?}", refs.get_path().unwrap_or_default());
        println!("{:?}", refs.get_buffer());
    }

    #[test]
    fn read_tabstate_folder() {

        let files =std::fs::read_dir(r"C:\Users\Nordgaren\AppData\Local\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState").unwrap();

        for file in files {
            let path = match file {
                Ok(p) => p,
                Err(_) => continue,
            };

            let buffer = std::fs::read(BUFFER_PATH).unwrap();
            let np = NPBufferReader::new(&buffer[..]);
            let refs = match np.get_refs() {
                Ok(r) => r,
                Err(_) => continue,
            };

            println!("{:?}", refs.get_path().unwrap_or_default());
            println!("{:?}", refs.get_buffer());

        }

    }
}
