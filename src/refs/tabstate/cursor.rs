use crate::refs::varint::VarIntRef;

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
}
