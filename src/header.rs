use bytemuck::{AnyBitPattern, Zeroable};

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum State {
    Unsaved = 0,
    Saved = 1,
}
unsafe impl Zeroable for State {}
unsafe impl AnyBitPattern for State {}

/// A header that represents the magic bytes and the state of the TabState file.
#[repr(C)]
#[derive(Copy, Clone, AnyBitPattern)]
pub struct Header {
    pub(crate) magic: [u8; 3],
    pub(crate) state: State,
}

pub const HEADER_SIZE: usize = 0x4;
const _: () = assert!(std::mem::size_of::<Header>() == HEADER_SIZE);
