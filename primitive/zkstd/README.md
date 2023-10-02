# ZkStd
[![CI](https://github.com/KogarashiNetwork/zkstd/actions/workflows/ci.yml/badge.svg)](https://github.com/KogarashiNetwork/zkstd/actions/workflows/ci.yml) [![crates.io badge](https://img.shields.io/crates/v/zero-crypto.svg)](https://crates.io/crates/zkstd) [![Documentation](https://docs.rs/zkstd/badge.svg)](https://docs.rs/zkstd) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/zkstd/branch/master/graph/badge.svg?token=801ESOH5ZV)](https://codecov.io/gh/KogarashiNetwork/zkstd) [![dependency status](https://deps.rs/crate/zkstd/latest/status.svg)](https://deps.rs/crate/zkstd/latest)

This crate provides basic cryptographic implementation as in `Field`, `Curve` and `Pairing`, `Fft`, `Kzg`, and also supports fully `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec).

## Design

Cryptography libraries need to be applied optimization easily because computation cost affects users waiting time and on-chain gas cost. We design this library following two perspectives.

- The simplicity to replace with the latest algorithm
- The brevity of code by avoiding duplication

We divide arithmetic operation and interface. Arithmetic operation is concrete logic as in elliptic curve addition and so on, and the interface is trait cryptography primitive supports. And we combine them with macro. With this design, we can keep the finite field and elliptic curve implementation simple.

### Directory Structure

- [arithmetic](./src/arithmetic): the arithmetic operation of limbs, points and bit operation.
- [behave](./src/behave): the interface of cryptography components as in `Fft Field`, `Pairing Field` and so on.
- [dress](./src/dress): the macro used for implementation and in charge of combing `arithmetic` and `behave` together.

## Usage
### Field
The following `Fr` support four basic operation.

```rust
use zkstd::common::*;
use zkstd::behave::*;
use zkstd::dress::field::*;
use zkstd::arithmetic::bits_256::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Decode, Encode, Serialize, Deserialize)]
pub struct Fr(pub [u64; 4]);

impl SigUtils<32> for Fr {
    fn to_bytes(self) -> [u8; Self::LENGTH] {
        let tmp = self.montgomery_reduce();

        let mut res = [0; Self::LENGTH];
        res[0..8].copy_from_slice(&tmp[0].to_le_bytes());
        res[8..16].copy_from_slice(&tmp[1].to_le_bytes());
        res[16..24].copy_from_slice(&tmp[2].to_le_bytes());
        res[24..32].copy_from_slice(&tmp[3].to_le_bytes());

        res
    }

    fn from_bytes(bytes: [u8; Self::LENGTH]) -> Option<Self> {
        // SBP-M1 review: apply proper error handling instead of `unwrap`
        let l0 = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let l1 = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
        let l2 = u64::from_le_bytes(bytes[16..24].try_into().unwrap());
        let l3 = u64::from_le_bytes(bytes[24..32].try_into().unwrap());

        let (_, borrow) = sbb(l0, MODULUS[0], 0);
        let (_, borrow) = sbb(l1, MODULUS[1], borrow);
        let (_, borrow) = sbb(l2, MODULUS[2], borrow);
        let (_, borrow) = sbb(l3, MODULUS[3], borrow);

        if borrow & 1 == 1 {
            Some(Self([l0, l1, l2, l3]) * Self(R2))
        } else {
            None
        }
    }
}

const MODULUS: [u64; 4] = [
    0xffffffff00000001,
    0x53bda402fffe5bfe,
    0x3339d80809a1d805,
    0x73eda753299d7d48,
];

/// Generator of the Scalar field
pub const MULTIPLICATIVE_GENERATOR: Fr = Fr([7, 0, 0, 0]);

const GENERATOR: [u64; 4] = [
    0x0000000efffffff1,
    0x17e363d300189c0f,
    0xff9c57876f8457b0,
    0x351332208fc5a8c4,
];

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x00000001fffffffe,
    0x5884b7fa00034802,
    0x998c4fefecbc4ff5,
    0x1824b159acc5056f,
];

/// R^2 = 2^512 mod r
const R2: [u64; 4] = [
    0xc999e990f3f29c6d,
    0x2b6cedcb87925c23,
    0x05d314967254398f,
    0x0748d9d99f59ff11,
];

/// R^3 = 2^768 mod r
const R3: [u64; 4] = [
    0xc62c1807439b73af,
    0x1b3e0d188cf06990,
    0x73d13c71c7b5f418,
    0x6e2a5bb9c8db33e9,
];

pub const INV: u64 = 0xfffffffeffffffff;

const S: usize = 32;

pub const ROOT_OF_UNITY: Fr = Fr([
    0xb9b58d8c5f0e466a,
    0x5b1b4c801819d7ec,
    0x0af53ae352a31e64,
    0x5bf3adda19e9b27b,
]);

impl Fr {
    pub(crate) const fn montgomery_reduce(self) -> [u64; 4] {
        mont(
            [self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0],
            MODULUS,
            INV,
        )
    }
}

fft_field_operation!(
    Fr,
    MODULUS,
    GENERATOR,
    MULTIPLICATIVE_GENERATOR,
    INV,
    ROOT_OF_UNITY,
    R,
    R2,
    R3,
    S
);

#[cfg(test)]
mod tests {
    use super::*;
    use paste::paste;
    use rand_core::OsRng;

    field_test!(bls12_381_scalar, Fr, 1000);
}
```

### Curve
The following `G1Affine` and `G1Projective` supports point arithmetic.

```rust
use jub_jub::Fp;
use serde::{Deserialize, Serialize};
use bls_12_381::Fr;
use zkstd::arithmetic::edwards::*;
use zkstd::common::*;
use zkstd::dress::curve::edwards::*;

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
        add_projective_point(self.to_extended(), rhs.to_extended())
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
        add_projective_point(self.to_extended(), rhs.neg().to_extended())
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
        scalar_point(rhs, &self)
    }
}

twisted_edwards_curve_operation!(Fr, Fr, EDWARDS_D, JubjubAffine, JubjubExtended, X, Y, T);

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use zkstd::dress::curve::weierstrass::*;

    curve_test!(bls12_381, Fr, G1Affine, G1Projective, 100);
}
```

## Test

```shell
$ cargo test
```
