use crate::fp::Fp;
use serde::{Deserialize, Serialize};
use zero_crypto::arithmetic::edwards::*;
use zero_crypto::common::*;
use zero_crypto::*;

const EDWARDS_D: Fp = Fp::to_mont_form([
    0x75eb4dca135978a3,
    0x00700a4d4141d8ab,
    0x8cc740797779e898,
    0x52036cee2b6ffe73,
]);

const X: Fp = Fp::to_mont_form([
    0xc9562d608f25d51a,
    0x692cc7609525a7b2,
    0xc0a4e231fdd6dc5c,
    0x216936d3cd6e53fe,
]);

const Y: Fp = Fp::to_mont_form([
    0x6666666666666658,
    0x6666666666666666,
    0x6666666666666666,
    0x6666666666666666,
]);

const T: Fp = Fp::to_mont_form([
    0x6dde8ab3a5b7dda3,
    0x20f09f80775152f5,
    0x66ea4e8e64abe37d,
    0x67875f0fd78b7665,
]);

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct Ed25519Affine {
    x: Fp,
    y: Fp,
}

impl Add for Ed25519Affine {
    type Output = Ed25519Extend;

    fn add(self, rhs: Ed25519Affine) -> Self::Output {
        Ed25519Extend::from(add_point(self.to_extended(), rhs.to_extended()))
    }
}

impl Neg for Ed25519Affine {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: self.y,
        }
    }
}

impl Sub for Ed25519Affine {
    type Output = Ed25519Extend;

    fn sub(self, rhs: Ed25519Affine) -> Self::Output {
        Ed25519Extend::from(add_point(self.to_extended(), rhs.neg().to_extended()))
    }
}

impl Mul<Fp> for Ed25519Affine {
    type Output = Ed25519Extend;

    fn mul(self, rhs: Fp) -> Self::Output {
        scalar_point(self.to_extended(), &rhs)
    }
}

impl Mul<Ed25519Affine> for Fp {
    type Output = Ed25519Extend;

    fn mul(self, rhs: Ed25519Affine) -> Self::Output {
        scalar_point(rhs.to_extended(), &self)
    }
}

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct Ed25519Extend {
    x: Fp,
    y: Fp,
    t: Fp,
    z: Fp,
}

impl Add for Ed25519Extend {
    type Output = Ed25519Extend;

    fn add(self, rhs: Ed25519Extend) -> Self::Output {
        Ed25519Extend::from(add_point(self, rhs))
    }
}

impl Neg for Ed25519Extend {
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

impl Sub for Ed25519Extend {
    type Output = Ed25519Extend;

    fn sub(self, rhs: Ed25519Extend) -> Self::Output {
        Ed25519Extend::from(add_point(self, rhs.neg()))
    }
}

impl Mul<Fp> for Ed25519Extend {
    type Output = Ed25519Extend;

    fn mul(self, rhs: Fp) -> Self::Output {
        scalar_point(self, &rhs)
    }
}

impl Mul<Ed25519Extend> for Fp {
    type Output = Ed25519Extend;

    fn mul(self, rhs: Ed25519Extend) -> Self::Output {
        scalar_point(rhs, &self)
    }
}

twisted_edwards_curve_operation!(Fp, Fp, EDWARDS_D, Ed25519Affine, Ed25519Extend, X, Y, T);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    curve_test!(ed25519, Fp, Ed25519Affine, Ed25519Extend, 100);
}
