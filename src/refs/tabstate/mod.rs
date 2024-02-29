#![doc = "TabState references to each part of a TabState file. This is generic, so some parts are optional"]

use std::io::{Error, ErrorKind};
use buffer_reader::BufferReader;
use widestring::WideStr;
use crate::consts::{FILE_STATE_SAVED, FILE_STATE_UNSAVED, FIRST_MARKER_BYTE, FIRST_MARKER_VARIANTS, SECOND_MARKER_BYTES, SIZE_END_MARKER};
use crate::footer::TabStateFooter;
use crate::metadata::TabStateMetadata;
use crate::refs::varint::VarIntRef;

pub mod unsaved;

/// A structure tht holds references to the data in a Notepad buffer.
#[allow(unused)]
pub struct TabStateRefs<'a> {
    file_path: Option<&'a WideStr>,
    unk_varint: Option<VarIntRef<'a>>,
    some_metadata: Option<&'a TabStateMetadata>,
    line_position_one: VarIntRef<'a>,
    line_position_two: VarIntRef<'a>,
    buffer_size: VarIntRef<'a>,
    text_buffer: &'a WideStr,
    footer: &'a TabStateFooter,
}

impl<'a> TabStateRefs<'a> {
    /// Returns a new `TabStateRefs` object containing the provided refs.
    pub fn new(
        file_path: Option<&'a WideStr>,
        unk_varint: Option<VarIntRef<'a>>,
        some_metadata: Option<&'a TabStateMetadata>,
        line_position_one: VarIntRef<'a>,
        line_position_two: VarIntRef<'a>,
        buffer_size: VarIntRef<'a>,
        text_buffer: &'a WideStr,
        footer: &'a TabStateFooter,
    ) -> TabStateRefs<'a> {
        Self {
            file_path,
            unk_varint,
            some_metadata,
            line_position_one,
            line_position_two,
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
        let path_len = br.read_byte()?;
        let str_bytes = br.read_bytes(path_len as usize * 2)?;
        let file_path =
            Some(unsafe { WideStr::from_ptr(str_bytes.as_ptr() as *const u16, path_len as usize) });

        let unk_varint = VarIntRef::from_reader(&br)?;

        // Get the first marker, which denotes the start of the metadata structure. This might be two
        // different fields, or incidental. I am not sure.
        let marker_one_location = br.find_bytes(&FIRST_MARKER_BYTE).ok_or(Error::new(
            ErrorKind::InvalidData,
            format!("Could not find marker bytes: {FIRST_MARKER_BYTE:02X?}"),
        ))?;

        // Read first size and possibly some ohter metadata which is currently unknown.
        br.read_bytes(marker_one_location)?;

        // Get the main metadata object (?)
        let some_metadata = br.read_t::<TabStateMetadata>()?;

        // The metadata structure includes the 0x5 marker and the variant. The variant is the second
        // byte.
        let marker_variant = &some_metadata.variant;
        if !FIRST_MARKER_VARIANTS.contains(marker_variant) {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Unknown file variant. Expected one of: {FIRST_MARKER_VARIANTS:?}. Got: {:X}",
                    *marker_variant
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

        let line_position_one = VarIntRef::from_reader(&br)?;
        let line_position_two = VarIntRef::from_reader(&br)?;

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
        let text_buffer =
            unsafe { WideStr::from_ptr(text_buffer.as_ptr() as *const u16, decoded_size) };
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

        Ok(TabStateRefs::new(
            file_path,
            Some(unk_varint),
            Some(some_metadata),
            line_position_one,
            line_position_two,
            buffer_size,
            text_buffer,
            footer,
        ))
    }
}