use crate::Fp;

pub use bls_12_381::Fr;
use serde::{Deserialize, Serialize};
use zkstd::arithmetic::edwards::*;
use zkstd::common::*;
use zkstd::dress::curve::edwards::*;

/// Twisted Edwards curve Jubjub D params
pub const EDWARDS_D: Fr = Fr::to_mont_form([
    0x01065fd6d6343eb1,
    0x292d7f6d37579d26,
    0xf5fd9207e6bd7fd4,
    0x2a9318e74bfa2b48,
]);

const X: Fr = Fr::to_mont_form([
    0xe4b3d35df1a7adfe,
    0xcaf55d1b29bf81af,
    0x8b0f03ddd60a8187,
    0x62edcbb8bf3787c8,
]);

const Y: Fr = Fr::to_mont_form([
    0x000000000000000b,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
]);

const T: Fr = Fr::to_mont_form([
    0xd3ba1512623479e1,
    0xc6e03c0fcb495697,
    0x2c9c923fdbc2f8a5,
    0x2cdcdf03c0d96e14,
]);

/// Twisted Edwards curve Jubjub affine coordinate
#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct JubjubAffine {
    x: Fr,
    y: Fr,
}

// SBP-M1 review: use safe math operations
impl SigUtils<32> for JubjubAffine {
    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let mut tmp = self.y.to_bytes();
        let x = self.x.to_bytes();
        tmp[31] |= x[0] << 7;

        tmp
    }

    fn from_bytes(mut bytes: [u8; Self::LENGTH]) -> Option<Self> {
        let sign = (bytes[31] >> 7) == 1;
        bytes[31] &= 0b01111111;

        match Fr::from_bytes(bytes) {
            Some(y) => {
                let y2 = y.square();
                let y2_p = y2 * EDWARDS_D + Fr::one();
                let y2_n = y2 - Fr::one();
                match y2_p.invert() {
                    Some(y2_p) => {
                        let y2_n = y2_n * y2_p;

                        match y2_n.sqrt() {
                            Some(mut x) => {
                                if x.is_odd() ^ sign {
                                    x = -x;
                                }
                                Some(Self { x, y })
                            }
                            None => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        }
    }
}

impl JubjubAffine {
    pub const fn from_raw_unchecked(x: Fr, y: Fr) -> JubjubAffine {
        JubjubAffine { x, y }
    }
}

impl Add for JubjubAffine {
    type Output = JubjubExtended;

    fn add(self, rhs: JubjubAffine) -> Self::Output {
        add_affine_point(self, rhs)
    }
}

impl Neg for JubjubAffine {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }
}

impl Sub for JubjubAffine {
    type Output = JubjubExtended;

    fn sub(self, rhs: JubjubAffine) -> Self::Output {
        add_affine_point(self, rhs.neg())
    }
}

impl Mul<Fr> for JubjubAffine {
    type Output = JubjubExtended;

    fn mul(self, rhs: Fr) -> Self::Output {
        scalar_point(self.to_extended(), &rhs)
    }
}

impl Mul<JubjubAffine> for Fr {
    type Output = JubjubExtended;

    fn mul(self, rhs: JubjubAffine) -> Self::Output {
        rhs * self
    }
}

impl Mul<JubjubAffine> for Fp {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: JubjubAffine) -> JubjubExtended {
        &self * &rhs
    }
}

impl<'a, 'b> Mul<&'b JubjubAffine> for &'a Fp {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: &'b JubjubAffine) -> JubjubExtended {
        rhs * self
    }
}

impl Mul<Fp> for JubjubAffine {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: Fp) -> JubjubExtended {
        &self * &rhs
    }
}

impl<'a, 'b> Mul<&'b Fp> for &'a JubjubAffine {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: &'b Fp) -> JubjubExtended {
        let mut res = JubjubExtended::ADDITIVE_IDENTITY;
        for &naf in rhs.to_nafs().iter() {
            res = double_projective_point(res);
            if naf == Naf::Plus {
                res += self;
            } else if naf == Naf::Minus {
                res -= self;
            }
        }
        res
    }
}

/// Twisted Edwards curve Jubjub extended coordinate
#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize, PartialOrd, Ord)]
pub struct JubjubExtended {
    x: Fr,
    y: Fr,
    t: Fr,
    z: Fr,
}

impl Add for JubjubExtended {
    type Output = JubjubExtended;

    fn add(self, rhs: JubjubExtended) -> Self::Output {
        add_projective_point(self, rhs)
    }
}

impl Neg for JubjubExtended {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
            t: -self.t,
            z: self.z,
        }
    }
}

impl SigUtils<32> for JubjubExtended {
    fn to_bytes(self) -> [u8; Self::LENGTH] {
        self.to_affine().to_bytes()
    }

    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        JubjubAffine::from_bytes(bytes).map(|point| point.to_extended())
    }
}

impl Sub for JubjubExtended {
    type Output = JubjubExtended;

    fn sub(self, rhs: JubjubExtended) -> Self::Output {
        add_projective_point(self, rhs.neg())
    }
}

impl Mul<Fr> for JubjubExtended {
    type Output = JubjubExtended;

    fn mul(self, rhs: Fr) -> Self::Output {
        scalar_point(self, &rhs)
    }
}

impl Mul<JubjubExtended> for Fr {
    type Output = JubjubExtended;

    fn mul(self, rhs: JubjubExtended) -> Self::Output {
        rhs * self
    }
}

twisted_edwards_curve_operation!(Fr, Fr, EDWARDS_D, JubjubAffine, JubjubExtended, X, Y, T);

impl Mul<JubjubExtended> for Fp {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: JubjubExtended) -> JubjubExtended {
        &self * &rhs
    }
}

impl<'a, 'b> Mul<&'b JubjubExtended> for &'a Fp {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: &'b JubjubExtended) -> JubjubExtended {
        rhs * self
    }
}

impl Mul<Fp> for JubjubExtended {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: Fp) -> JubjubExtended {
        &self * &rhs
    }
}

impl<'a, 'b> Mul<&'b Fp> for &'a JubjubExtended {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: &'b Fp) -> JubjubExtended {
        let mut res = JubjubExtended::ADDITIVE_IDENTITY;
        for &naf in rhs.to_nafs().iter() {
            res = double_projective_point(res);
            if naf == Naf::Plus {
                res += self;
            } else if naf == Naf::Minus {
                res -= self;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use zkstd::dress::curve::weierstrass::*;

    curve_test!(jubjub, Fr, JubjubAffine, JubjubExtended, 100);

    #[test]
    fn test_serde() {
        for _ in 0..1000 {
            let s = Fr::random(OsRng);
            let point = s * JubjubAffine::ADDITIVE_GENERATOR;
            let bytes = point.to_bytes();
            let point_p = JubjubAffine::from_bytes(bytes).unwrap();

            assert_eq!(point.to_affine(), point_p)
        }
    }
}
