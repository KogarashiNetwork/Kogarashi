#![allow(clippy::suspicious_arithmetic_impl)]
#![allow(clippy::suspicious_op_assign_impl)]

use zero_crypto::{
    common::{CurveExtended, CurveGroup},
    dress::{curve::edwards::*, field::*},
};

macro_rules! field_test_data {
    ($test_data_name:ident, $test_bits:ident, $limbs_type:ident, $modulus:ident, $inv:ident, $r2:ident, $r3:ident) => {
        #[allow(dead_code)]
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

pub mod jubjub_curve {
    use super::*;
    use zero_crypto::arithmetic::bits_256::*;
    use zero_crypto::arithmetic::edwards::*;
    use zero_crypto::common::*;

    #[derive(Clone, Copy, Decode, Encode)]
    pub struct BlsScalar(pub Bits256Limbs);

    pub const BLS_SCALAR_MODULUS: [u64; 4] = [
        0xffffffff00000001,
        0x53bda402fffe5bfe,
        0x3339d80809a1d805,
        0x73eda753299d7d48,
    ];

    pub const BLS_SCALAR_GENERATOR: [u64; 4] = [
        0x0000000efffffff1,
        0x17e363d300189c0f,
        0xff9c57876f8457b0,
        0x351332208fc5a8c4,
    ];

    pub const BLS_SCALAR_MULTIPLICATIVE_GENERATOR: BlsScalar = BlsScalar([7, 0, 0, 0]);

    pub const BLS_SCALAR_R: [u64; 4] = [
        0x00000001fffffffe,
        0x5884b7fa00034802,
        0x998c4fefecbc4ff5,
        0x1824b159acc5056f,
    ];

    pub const BLS_SCALAR_R2: [u64; 4] = [
        0xc999e990f3f29c6d,
        0x2b6cedcb87925c23,
        0x05d314967254398f,
        0x0748d9d99f59ff11,
    ];

    pub const BLS_SCALAR_R3: [u64; 4] = [
        0xc62c1807439b73af,
        0x1b3e0d188cf06990,
        0x73d13c71c7b5f418,
        0x6e2a5bb9c8db33e9,
    ];

    pub const BLS_SCALAR_INV: u64 = 0xfffffffeffffffff;

    pub const S: usize = 32;

    pub const ROOT_OF_UNITY: BlsScalar = BlsScalar([
        0xb9b58d8c5f0e466a,
        0x5b1b4c801819d7ec,
        0x0af53ae352a31e64,
        0x5bf3adda19e9b27b,
    ]);

    pub const EDWARDS_D: BlsScalar = BlsScalar::to_mont_form([
        0x01065fd6d6343eb1,
        0x292d7f6d37579d26,
        0xf5fd9207e6bd7fd4,
        0x2a9318e74bfa2b48,
    ]);

    const X: BlsScalar = BlsScalar::to_mont_form([
        0x4df7b7ffec7beaca,
        0x2e3ebb21fd6c54ed,
        0xf1fbf02d0fd6cce6,
        0x3fd2814c43ac65a6,
    ]);

    const Y: BlsScalar = BlsScalar::to_mont_form([
        0x0000000000000012,
        000000000000000000,
        000000000000000000,
        000000000000000000,
    ]);

    const T: BlsScalar = BlsScalar::to_mont_form([
        0x07b6af007a0b6822b,
        0x04ebe6448d1acbcb8,
        0x036ae4ae2c669cfff,
        0x0697235704b95be33,
    ]);

    impl BlsScalar {
        pub const fn to_mont_form(val: [u64; 4]) -> Self {
            Self(to_mont_form(
                val,
                BLS_SCALAR_R2,
                BLS_SCALAR_MODULUS,
                BLS_SCALAR_INV,
            ))
        }

        pub(crate) const fn montgomery_reduce(self) -> [u64; 4] {
            mont(
                [self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0],
                BLS_SCALAR_MODULUS,
                BLS_SCALAR_INV,
            )
        }
    }

    #[derive(Clone, Copy, Debug, Encode, Decode)]
    pub struct JubjubAffine {
        x: BlsScalar,
        y: BlsScalar,
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

    impl Mul<BlsScalar> for JubjubAffine {
        type Output = JubjubExtended;

        fn mul(self, rhs: BlsScalar) -> Self::Output {
            scalar_point(self.to_extended(), &rhs)
        }
    }

    impl Mul<JubjubAffine> for BlsScalar {
        type Output = JubjubExtended;

        fn mul(self, rhs: JubjubAffine) -> Self::Output {
            scalar_point(rhs.to_extended(), &self)
        }
    }

    #[derive(Clone, Copy, Debug, Encode, Decode)]
    pub struct JubjubExtended {
        x: BlsScalar,
        y: BlsScalar,
        t: BlsScalar,
        z: BlsScalar,
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

    impl Mul<BlsScalar> for JubjubExtended {
        type Output = JubjubExtended;

        fn mul(self, rhs: BlsScalar) -> Self::Output {
            scalar_point(self, &rhs)
        }
    }

    impl Mul<JubjubExtended> for BlsScalar {
        type Output = JubjubExtended;

        fn mul(self, rhs: JubjubExtended) -> Self::Output {
            scalar_point(rhs, &self)
        }
    }
    fft_field_operation!(
        BlsScalar,
        BLS_SCALAR_MODULUS,
        BLS_SCALAR_GENERATOR,
        BLS_SCALAR_MULTIPLICATIVE_GENERATOR,
        BLS_SCALAR_INV,
        ROOT_OF_UNITY,
        BLS_SCALAR_R,
        BLS_SCALAR_R2,
        BLS_SCALAR_R3,
        S
    );

    twisted_edwards_curve_operation!(
        BlsScalar,
        BlsScalar,
        EDWARDS_D,
        JubjubAffine,
        JubjubExtended,
        X,
        Y,
        T
    );
}

pub const JUBJUB_MODULUS: [u64; 4] = [
    0xd0970e5ed6f72cb7,
    0xa6682093ccc81082,
    0x06673b0101343b00,
    0x0e7db4ea6533afa9,
];

pub const JUBJUB_INV: u64 = 0x1ba3a358ef788ef9;

pub const JUBJUB_R2: [u64; 4] = [
    0x67719aa495e57731,
    0x51b0cef09ce3fc26,
    0x69dab7fac026e9a5,
    0x04f6547b8d127688,
];

pub const JUBJUB_R3: [u64; 4] = [
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

pub const BLS12_381_R2: [u64; 6] = [
    0xf4df1f341c341746,
    0x0a76e6a609d104f1,
    0x8de5476c4c95b6d5,
    0x67eb88a9939d83c0,
    0x9a793e85b519952d,
    0x11988fe592cae3aa,
];

pub const BLS12_381_R3: [u64; 6] = [
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
