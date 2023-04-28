use crate::Fp;
use serde::{Deserialize, Serialize};
use zero_bls12_381::Fr;
use zero_crypto::arithmetic::edwards::*;
use zero_crypto::common::*;
use zero_crypto::*;

pub const EDWARDS_D: Fr = Fr::to_mont_form([
    0x01065fd6d6343eb1,
    0x292d7f6d37579d26,
    0xf5fd9207e6bd7fd4,
    0x2a9318e74bfa2b48,
]);

const X: Fr = Fr::to_mont_form([
    0x4df7b7ffec7beaca,
    0x2e3ebb21fd6c54ed,
    0xf1fbf02d0fd6cce6,
    0x3fd2814c43ac65a6,
]);

const Y: Fr = Fr::to_mont_form([
    0x0000000000000012,
    000000000000000000,
    000000000000000000,
    000000000000000000,
]);

const T: Fr = Fr::to_mont_form([
    0x07b6af007a0b6822b,
    0x04ebe6448d1acbcb8,
    0x036ae4ae2c669cfff,
    0x0697235704b95be33,
]);

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct JubjubAffine {
    x: Fr,
    y: Fr,
}

impl Add for JubjubAffine {
    type Output = JubjubExtended;

    fn add(self, rhs: JubjubAffine) -> Self::Output {
        add_point(self.to_extended(), rhs.to_extended())
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
        add_point(self.to_extended(), rhs.neg().to_extended())
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
        scalar_point(rhs.to_extended(), &self)
    }
}

impl JubjubAffine {
    /// Constructs an JubJubAffine given `x` and `y` without checking
    /// that the point is on the curve.
    pub const fn from_raw_unchecked(x: Fr, y: Fr) -> JubjubAffine {
        JubjubAffine { x, y }
    }
}

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct JubjubExtended {
    x: Fr,
    y: Fr,
    t: Fr,
    z: Fr,
}

impl Add for JubjubExtended {
    type Output = JubjubExtended;

    fn add(self, rhs: JubjubExtended) -> Self::Output {
        add_point(self, rhs)
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

impl Sub for JubjubExtended {
    type Output = JubjubExtended;

    fn sub(self, rhs: JubjubExtended) -> Self::Output {
        add_point(self, rhs.neg())
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
        scalar_point(rhs, &self)
    }
}

twisted_edwards_curve_operation!(Fr, Fr, EDWARDS_D, JubjubAffine, JubjubExtended, X, Y, T);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    curve_test!(jubjub, Fr, JubjubAffine, JubjubExtended, 100);
}

impl Mul<Fp> for JubjubExtended {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: Fp) -> JubjubExtended {
        let mut res = JubjubExtended::ADDITIVE_IDENTITY;
        let mut acc = self;
        for &naf in rhs.to_nafs().iter() {
            if naf == Naf::Plus {
                res += acc;
            } else if naf == Naf::Minus {
                res -= acc;
            }
            acc = acc.double();
        }
        res
    }
}

impl<'a, 'b> Mul<&'b Fp> for &'a JubjubExtended {
    type Output = JubjubExtended;

    #[inline]
    fn mul(self, rhs: &'b Fp) -> JubjubExtended {
        let mut res = JubjubExtended::ADDITIVE_IDENTITY;
        let mut acc = *self;
        for &naf in rhs.to_nafs().iter() {
            if naf == Naf::Plus {
                res += acc;
            } else if naf == Naf::Minus {
                res -= acc;
            }
            acc = acc.double();
        }
        res
    }
}

#[cfg(test)]
mod test {
    use rand_core::OsRng;
    use zero_bls12_381::Fr;
    use zero_crypto::common::{CurveExtended, CurveGroup};

    use crate::{JubjubAffine, JubjubExtended};

    #[test]
    #[allow(clippy::op_ref)]
    fn edwards_operations() {
        let aff1 = JubjubAffine::random(OsRng).to_affine();
        let aff2 = JubjubAffine::random(OsRng).to_affine();
        let mut ext1 = JubjubExtended::random(OsRng);
        let ext2 = JubjubExtended::random(OsRng);
        let scalar = Fr::from(42);

        let _ = aff1 + aff2;
        let _ = &aff1 + &aff2;
        let _ = &aff1 + aff2;
        let _ = aff1 + &aff2;

        let _ = aff1 + ext1;
        let _ = &aff1 + &ext1;
        let _ = &aff1 + ext1;
        let _ = aff1 + &ext1;
        let _ = ext1 + aff1;
        let _ = &ext1 + &aff1;
        let _ = &ext1 + aff1;
        let _ = ext1 + &aff1;

        let _ = ext1 + ext2;
        let _ = &ext1 + &ext2;
        let _ = &ext1 + ext2;
        let _ = ext1 + &ext2;
        ext1 += ext2;
        ext1 += &ext2;
        ext1 += aff2;
        ext1 += &aff2;

        let _ = aff1 - aff2;
        let _ = &aff1 - &aff2;
        let _ = &aff1 - aff2;
        let _ = aff1 - &aff2;

        let _ = aff1 - ext1;
        let _ = &aff1 - &ext1;
        let _ = &aff1 - ext1;
        let _ = aff1 - &ext1;
        let _ = ext1 - aff1;
        let _ = &ext1 - &aff1;
        let _ = &ext1 - aff1;
        let _ = ext1 - &aff1;

        let _ = ext1 - ext2;
        let _ = &ext1 - &ext2;
        let _ = &ext1 - ext2;
        let _ = ext1 - &ext2;
        ext1 -= ext2;
        ext1 -= &ext2;
        ext1 -= aff2;
        ext1 -= &aff2;

        let _ = aff1 * scalar;
        let _ = aff1 * &scalar;
        let _ = &aff1 * scalar;
        let _ = &aff1 * &scalar;
        let _ = scalar * aff1;
        let _ = &scalar * &aff1;
        let _ = scalar * &aff1;
        let _ = &scalar * aff1;

        let _ = ext1 * scalar;
        let _ = ext1 * &scalar;
        let _ = &ext1 * scalar;
        let _ = &ext1 * &scalar;
        let _ = scalar * ext1;
        let _ = &scalar * &ext1;
        let _ = scalar * &ext1;
        let _ = &scalar * ext1;
        ext1 *= scalar;
        ext1 *= &scalar;
    }
}
