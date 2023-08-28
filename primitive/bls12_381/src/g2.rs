use crate::fq::Fq;
use crate::fqn::{Fq12, Fq2};
use crate::fr::Fr;
use crate::params::*;
use core::borrow::Borrow;
use core::iter::Sum;
use zkstd::arithmetic::weierstrass::*;
use zkstd::common::*;
use zkstd::dress::{curve::weierstrass::*, pairing::bls12_g2_pairing};

const B: Fq2 = Fq2([
    Fq([
        0xaa270000000cfff3,
        0x53cc0032fc34000a,
        0x478fe97a6b0a807f,
        0xb1d37ebee6ba24d7,
        0x8ec9733bbf78ab2f,
        0x9d645513d83de7e,
    ]),
    Fq([
        0xaa270000000cfff3,
        0x53cc0032fc34000a,
        0x478fe97a6b0a807f,
        0xb1d37ebee6ba24d7,
        0x8ec9733bbf78ab2f,
        0x9d645513d83de7e,
    ]),
]);

const B3: Fq2 = B.add_const(B).add_const(B);

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G2Affine {
    x: Fq2,
    y: Fq2,
    is_infinity: bool,
}

impl G2Affine {
    /// Returns true if this point is free of an $h$-torsion component, and so it
    /// exists within the $q$-order subgroup $\mathbb{G}_2$. This should always return true
    /// unless an "unchecked" API was used.
    pub fn is_torsion_free(&self) -> bool {
        // Algorithm from Section 4 of https://eprint.iacr.org/2021/1130
        // Updated proof of correctness in https://eprint.iacr.org/2022/352
        //
        // Check that psi(P) == [x] P
        let p = G2Projective::from(*self);
        p.psi() == p.mul_by_x()
    }
}

impl Add for G2Affine {
    type Output = G2Projective;

    fn add(self, rhs: G2Affine) -> Self::Output {
        add_affine_point(self, rhs)
    }
}

impl Neg for G2Affine {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            is_infinity: self.is_infinity,
        }
    }
}

impl Sub for G2Affine {
    type Output = G2Projective;

    fn sub(self, rhs: G2Affine) -> Self::Output {
        add_affine_point(self, rhs.neg())
    }
}

impl Mul<Fr> for G2Affine {
    type Output = G2Projective;

    fn mul(self, rhs: Fr) -> Self::Output {
        scalar_point(self.to_extended(), &rhs)
    }
}

impl Mul<G2Affine> for Fr {
    type Output = G2Projective;

    fn mul(self, rhs: G2Affine) -> Self::Output {
        scalar_point(rhs.to_extended(), &self)
    }
}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G2Projective {
    pub(crate) x: Fq2,
    pub(crate) y: Fq2,
    pub(crate) z: Fq2,
}

impl SigUtils<96> for G2Affine {
    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let infinity = self.is_infinity;

        // Strictly speaking, self.x is zero already when self.infinity is true, but
        // to guard against implementation mistakes we do not assume this.
        let x = if infinity { Fq2::zero() } else { self.x };

        let mut res = [0; Self::LENGTH];

        res[0..48].copy_from_slice(&x.0[1].to_bytes()[..]);
        res[48..96].copy_from_slice(&x.0[0].to_bytes()[..]);

        // This point is in compressed form, so we set the most significant bit.
        res[0] |= 1u8 << 7;

        // Is this point at infinity? If so, set the second-most significant bit.
        res[0] |= if infinity { 1u8 << 6 } else { 0u8 };

        // Is the y-coordinate the lexicographically largest of the two associated with the
        // x-coordinate? If so, set the third-most significant bit so long as this is not
        // the point at infinity.
        res[0] |= if (!infinity) & self.y.lexicographically_largest() {
            1u8 << 5
        } else {
            0u8
        };

        res
    }

    fn from_bytes(buf: [u8; Self::LENGTH]) -> Option<Self> {
        // We already know the point is on the curve because this is established
        // by the y-coordinate recovery procedure in from_compressed_unchecked().

        // Obtain the three flags from the start of the byte sequence
        let compression_flag_set = (buf[0] >> 7) & 1 == 1;
        let infinity_flag_set = (buf[0] >> 6) & 1 == 1;
        let sort_flag_set = (buf[0] >> 5) & 1 == 1;

        // Attempt to obtain the x-coordinate
        let xc1 = {
            let mut tmp = [0; 48];
            tmp.copy_from_slice(&buf[0..48]);

            // Mask away the flag bits
            tmp[0] &= 0b0001_1111;

            Fq::from_bytes(tmp)
        };
        let xc0 = {
            let mut tmp = [0; 48];
            tmp.copy_from_slice(&buf[48..96]);

            Fq::from_bytes(tmp)
        };

        xc1.and_then(|xc1| {
            xc0.and_then(|xc0| {
                let x = Fq2([xc0, xc1]);

                // If the infinity flag is set, return the value assuming
                // the x-coordinate is zero and the sort bit is not set.
                //
                // Otherwise, return a recovered point (assuming the correct
                // y-coordinate can be found) so long as the infinity flag
                // was not set.
                if infinity_flag_set & // Infinity flag should be set
                    compression_flag_set & // Compression flag should be set
                    (!sort_flag_set) & // Sort flag should not be set
                    x.is_zero()
                {
                    Some(G2Affine::ADDITIVE_IDENTITY)
                } else {
                    // Recover a y-coordinate given x by y = sqrt(x^3 + 4)
                    ((x.square() * x) + B).sqrt().and_then(|y| {
                        // Switch to the correct y-coordinate if necessary.
                        let y = if y.lexicographically_largest() ^ sort_flag_set {
                            -y
                        } else {
                            y
                        };
                        if (!infinity_flag_set) & // Infinity flag should not be set
                            compression_flag_set
                        {
                            Some(G2Affine {
                                x,
                                y,
                                is_infinity: infinity_flag_set,
                            })
                        } else {
                            None
                        }
                    })
                }
            })
        })
        .and_then(|p| if p.is_torsion_free() { Some(p) } else { None })
    }
}

impl G2Projective {
    fn mul_by_x(&self) -> G2Projective {
        let mut xself = G2Projective::ADDITIVE_IDENTITY;
        // NOTE: in BLS12-381 we can just skip the first bit.
        let mut x = BLS_X >> 1;
        let mut acc = *self;
        while x != 0 {
            acc = acc.double();
            if x % 2 == 1 {
                xself += acc;
            }
            x >>= 1;
        }
        // finally, flip the sign
        if BLS_X_IS_NEGATIVE {
            xself = -xself;
        }
        xself
    }

    fn psi(&self) -> G2Projective {
        // 1 / ((u+1) ^ ((q-1)/3))
        let psi_coeff_x = Fq2([
            Fq::zero(),
            Fq([
                0x890dc9e4867545c3,
                0x2af322533285a5d5,
                0x50880866309b7e2c,
                0xa20d1b8c7e881024,
                0x14e4f04fe2db9068,
                0x14e56d3f1564853a,
            ]),
        ]);
        // 1 / ((u+1) ^ (p-1)/2)
        let psi_coeff_y = Fq2([
            Fq([
                0x3e2f585da55c9ad1,
                0x4294213d86c18183,
                0x382844c88b623732,
                0x92ad2afd19103e18,
                0x1d794e4fac7cf0b9,
                0x0bd592fc7d825ec8,
            ]),
            Fq([
                0x7bcfa7a25aa30fda,
                0xdc17dec12a927e7c,
                0x2f088dd86b4ebef1,
                0xd1ca2087da74d4a7,
                0x2da2596696cebc1d,
                0x0e2b7eedbbfd87d2,
            ]),
        ]);

        G2Projective {
            // x = frobenius(x)/((u+1)^((p-1)/3))
            x: self.x.frobenius_map() * psi_coeff_x,
            // y = frobenius(y)/(u+1)^((p-1)/2)
            y: self.y.frobenius_map() * psi_coeff_y,
            // z = frobenius(z)
            z: self.z.frobenius_map(),
        }
    }
}

impl Add for G2Projective {
    type Output = Self;

    fn add(self, rhs: G2Projective) -> Self {
        add_projective_point(self, rhs)
    }
}

impl Neg for G2Projective {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}

impl Sub for G2Projective {
    type Output = Self;

    fn sub(self, rhs: G2Projective) -> Self {
        add_projective_point(self, -rhs)
    }
}

impl Mul<Fr> for G2Projective {
    type Output = G2Projective;

    fn mul(self, rhs: Fr) -> Self::Output {
        scalar_point(self, &rhs)
    }
}

impl Mul<G2Projective> for Fr {
    type Output = G2Projective;

    fn mul(self, rhs: G2Projective) -> Self::Output {
        scalar_point(rhs, &self)
    }
}

/// The coefficient for pairing affine format
#[derive(Debug, Clone, PartialEq, Eq, Copy, Decode, Encode)]
pub struct PairingCoeff(pub(crate) Fq2, pub(crate) Fq2, pub(crate) Fq2);

/// The pairing format coordinate
#[derive(Debug, Clone, Eq, Decode, Encode)]
pub struct G2PairingAffine {
    pub coeffs: Vec<PairingCoeff>,
    is_infinity: bool,
}

impl PartialEq for G2PairingAffine {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs && self.is_infinity == other.is_infinity
    }
}

impl<T> Sum<T> for G2Projective
where
    T: Borrow<G2Projective>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        iter.fold(Self::ADDITIVE_IDENTITY, |acc, item| acc + item.borrow())
    }
}

weierstrass_curve_operation!(
    Fr,
    Fq2,
    G2_PARAM_A,
    G2_PARAM_B,
    B3,
    G2Affine,
    G2Projective,
    G2_GENERATOR_X,
    G2_GENERATOR_Y
);
bls12_g2_pairing!(G2Projective, G2Affine, PairingCoeff, G2PairingAffine, Fq12);

#[cfg(test)]
mod tests {
    use super::curve_test;

    curve_test!(bls12_381, Fr, G2Affine, G2Projective, 50);
}
