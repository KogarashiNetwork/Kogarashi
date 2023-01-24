macro_rules! field_test_data {
    ($test_data_name:ident, $test_bits:ident, $limbs_type:ident, $modulus:ident, $inv:ident, $r2:ident, $r3:ident) => {
        pub mod $test_data_name {
            use super::*;
            use rand_core::RngCore;
            use zero_crypto::arithmetic::$test_bits::*;

            pub const MODULUS: $limbs_type = $modulus;

            pub const INV: u64 = $inv;

            const R2: $limbs_type = $r2;

            const R3: $limbs_type = $r3;

            pub fn random(rand: impl RngCore) -> $limbs_type {
                random_limbs(rand, R2, R3, MODULUS, INV)
            }

            pub const fn from_raw(val: $limbs_type) -> $limbs_type {
                to_mont_form(val, R2, MODULUS, INV)
            }
        }
    };
}

pub const JUBJUB_MODULUS: [u64; 4] = [
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

pub const JUBJUB_INV: u64 = 0x1ba3a358ef788ef9;

const JUBJUB_R2: [u64; 4] = [
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
];

const JUBJUB_R3: [u64; 4] = [
    0xe0d6c6563d830544,
    0x323e3883598d0f85,
    0xf0fea3004c2e2ba8,
    0x05874f84946737ec,
];

pub const BLS12_381_MODULUS: [u64; 6] = [
    0xb9feffffffffaaab,
    0x1eabfffeb153ffff,
    0x6730d2a0f6b0f624,
    0x64774b84f38512bf,
    0x4b1ba7b6434bacd7,
    0x1a0111ea397fe69a,
];

pub const BLS12_381_INV: u64 = 0x89f3fffcfffcfffd;

const BLS12_381_R2: [u64; 6] = [
    0xf4df1f341c341746,
    0x0a76e6a609d104f1,
    0x8de5476c4c95b6d5,
    0x67eb88a9939d83c0,
    0x9a793e85b519952d,
    0x11988fe592cae3aa,
];

const BLS12_381_R3: [u64; 6] = [
    0xed48ac6bd94ca1e0,
    0x315f831e03a7adf8,
    0x9a53352a615e29dd,
    0x34c04e5e921e1761,
    0x2512d43565724728,
    0x0aa6346091755d4d,
];

pub(crate) type Bits256Limbs = [u64; 4];
pub(crate) type Bits384Limbs = [u64; 6];

field_test_data!(
    jubjub_field,
    bits_256,
    Bits256Limbs,
    JUBJUB_MODULUS,
    JUBJUB_INV,
    JUBJUB_R2,
    JUBJUB_R3
);

field_test_data!(
    bls12_381_field,
    bits_384,
    Bits384Limbs,
    BLS12_381_MODULUS,
    BLS12_381_INV,
    BLS12_381_R2,
    BLS12_381_R3
);
