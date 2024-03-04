use bytemuck::AnyBitPattern;

/// I am pretty sure the footer is this size.
#[repr(C)]
#[derive(Copy, Clone, AnyBitPattern)]
pub struct TabStateFooter {
    pub the_number_zero: u8,
    pub unk: [u8; 4],
}
pub const FOOTER_SIZE: usize = 0x5;
const _: () = assert!(std::mem::size_of::<TabStateFooter>() == FOOTER_SIZE);
