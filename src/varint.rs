use crate::consts::{MAX_VAL, SIGN_BIT};
use crate::refs::varint::VarIntRef;

pub struct VarInt {
    buffer: Vec<u8>,
}

impl VarInt {
    pub fn new(num: u128) -> VarInt {
        let mut buffer = vec![];

        let mut val = num;
        while val > MAX_VAL as u128 {
            let chunk = get_chunk(val);
            buffer.push(chunk | SIGN_BIT);
            val >>= 7;
        }

        buffer.push(val as u8);

        Self { buffer }
    }
    /// Copies the provided buffer to a new vector and returns a `VarInt`
    pub fn from_buffer(buffer: &[u8]) -> VarInt {
        Self {
            buffer: buffer.to_vec(),
        }
    }
    pub fn get_ref(&self) -> VarIntRef {
        VarIntRef::new(&self.buffer[..])
    }
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer[..]
    }

    pub fn size_of(&self) -> usize {
        self.buffer.len()
    }
}

fn get_chunk(val: u128) -> u8 {
    (val & MAX_VAL as u128) as u8
}

#[cfg(test)]
mod tests {
    use crate::consts::{MAX_VAL, SIGN_BIT};
    use crate::varint::VarInt;

    const TEST_ONE: [u8; 3] = [0xBB, 0x93, 0x2];

    #[test]
    /// This should be the exact three bytes in `TEST_ONE`
    fn encode_varint_large_value() {
        let varint = VarInt::new(35259);
        assert_eq!(TEST_ONE, varint.get_ref().get_buffer());
    }

    const TEST_TWO: [u8; 2] = [0x87, 0x02];

    #[test]
    /// This should be the exact two bytes in `TEST_TWO`
    fn encode_varint_medium_sized_value() {
        let varint = VarInt::new(263);
        assert_eq!(TEST_TWO, varint.get_ref().get_buffer());
    }

    const TEST_THREE: [u8; 1] = [MAX_VAL];

    #[test]
    /// This should be the exact two bytes in `TEST_THREE`
    fn encode_varint_max_value() {
        let varint = VarInt::new(MAX_VAL as u128);
        assert_eq!(TEST_THREE, varint.get_ref().get_buffer());
    }

    const TEST_FOUR: [u8; 2] = [0x80, 0x01];

    /// This should be the exact two bytes in `TEST_FOUR`
    #[test]
    fn encode_varint_should_be_two_bytes() {
        let varint = VarInt::new(SIGN_BIT as u128);
        assert_eq!(TEST_FOUR, varint.get_ref().get_buffer());
    }
}
