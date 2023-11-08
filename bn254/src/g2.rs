use crate::fq::Fq;
use crate::fqn::Fq2;
use crate::fr::Fr;
use crate::pairing::{SIX_U_PLUS_2_NAF, XI_TO_Q_MINUS_1_OVER_2};
use crate::params::*;
use core::borrow::Borrow;
use core::iter::Sum;
use zkstd::arithmetic::weierstrass::*;
use zkstd::common::*;
use zkstd::macros::curve::weierstrass::*;

const B: Fq2 = Fq2([
    Fq::to_mont_form([
        0x3267e6dc24a138e5,
        0xb5b4c5e559dbefa3,
        0x81be18991be06ac3,
        0x2b149d40ceb8aaae,
    ]),
    Fq::to_mont_form([
        0xe4a2bd0685c315d2,
        0xa74fa084e52d1852,
        0xcd2cafadeed8fdf4,
        0x009713b03af0fed4,
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

        res[0..32].copy_from_slice(&x.0[1].to_bytes()[..]);
        res[32..96].copy_from_slice(&x.0[0].to_bytes()[..]);

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
            let mut tmp = [0; 32];
            tmp.copy_from_slice(&buf[0..32]);

            // Mask away the flag bits
            tmp[0] &= 0b0001_1111;

            Fq::from_bytes(tmp)
        };
        let xc0 = {
            let mut tmp = [0; 32];
            tmp.copy_from_slice(&buf[32..96]);

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
        let mut x = BN_X >> 1;
        let mut acc = *self;
        while x != 0 {
            acc = acc.double();
            if x % 2 == 1 {
                xself += acc;
            }
            x >>= 1;
        }
        // finally, flip the sign
        if BN_X_IS_NEGATIVE {
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
            ]),
        ]);
        // 1 / ((u+1) ^ (p-1)/2)
        let psi_coeff_y = Fq2([
            Fq([
                0x3e2f585da55c9ad1,
                0x4294213d86c18183,
                0x382844c88b623732,
                0x92ad2afd19103e18,
            ]),
            Fq([
                0x7bcfa7a25aa30fda,
                0xdc17dec12a927e7c,
                0x2f088dd86b4ebef1,
                0xd1ca2087da74d4a7,
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
#[derive(Debug, Clone, Eq, Decode, Encode, Default)]
pub struct G2PairingAffine {
    pub coeffs: Vec<PairingCoeff>,
    is_infinity: bool,
}

impl PartialEq for G2PairingAffine {
    fn eq(&self, other: &Self) -> bool {
        self.coeffs == other.coeffs && self.is_infinity == other.is_infinity
    }
}

impl ParityCmp for PairingCoeff {}
impl ParityCmp for G2PairingAffine {}

impl G2Projective {
    pub(crate) fn double_eval(&mut self) -> PairingCoeff {
        // Adaptation of Algorithm 26, https://eprint.iacr.org/2010/354.pdf
        let tmp0 = self.x.square();
        let tmp1 = self.y.square();
        let tmp2 = tmp1.square();
        let tmp3 = (tmp1 + self.x).square() - tmp0 - tmp2;
        let tmp3 = tmp3.double();
        let tmp4 = tmp0.double() + tmp0;
        let tmp6 = self.x + tmp4;
        let tmp5 = tmp4.square();
        let zsquared = self.z.square();
        self.x = tmp5 - tmp3.double();
        self.z = (self.z + self.y).square() - tmp1 - zsquared;
        self.y = (tmp3 - self.x) * tmp4 - tmp2.double().double().double();
        let tmp3 = -(tmp4 * zsquared).double();
        let tmp6 = tmp6.square() - tmp0 - tmp5;
        let tmp1 = tmp1.double().double();
        let tmp6 = tmp6 - tmp1;
        let tmp0 = self.z * zsquared;
        let tmp0 = tmp0.double();

        PairingCoeff(tmp0, tmp3, tmp6)
    }

    pub(crate) fn add_eval(&mut self, rhs: G2Affine) -> PairingCoeff {
        // Adaptation of Algorithm 27, https://eprint.iacr.org/2010/354.pdf
        let zsquared = self.z.square();
        let ysquared = rhs.y.square();
        let t0 = zsquared * rhs.x;
        let t1 = ((rhs.y + self.z).square() - ysquared - zsquared) * zsquared;
        let t2 = t0 - self.x;
        let t3 = t2.square();
        let t4 = t3.double().double();
        let t5 = t4 * t2;
        let t6 = t1 - self.y.double();
        let t9 = t6 * rhs.x;
        let t7 = t4 * self.x;
        self.x = t6.square() - t5 - t7.double();
        self.z = (self.z + t2).square() - zsquared - t3;
        let t10 = rhs.y + self.z;
        let t8 = (t7 - self.x) * t6;
        let t0 = self.y * t5;
        self.y = t8 - t0.double();
        let t10 = t10.square() - ysquared;
        let ztsquared = self.z.square();
        let t10 = t10 - ztsquared;
        let t9 = t9.double() - t10;
        let t10 = self.z.double();
        let t1 = -t6.double();

        PairingCoeff(t10, t1, t9)
    }
}

impl From<G2Affine> for G2PairingAffine {
    fn from(g2: G2Affine) -> G2PairingAffine {
        if g2.is_identity() {
            Self {
                coeffs: vec![],
                is_infinity: true,
            }
        } else {
            let mut coeffs = vec![];
            let mut g2_projective = G2Projective::from(g2);
            let neg = -g2;

            for i in (1..SIX_U_PLUS_2_NAF.len()).rev() {
                coeffs.push(g2_projective.double_eval());
                let x = SIX_U_PLUS_2_NAF[i - 1];
                match x {
                    1 => {
                        coeffs.push(g2_projective.add_eval(g2));
                    }
                    -1 => {
                        coeffs.push(g2_projective.add_eval(neg));
                    }
                    _ => continue,
                }
            }

            let mut q = g2;

            q.x.0[1] = -q.x.0[1];
            q.x *= FROBENIUS_COEFF_FQ6_C1[1];

            q.y.0[1] = -q.y.0[1];
            q.y *= XI_TO_Q_MINUS_1_OVER_2;

            coeffs.push(g2_projective.add_eval(q));

            let mut minusq2 = g2;
            minusq2.x *= FROBENIUS_COEFF_FQ6_C1[2];

            coeffs.push(g2_projective.add_eval(minusq2));

            Self {
                coeffs,
                is_infinity: false,
            }
        }
    }
}

impl G2PairingAffine {
    pub fn is_identity(&self) -> bool {
        self.is_infinity
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

#[cfg(test)]
mod tests {
    use super::curve_test;
    use rand_core::OsRng;

    curve_test!(bn254, Fr, G2Affine, G2Projective, 50);
}
