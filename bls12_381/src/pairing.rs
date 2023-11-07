use crate::params::{BLS_X, BLS_X_IS_NEGATIVE};
use crate::{Fq12, G1Affine, G2Affine, G2PairingAffine, G2Projective, Gt};
use zkstd::common::*;

/// Tate pairing struct holds necessary components for pairing.
/// `pairing` function takes G1 and G2 group elements and output
/// GT target group element.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Encode, Decode, Copy)]
pub struct TatePairing;

impl TatePairing {
    pub fn pairing(g1: G1Affine, g2: G2Affine) -> Gt {
        Self::miller_loop(g1, g2).final_exp()
    }

    pub fn miller_loop(g1: G1Affine, g2: G2Affine) -> Fq12 {
        let mut acc = Fq12::one();
        let mut g2_projective = G2Projective::from(g2);
        let mut found_one = false;

        for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
            if !found_one {
                found_one = i;
                continue;
            }

            acc = acc.untwist(g2_projective.double_eval(), g1);

            if i {
                acc = acc.untwist(g2_projective.add_eval(g2), g1);
            }

            acc.square_assign();
        }

        acc = acc.untwist(g2_projective.double_eval(), g1);

        if BLS_X_IS_NEGATIVE {
            acc.conjugate()
        } else {
            acc
        }
    }

    pub fn multi_miller_loop(pairs: &[(G1Affine, G2PairingAffine)]) -> Fq12 {
        let pairs = pairs
            .iter()
            .filter(|(a, b)| !a.is_identity() && !b.is_identity())
            .collect::<Vec<_>>();
        let mut acc = Fq12::one();
        let mut counter = 0;
        let mut found_one = false;

        for i in (0..64).rev().map(|b| (((BLS_X >> 1) >> b) & 1) == 1) {
            if !found_one {
                found_one = i;
                continue;
            }

            for (g1, g2) in pairs.iter() {
                acc = acc.untwist(g2.coeffs[counter], *g1);
            }
            counter += 1;

            if i {
                for (g1, g2) in pairs.iter() {
                    acc = acc.untwist(g2.coeffs[counter], *g1);
                }
                counter += 1;
            }

            acc.square_assign();
        }

        for (g1, g2) in pairs {
            acc = acc.untwist(g2.coeffs[counter], *g1);
        }

        if BLS_X_IS_NEGATIVE {
            acc.conjugate()
        } else {
            acc
        }
    }
}
