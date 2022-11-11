# Crypto
This is the primitive of `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec) cryptography libraries.

## Usage
### Field
The following `Fr` support four basic operation.

```rust
use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::common::*;
use zero_crypto::dress::field::*;

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Fr(pub(crate) [u64; 4]);

const MODULUS: [u64; 4] = [
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

const GENERATOR: [u64; 4] = [2, 0, 0, 0];

const IDENTITY: [u64; 4] = [1, 0, 0, 0];

/// R = 2^256 mod r
const R: [u64; 4] = [
    0x25f80bb3b99607d9,
    0xf315d62f66b6e750,
    0x932514eeeb8814f4,
    0x09a6fc6f479155c6,
];

/// R^2 = 2^512 mod r
const R2: [u64; 4] = [
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
];

/// R^3 = 2^768 mod r
const R3: [u64; 4] = [
    0xe0d6c6563d830544,
    0x323e3883598d0f85,
    0xf0fea3004c2e2ba8,
    0x05874f84946737ec,
];

pub const INV: u64 = 0x1ba3a358ef788ef9;

const S: usize = 1;

const ROOT_OF_UNITY: Fr = Fr([
    0xaa9f02ab1d6124de,
    0xb3524a6466112932,
    0x7342261215ac260b,
    0x4d6b87b1da259e2,
]);

fft_field_operation!(
    Fr,
    MODULUS,
    GENERATOR,
    IDENTITY,
    INV,
    ROOT_OF_UNITY,
    R2,
    R3,
    S
);
```

### Curve
The following `JubjubProjective` supports point arithmetic.
```rust
use crate::fr::Fr;
use zero_crypto::arithmetic::bits_256::*;
use zero_crypto::common::*;
use zero_crypto::dress::curve::*;

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct JubjubProjective {
    pub(crate) x: Fr,
    pub(crate) y: Fr,
    pub(crate) z: Fr,
}

const IDENTITY: JubjubProjective = JubjubProjective {
    x: Fr::zero(),
    y: Fr::zero(),
    z: Fr::zero(),
};

const GENERATOR: JubjubProjective = JubjubProjective {
    x: Fr::to_mont_form([
        0x7c24d812779a3316,
        0x72e38f4ebd4070f3,
        0x03b3fe93f505a6f2,
        0xc4c71e5a4102960,
    ]),
    y: Fr::to_mont_form([
        0xd2047ef3463de4af,
        0x01ca03640d236cbf,
        0xd3033593ae386e92,
        0xaa87a50921b80ec,
    ]),
    z: Fr::one(),
};

const PARAM_A: Fr = Fr::zero();

const PARAM_B: Fr = Fr::to_mont_form([4, 0, 0, 0]);

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct JubjubAffine {
    x: Fr,
    y: Fr,
    is_infinity: bool,
}

curve_operation!(
    Fr,
    PARAM_A,
    PARAM_B,
    JubjubAffine,
    JubjubProjective,
    GENERATOR,
    IDENTITY
);
```
