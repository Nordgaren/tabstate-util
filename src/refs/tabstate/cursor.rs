use crate::refs::varint::VarIntRef;

/// Two varints that represent the start and end of the tabs cursor in chars. These should be the same
/// if there is no selection.
pub struct TabStateCursor<'a> {
    cursor_start: VarIntRef<'a>,
    cursor_end: VarIntRef<'a>,
}

impl<'a> TabStateCursor<'a> {
    pub fn new(cursor_start: VarIntRef<'a>, cursor_end: VarIntRef<'a>) -> Self {
        Self {
            cursor_start,
            cursor_end,
        }
    }
    /// Get a reference to the cursor start VarInt that represents the size of the text file on disk,
    /// if available.
    pub fn get_cursor_start(&'a self) -> VarIntRef<'a> {
        self.cursor_start
    }
    /// Get a reference to the cursor end VarInt that represents the size of the text file on disk,
    /// if available.
    pub fn get_cursor_end(&'a self) -> VarIntRef<'a> {
        self.cursor_end
    }
    /// Returns true if the cursor start and cursor end varints are not the same
    pub fn is_selection(&self) -> bool {
        self.cursor_start != self.cursor_end
    }
}
