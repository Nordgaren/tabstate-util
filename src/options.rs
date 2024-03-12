use bytemuck::AnyBitPattern;


/// An option struct that holds text editor option state for the tab.
#[repr(C)]
#[derive(Copy, Clone, AnyBitPattern)]
pub struct TabStateOptions {
    word_wrap: u8,
    right_to_left: u8,
    show_unicode_control: u8,
    unk: u8,
}

impl TabStateOptions {
    pub fn word_wrap(&self) -> bool {
        self.word_wrap != 0
    }
    pub fn right_to_left(&self) -> bool {
        self.right_to_left != 0
    }
    pub fn show_unicode_control(&self) -> bool {
        self.show_unicode_control != 0
    }
    pub fn unk(&self) -> bool {
        self.unk != 0
    }
}

pub const OPTIONS_SIZE: usize = 0x4;
const _: () = assert!(std::mem::size_of::<TabStateOptions>() == OPTIONS_SIZE);