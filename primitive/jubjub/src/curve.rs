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

impl DigitalSig for JubjubAffine {
    const LENGTH: usize = 32;
}

impl JubjubAffine {
    pub const fn from_raw_unchecked(x: Fr, y: Fr) -> JubjubAffine {
        JubjubAffine { x, y }
    }

    fn as_bytes(self) -> [u8; Self::LENGTH] {
        let mut tmp = self.x.as_bytes();
        let u = self.y.as_bytes();
        tmp[31] |= u[0] << 7;

        tmp
    }
}

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct JubjubExtend {
    x: Fr,
    y: Fr,
    t: Fr,
    z: Fr,
}

impl JubjubExtend {
    pub fn batch_normalize<'a>(
        y: &'a mut [JubjubExtend],
    ) -> impl Iterator<Item = JubjubAffine> + 'a {
        y.iter().map(|p| JubjubAffine::from(*p))
    }

    pub(crate) fn as_bytes(self) -> [u8; Self::LENGTH] {
        let mut tmp = self.x.as_bytes();
        let u = self.y.as_bytes();
        tmp[31] |= u[0] << 7;

        tmp
    }
}

impl DigitalSig for JubjubExtend {
    const LENGTH: usize = 32;
}

twisted_edwards_curve_operation!(Fr, Fr, EDWARDS_D, JubjubAffine, JubjubExtend, X, Y, T);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    curve_test!(jubjub, Fr, JubjubAffine, JubjubExtend, 100);
}

impl Mul<Fp> for JubjubExtend {
    type Output = JubjubExtend;

    #[inline]
    fn mul(self, rhs: Fp) -> JubjubExtend {
        let mut res = JubjubExtend::ADDITIVE_IDENTITY;
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

impl<'a, 'b> Mul<&'b Fp> for &'a JubjubExtend {
    type Output = JubjubExtend;

    #[inline]
    fn mul(self, rhs: &'b Fp) -> JubjubExtend {
        let mut res = JubjubExtend::ADDITIVE_IDENTITY;
        let mut acc = self.clone();
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
