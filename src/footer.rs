use crate::consts::FOOTER_SIZE;

/// I am pretty sure the footer is this size.
#[repr(C)]
pub struct TabStateFooter {
    pub the_number_zero: u8,
    pub unk: [u8; 4],
}
const _: () = assert!(std::mem::size_of::<TabStateFooter>() == FOOTER_SIZE);
