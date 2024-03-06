pub mod consts;
pub mod footer;
pub mod header;
pub mod metadata;
pub mod refs;
pub mod varint;

use crate::consts::*;
use crate::refs::tabstate::TabStateRefs;
use std::io::{Error, ErrorKind};

/// A structure that parses the Notepad buffer data.
pub struct TabStateReader<'a> {
    buffer: &'a [u8],
}

impl<'a> TabStateReader<'a> {
    /// Returns a new `TabStateReader` that contains the provided buffer.
    pub fn new(buffer: &'a [u8]) -> std::io::Result<Self> {
        if buffer.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Buffer is empty."));
        }

        Ok(Self { buffer })
    }
    /// Get references to the individual parts of the Notepad buffer, like the filepath and the text
    /// buffer, as well as some unknown metadata.
    pub fn get_refs(&self) -> std::io::Result<TabStateRefs<'a>> {
        TabStateRefs::from_buffer(self.buffer)
    }
}

/// # Leaving this for the funny
/// Decodes the buffer as a size that wraps at 127. Then the count starts at 0x80. It's basically wrapping
/// as `i8::MAX`, but the carry bytes all have the sign bit set. I wonder if this is for them to decode
/// in order?
#[allow(unused)]
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
    let first_byte = (size_buffer[0] & MAX_VAL) as usize;
    if size_buffer.len() == 1 {
        return Ok(first_byte);
    }

    // Since we are only doing 2 bytes, we hard code the last one for now. We need to multiply the
    // max value by this number and then add the first bytes real value to that, as well as the value
    // of the iterator itself. Yea, idk.
    let iter = size_buffer[1] & MAX_VAL;
    let initial_val = (MAX_VAL * iter) as usize;
    let size = initial_val + iter as usize + first_byte;

    Ok(size)
}

#[cfg(test)]
mod tests {
    use crate::TabStateReader;
    use std::io::ErrorKind;

    const BUFFER_PATH: &str = concat!(
        env!("localappdata"),
        r"\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState\",
        "fd86c273-1b2e-4375-937b-8b8513bbd50a.bin" // You should be able to just change this file name.
    );

    /// Should not panic
    #[test]
    fn read_tabstate() {
        let buffer = std::fs::read(BUFFER_PATH).unwrap();
        let np = TabStateReader::new(&buffer[..]).expect("Could not create ne TabStateReader");
        let _ = np.get_refs().unwrap();

        //println!("{:?}", refs.get_path().unwrap_or_default());
        //println!("{:?}", refs.get_buffer());
    }

    /// Prints out error message if something abnormal happens.
    #[test]
    fn read_tabstate_folder() {
        let env = env!("localappdata");
        let files = std::fs::read_dir(&format!(
            r"{env}\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState"
        ))
        .unwrap();
        for file in files {
            let path = match file {
                Ok(p) => p,
                Err(_) => continue,
            };
            //println!("{path:?}");

            let buffer = std::fs::read(path.path()).unwrap();
            let np = match TabStateReader::new(&buffer[..]) {
                Ok(np) => np,
                Err(_) => continue,
            };

            let error = match np.get_refs() {
                Ok(_) => continue,
                Err(e) => e,
            };
            match error.kind() {
                ErrorKind::Unsupported => {}
                _ => {
                    println!("{path:?}");
                    println!("{error}");
                }
            }
        }
    }
}
