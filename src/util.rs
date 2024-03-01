use widestring::WideStr;

/// The buffer should be double the length of the provided string_len, otherwise this function panics.
/// Unsure if I should make this function unsafe, as well, as the caller also needs to uphold the
/// safety guarantees of `WideStr::from_ptr`, but really we should have a valid pointer if we have a
/// slice.
pub(crate) fn wide_string_from_buffer(buffer: &[u8], string_len: usize) -> &WideStr {
    assert_eq!(
        buffer.len() / 2,
        string_len,
        "&[u8] length must be half the length of the provided string length."
    );

    unsafe { WideStr::from_ptr(buffer.as_ptr() as *const u16, string_len) }
}
