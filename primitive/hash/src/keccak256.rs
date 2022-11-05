/// keccak implementation according to following
/// https://keccak.team/obsolete/Keccak-specifications.pdf
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
