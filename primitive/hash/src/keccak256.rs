/// the terminology bellow is aligned with the following paper
/// https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.202.pdf
use crate::utils::{to_bits, trunc, Bits, HexBytes};
use parity_scale_codec::alloc::vec::Vec;

struct Keccak {
    r: u32,
    c: u32,
    s: Vec<u8>,
}

impl Keccak {
    fn new(witdh: usize) -> Self {
        match witdh {
            256 => Self {
                r: 1088,
                c: 512,
                s: Vec::new(),
            },
            _ => unimplemented!(),
        }
    }

    fn pad(input: Vec<u8>) -> Vec<u8> {
        let n = 8;

        input
    }
}

pub fn keccak256(input: Vec<u8>) -> Vec<u8> {
    let keccak_1600 = Keccak::new(256);

    input
}

fn h2b(bytes: &HexBytes, n: usize) -> Bits {
    let mut output = Vec::new();
    for byte in bytes.iter() {
        let mut bits = to_bits(*byte);
        output.append(&mut bits);
    }
    trunc(n, output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h2b_test() {
        let text: [u8; 2] = [0xA3, 0x2E];
        let binary_text = h2b(&text, 14);
        let expected_binary_text = [1, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1].to_vec();

        assert_eq!(binary_text, expected_binary_text);
    }
}