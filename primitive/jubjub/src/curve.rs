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
    0xa79ca00515cf0172,
    0x347eed85e11dd325,
    0x247431ec84468aaa,
    0x464c23a03263d422,
]);

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct JubjubAffine {
    x: Fr,
    y: Fr,
}

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct JubjubExtend {
    x: Fr,
    y: Fr,
    t: Fr,
    z: Fr,
}

twisted_edwards_curve_operation!(Fr, Fr, EDWARDS_D, JubjubAffine, JubjubExtend, X, Y, T);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    curve_test!(bls12_381, Fr, JubjubAffine, JubjubExtend, 100);
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
