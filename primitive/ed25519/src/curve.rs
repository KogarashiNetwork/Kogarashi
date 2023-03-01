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
    0x0000000000000009,
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
]);

const Y: Fp = Fp::to_mont_form([
    0xa81b1ae2a7ef57d6,
    0x580ba29ce48e9f34,
    0x0000000000000004,
    0x0000000000000000,
]);

const T: Fp = Fp::to_mont_form([
    0xe8f3f1f7e76a1686,
    0x1868b784090398d9,
    0x0000000000000027,
    0x0000000000000000,
]);

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct Ed25519Affine {
    x: Fp,
    y: Fp,
}

#[derive(Clone, Copy, Debug, Encode, Decode, Deserialize, Serialize)]
pub struct Ed25519Extend {
    x: Fp,
    y: Fp,
    t: Fp,
    z: Fp,
}

twisted_edwards_curve_operation!(Fp, Fp, EDWARDS_D, Ed25519Affine, Ed25519Extend, X, Y, T);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    curve_test!(ed25519, Fp, Ed25519Affine, Ed25519Extend, 100);
}
