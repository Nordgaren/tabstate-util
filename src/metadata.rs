use crate::consts::METADATA_STRUCTURE_SIZE;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum CarriageType {
    Unix = 1,
    CRLF = 3,
}
/// I am pretty sure there is a metadata structure of this size
#[repr(C)]
pub struct TabStateMetadata {
    pub the_number_five: u8,
    pub return_carriage: CarriageType,
    pub unk: [u8; 0x6],
    pub unk_two: [u8; 0x23],
}
const _: () = assert!(std::mem::size_of::<TabStateMetadata>() == METADATA_STRUCTURE_SIZE);
