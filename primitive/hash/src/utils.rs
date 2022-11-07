use parity_scale_codec::alloc::vec::Vec;

pub(crate) type HexBytes = [u8];

pub(crate) type Bits = Vec<u8>;

pub(crate) fn to_bits(mut byte: u8) -> Bits {
    let mut bits = Vec::new();
    for _ in 0..8 {
        bits.push((byte & 1) as u8);
        byte >>= 1;
    }
    bits
}

pub(crate) fn trunc(n: usize, text: Bits) -> Bits {
    text[..n].to_vec()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn to_bits_test() {
        let bytes_small_a = b"a";
        let bytes_big_a = [0xA3];
        let bits_small_a = to_bits(bytes_small_a[0]);
        let bits_big_a = to_bits(bytes_big_a[0]);
        let expected_small_a: Bits = [1, 0, 0, 0, 0, 1, 1, 0].to_vec();
        let expected_big_a: Bits = [1, 1, 0, 0, 0, 1, 0, 1].to_vec();

        assert_eq!(bits_small_a, expected_small_a);
        assert_eq!(bits_big_a, expected_big_a);
    }

    #[test]
    fn trunc_test() {
        let bits: Bits = [1, 0, 0, 0, 0, 1, 1, 0].to_vec();
        let n = 6;
        let trunc_bits = trunc(n, bits);
        let expected_bits = [1, 0, 0, 0, 0, 1].to_vec();

        assert_eq!(trunc_bits, expected_bits);
    }
}
