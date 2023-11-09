use crate::{Fq, Fq12, Fq2, G1Affine, G2Affine, G2PairingAffine, Gt};
use zkstd::common::*;

// 6U+2 for in NAF form
pub const SIX_U_PLUS_2_NAF: [i8; 65] = [
    0, 0, 0, 1, 0, 1, 0, -1, 0, 0, 1, -1, 0, 0, 1, 0, 0, 1, 1, 0, -1, 0, 0, 1, 0, -1, 0, 0, 0, 0,
    1, 1, 1, 0, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, 1, 1, 0, 0, -1, 0, 0, 0, 1, 1, 0, -1, 0,
    0, 1, 0, 1, 1,
];

pub const XI_TO_Q_MINUS_1_OVER_2: Fq2 = Fq2([
    Fq([
        0xe4bbdd0c2936b629,
        0xbb30f162e133bacb,
        0x31a9d1b6f9645366,
        0x253570bea500f8dd,
    ]),
    Fq([
        0xa1d77ce45ffe77c7,
        0x07affd117826d1db,
        0x6d16bd27bb7edc6b,
        0x2c87200285defecc,
    ]),
]);

/// Ate pairing struct holds necessary components for pairing.
/// `pairing` function takes G1 and G2 group elements and output
/// GT target group element.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Encode, Decode, Copy)]
pub struct AteParing;

impl AteParing {
    pub fn pairing(g1: G1Affine, g2: G2Affine) -> Gt {
        let g2 = G2PairingAffine::from(g2);
        Self::multi_miller_loop(&[(g1, g2)]).final_exp()
    }

    pub fn multi_miller_loop(pairs: &[(G1Affine, G2PairingAffine)]) -> Fq12 {
        let mut pairs = pairs
            .iter()
            .filter(|(a, b)| !(a.is_identity()) && !b.is_identity())
            .map(|(g1, g2)| (g1, g2.coeffs.iter()))
            .collect::<Vec<_>>();

        let mut acc = Fq12::one();

        for i in (1..SIX_U_PLUS_2_NAF.len()).rev() {
            if i != SIX_U_PLUS_2_NAF.len() - 1 {
                acc.square_assign();
            }
            for &mut (p, ref mut coeffs) in &mut pairs {
                acc = acc.untwist(*coeffs.next().unwrap(), *p);
            }
            let x = SIX_U_PLUS_2_NAF[i - 1];
            match x {
                1 => {
                    for &mut (p, ref mut coeffs) in &mut pairs {
                        acc = acc.untwist(*coeffs.next().unwrap(), *p);
                    }
                }
                -1 => {
                    for &mut (p, ref mut coeffs) in &mut pairs {
                        acc = acc.untwist(*coeffs.next().unwrap(), *p);
                    }
                }
                _ => continue,
            }
        }

        for &mut (p, ref mut coeffs) in &mut pairs {
            acc = acc.untwist(*coeffs.next().unwrap(), *p);
        }

        for &mut (p, ref mut coeffs) in &mut pairs {
            acc = acc.untwist(*coeffs.next().unwrap(), *p);
        }

        for &mut (_p, ref mut coeffs) in &mut pairs {
            assert_eq!(coeffs.next(), None);
        }

        acc
    }
}
