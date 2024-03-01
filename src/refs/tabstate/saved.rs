use crate::metadata::TabStateMetadata;
use crate::refs::varint::VarIntRef;
use widestring::WideStr;

#[derive(Copy, Clone)]
pub struct SavedRefs<'a> {
    file_path: &'a WideStr,
    full_buffer_size: VarIntRef<'a>,
    metadata: &'a TabStateMetadata,
}

impl<'a> SavedRefs<'a> {
    pub fn new(
        file_path: &'a WideStr,
        full_buffer_size: VarIntRef<'a>,
        metadata: &'a TabStateMetadata,
    ) -> Self {
        Self {
            file_path,
            full_buffer_size,
            metadata,
        }
    }
    /// Get a reference to the path of the file this TabState represents. Unsaved files do not have
    /// a path.
    pub fn get_path(&self) -> &'a WideStr {
        self.file_path
    }
    /// Get a reference to the full buffer size VarInt that represents the size of the text file on
    /// disk, if available.
    pub fn get_full_buffer_size(&'a self) -> VarIntRef<'a> {
        self.full_buffer_size
    }
    /// Get a reference to the metadata structure, if available.
    pub fn get_metadata(&self) -> &'a TabStateMetadata {
        self.metadata
    }
}
