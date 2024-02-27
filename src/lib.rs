mod consts;

use buffer_reader::BufferReader;
use std::io::{Error, ErrorKind};
use widestring::WideStr;
use crate::consts::*;

pub struct NPBufferReader<'a> {
    buffer: &'a [u8],
}

#[allow(unused)]
pub struct NPRefs<'a> {
    file_path: Option<&'a WideStr>,
    some_data: &'a [u8],
    text_buffer: &'a WideStr,
    footer: &'a [u8; FOOTER_SIZE],
}

impl<'a> NPRefs<'a> {
    pub fn get_path(&self) -> Option<&'a WideStr> {
        self.file_path
    }
    pub fn get_buffer(&self) -> &'a WideStr {
        self.text_buffer
    }
}



impl<'a> NPBufferReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer }
    }
    pub fn get_refs(&self) -> std::io::Result<NPRefs> {
        let br = BufferReader::new(self.buffer);

        let magic = br.read_bytes(3)?;

        if magic != b"NP\0" {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Magic bytes invalid. Should be \"NP\" and a null byte. Read: {} raw: {magic:?}", unsafe { std::str::from_utf8_unchecked(magic) }),
            ));
        }

        let file_state = br.read_byte()?;
        if file_state == FILE_STATE_SAVED {
            return self.read_saved_buffer(br);
        }

        self.read_unsaved_buffer(br)
    }
    fn read_saved_buffer(&self, br: BufferReader<'a>) -> std::io::Result<NPRefs> {
        // Get the file path.
        let path_len = br.read_byte()?;
        let str_bytes = br.read_bytes(path_len as usize * 2)?;
        let file_path =
            Some(unsafe { WideStr::from_ptr(str_bytes.as_ptr() as *const u16, path_len as usize) });

        let marker_one_location = br.find_bytes(&FIRST_MARKER_BYTES).ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("Could not find marker bytes {FIRST_MARKER_BYTES:02X?}"),
        ))?;

        // Read first size which is not important.
        br.read_bytes(marker_one_location)?;

        // Get the metadata
        let some_data = br.read_bytes(SIZE_OF_SOME_STRUCTURE)?;

        // Get the second marker, and make sure it's as expected.
        let marker_two = br.read_bytes(2)?;
        if marker_two != &SECOND_MARKER_BYTES {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown marker encountered. Expected {SECOND_MARKER_BYTES:02X?} Got: {:02X?}.",
                    marker_two
                ),
            ));
        }

        // Find the third marker.
        let marker_three_location = br.find_bytes(&THIRD_MARKER_BYTES).ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("Could not find marker bytes {THIRD_MARKER_BYTES:02X?}"),
        ))?;

        // The third marker is after 2 encoded sizes, which should be the same size as the size of the
        // text buffer. This might break, in some scenarios, as sometimes the size is different. I have
        // to check it out a bit more.
        let size_of_encoded_size = marker_three_location / 2;

        // Advance over the u32 marker we found.
        br.read_bytes(marker_three_location + THIRD_MARKER_BYTES.len())?;

        // Get the bytes that represent the size of the text buffer
        let size_bytes = br.read_bytes(size_of_encoded_size)?;
        let buffer_size = read_cursed_size_format(size_bytes)?;

        // Get the text buffer
        let text_buffer = br.read_bytes(buffer_size)?;
        let text_buffer =
            unsafe { WideStr::from_ptr(text_buffer.as_ptr() as *const u16, text_buffer.len() / 2) };
        let footer = br.read_t()?;

        Ok(NPRefs {
            file_path,
            some_data,
            text_buffer,
            footer,
        })
    }
    fn read_unsaved_buffer(&self, _br: BufferReader) -> std::io::Result<NPRefs> {
        Err(Error::new(
            ErrorKind::Unsupported,
            "Unsaved files are currently not supported by this reader.",
        ))
    }
}


fn read_cursed_size_format(size_buffer: &[u8]) -> std::io::Result<usize> {
    if size_buffer.len() > 2 {
        return Err(Error::new(
            ErrorKind::Unsupported,
            "Bold of you to think I know a good algorithm to decode more than 2 bytes of this \
            crap. It's a miracle I got this far. I curse you, Microsoft. 10,000 years!",
        ));
    }

    let first_byte = size_buffer[0] as usize;
    if size_buffer.len() == 1 {
        return Ok(first_byte);
    }

    let iter = size_buffer[1] as usize;
    let initial_val = MAX_VAL * iter;
    let size = initial_val + iter + (first_byte & MAX_VAL);

    Ok(size)
}

#[cfg(test)]
mod tests {
    use crate::NPBufferReader;

    const BUFFER_PATH: &str = r"C:\Users\Nordgaren\AppData\Local\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState\2dc4e84a-b897-4bd1-99a0-2fd3e875659e.bin";

    #[test]
    fn it_works() {
        let buffer = std::fs::read(BUFFER_PATH).unwrap();
        let np = NPBufferReader::new(&buffer[..]);
        let refs = np.get_refs().unwrap();

        println!("{:?}", refs.file_path);
        println!("{:?}", refs.text_buffer);
    }
}
