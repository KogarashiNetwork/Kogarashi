use zkstd::arithmetic::bits_256::*;
use zkstd::circuit::CircuitDriver;
use zkstd::common::*;
use zkstd::macros::field::*;

pub(crate) const FR_MODULUS: [u64; 4] = [
    0x43e1f593f0000001,
    0x2833e84879b97091,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

const FR_GENERATOR: [u64; 4] = [7, 0, 0, 0];

pub(crate) const FR_R: [u64; 4] = [
    0xac96341c4ffffffb,
    0x36fc76959f60cd29,
    0x666ea36f7879462e,
    0x0e0a77c19a07df2f,
];

pub(crate) const FR_R2: [u64; 4] = [
    0x1bb8e645ae216da7,
    0x53fe3ab1e35c59e3,
    0x8c49833d53bb8085,
    0x0216d0b17f4e44a5,
];

pub(crate) const FR_R3: [u64; 4] = [
    0x5e94d8e1b4bf0040,
    0x2a489cbe1cfbb6b8,
    0x893cc664a19fcfed,
    0x0cf8594b7fcc657c,
];

pub const FR_INV: u64 = 0xc2e1f593efffffff;

pub const FR_ROOT_OF_UNITY: [u64; 4] = [
    0xd34f1ed960c37c9c,
    0x3215cf6dd39329c8,
    0x98865ea93dd31f74,
    0x03ddb9f5166d18b7,
];

curve_macro!(Fr, FR_GENERATOR, FR_MODULUS, FR_R, FR_R2, FR_R3, FR_INV);

pub(crate) const FR_PARAM_B: Fr = Fr::new_unchecked([
    0xdd7056026000005a,
    0x223fa97acb319311,
    0xcc388229877910c0,
    0x034394632b724eaa,
]);

pub const FR_PARAM_B3: Fr = FR_PARAM_B.add_const(FR_PARAM_B).add_const(FR_PARAM_B);

#[macro_export]
macro_rules! curve_macro {
    ($field:ident, $generator:ident, $modulus:ident, $r:ident, $r2:ident, $r3:ident, $inv:ident) => {
        #[derive(Clone, Copy, Decode, Encode, Serialize, Deserialize)]
        pub struct $field(pub [u64; 4]);

        impl $field {
            pub const fn new_unchecked(val: [u64; 4]) -> Self {
                Self(val)
            }
            pub const fn add_const(self, rhs: Self) -> Self {
                Self(add(self.0, rhs.0, $modulus))
            }

            pub const fn to_mont_form(val: [u64; 4]) -> Self {
                Self(to_mont_form(val, $r2, $modulus, $inv))
            }

            pub const fn inner(&self) -> &[u64; 4] {
                &self.0
            }

            pub(crate) const fn montgomery_reduce(self) -> [u64; 4] {
                mont(
                    [self.0[0], self.0[1], self.0[2], self.0[3], 0, 0, 0, 0],
                    $modulus,
                    $inv,
                )
            }
        }

        impl SigUtils<32> for $field {
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

                let (_, borrow) = sbb(l0, $modulus[0], 0);
                let (_, borrow) = sbb(l1, $modulus[1], borrow);
                let (_, borrow) = sbb(l2, $modulus[2], borrow);
                let (_, borrow) = sbb(l3, $modulus[3], borrow);

                if borrow & 1 == 1 {
                    Some(Self([l0, l1, l2, l3]) * Self($r2))
                } else {
                    None
                }
            }
        }

        prime_field_operation!($field, $modulus, $generator, $inv, $r, $r2, $r3);
    };
}

// #[derive(Clone, Debug, Default, PartialEq, Eq)]
// pub struct GrumpkinDriver;

// impl CircuitDriver for GrumpkinDriver {
//     const NUM_BITS: u16 = 254;
//     type Affine = G1Affine;

//     type Base = Fq;

//     type Scalar = Fr;

//     fn b3() -> Self::Scalar {
//         PARAM_B3
//     }
// }
