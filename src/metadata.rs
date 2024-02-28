use crate::consts::METADATA_STRUCTURE_SIZE;

/// I am pretty sure there is a metadata structure of this size
#[repr(C)]
pub struct TabStateMetadata {
    pub the_number_five: u8,
    pub variant: u8,
    pub unk: [u8; 0x29],
}
const _: () = assert!(std::mem::size_of::<TabStateMetadata>() == METADATA_STRUCTURE_SIZE);
