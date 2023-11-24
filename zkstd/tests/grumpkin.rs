use zkstd::arithmetic::bits_256::*;
use zkstd::arithmetic::weierstrass::*;
use zkstd::circuit::CircuitDriver;
use zkstd::common::*;
use zkstd::macros::curve::weierstrass::*;
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

pub(crate) const FQ_MODULUS: [u64; 4] = [
    0x3c208c16d87cfd47,
    0x97816a916871ca8d,
    0xb85045b68181585d,
    0x30644e72e131a029,
];

pub(crate) const FQ_GENERATOR: [u64; 4] = [3, 0, 0, 0];

/// R = 2^256 mod q
pub(crate) const FQ_R: [u64; 4] = [
    0xd35d438dc58f0d9d,
    0x0a78eb28f5c70b3d,
    0x666ea36f7879462c,
    0x0e0a77c19a07df2f,
];

/// R^2 = 2^512 mod q
pub(crate) const FQ_R2: [u64; 4] = [
    0xf32cfc5b538afa89,
    0xb5e71911d44501fb,
    0x47ab1eff0a417ff6,
    0x06d89f71cab8351f,
];

/// R^3 = 2^768 mod q
pub(crate) const FQ_R3: [u64; 4] = [
    0xb1cd6dafda1530df,
    0x62f210e6a7283db6,
    0xef7f0b0c0ada0afb,
    0x20fd6e902d592544,
];

/// INV = -(q^{-1} mod 2^64) mod 2^64
pub(crate) const FQ_INV: u64 = 0x87d20782e4866389;

#[macro_export]
macro_rules! cycle_pair_field {
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

cycle_pair_field!(Fr, FR_GENERATOR, FR_MODULUS, FR_R, FR_R2, FR_R3, FR_INV);
cycle_pair_field!(Fq, FQ_GENERATOR, FQ_MODULUS, FQ_R, FQ_R2, FQ_R3, FQ_INV);

pub(crate) const FR_PARAM_B: Fr = Fr::new_unchecked([
    0xdd7056026000005a,
    0x223fa97acb319311,
    0xcc388229877910c0,
    0x034394632b724eaa,
]);
pub const FR_PARAM_B3: Fr = FR_PARAM_B.add_const(FR_PARAM_B).add_const(FR_PARAM_B);

pub(crate) const G1_GENERATOR_X: Fq = Fq::one();
pub(crate) const G1_GENERATOR_Y: Fq = Fq::to_mont_form([2, 0, 0, 0]);
pub(crate) const G1_PARAM_B: Fq = Fq::to_mont_form([3, 0, 0, 0]);
pub const FQ_PARAM_B3: Fq = G1_PARAM_B.add_const(G1_PARAM_B).add_const(G1_PARAM_B);

impl From<Fq> for Fr {
    fn from(val: Fq) -> Fr {
        Self(to_mont_form(
            val.montgomery_reduce(),
            FR_R2,
            FR_MODULUS,
            FR_INV,
        ))
    }
}

impl From<Fr> for Fq {
    fn from(val: Fr) -> Fq {
        Self(to_mont_form(
            val.montgomery_reduce(),
            FQ_R2,
            FQ_MODULUS,
            FQ_INV,
        ))
    }
}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Affine {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
    is_infinity: bool,
}

impl Add for G1Affine {
    type Output = G1Projective;

    fn add(self, rhs: G1Affine) -> Self::Output {
        add_affine_point(self, rhs)
    }
}

impl Neg for G1Affine {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            is_infinity: self.is_infinity,
        }
    }
}

impl Sub for G1Affine {
    type Output = G1Projective;

    fn sub(self, rhs: G1Affine) -> Self::Output {
        add_affine_point(self, rhs.neg())
    }
}

impl Mul<Fr> for G1Affine {
    type Output = G1Projective;

    fn mul(self, rhs: Fr) -> Self::Output {
        scalar_point(self.to_extended(), &rhs)
    }
}

impl Mul<G1Affine> for Fr {
    type Output = G1Projective;

    fn mul(self, rhs: G1Affine) -> Self::Output {
        scalar_point(rhs.to_extended(), &self)
    }
}

/// The projective form of coordinate
#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct G1Projective {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
    pub(crate) z: Fq,
}

impl Add for G1Projective {
    type Output = Self;

    fn add(self, rhs: G1Projective) -> Self {
        add_projective_point(self, rhs)
    }
}

impl Neg for G1Projective {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}

impl Sub for G1Projective {
    type Output = Self;

    fn sub(self, rhs: G1Projective) -> Self {
        add_projective_point(self, -rhs)
    }
}

impl Mul<Fr> for G1Projective {
    type Output = G1Projective;

    fn mul(self, rhs: Fr) -> Self::Output {
        scalar_point(self, &rhs)
    }
}

impl Mul<G1Projective> for Fr {
    type Output = G1Projective;

    fn mul(self, rhs: G1Projective) -> Self::Output {
        scalar_point(rhs, &self)
    }
}

weierstrass_curve_operation!(
    Fr,
    Fq,
    G1_PARAM_B,
    FQ_PARAM_B3,
    G1Affine,
    G1Projective,
    G1_GENERATOR_X,
    G1_GENERATOR_Y
);

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Affine {
    pub(crate) x: Fr,
    pub(crate) y: Fr,
    is_infinity: bool,
}
impl Add for Affine {
    type Output = Projective;

    fn add(self, rhs: Affine) -> Self::Output {
        add_affine_point(self, rhs)
    }
}

impl Neg for Affine {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            is_infinity: self.is_infinity,
        }
    }
}

impl Sub for Affine {
    type Output = Projective;

    fn sub(self, rhs: Affine) -> Self::Output {
        add_affine_point(self, rhs.neg())
    }
}

impl Mul<Fq> for Affine {
    type Output = Projective;

    fn mul(self, rhs: Fq) -> Self::Output {
        scalar_point(self.to_extended(), &rhs)
    }
}

impl Mul<Affine> for Fq {
    type Output = Projective;

    fn mul(self, rhs: Affine) -> Self::Output {
        scalar_point(rhs.to_extended(), &self)
    }
}

#[derive(Debug, Clone, Copy, Decode, Encode)]
pub struct Projective {
    pub(crate) x: Fr,
    pub(crate) y: Fr,
    pub(crate) z: Fr,
}

impl Add for Projective {
    type Output = Self;

    fn add(self, rhs: Projective) -> Self {
        add_projective_point(self, rhs)
    }
}

impl Neg for Projective {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: self.x,
            y: -self.y,
            z: self.z,
        }
    }
}

impl Sub for Projective {
    type Output = Self;

    fn sub(self, rhs: Projective) -> Self {
        add_projective_point(self, -rhs)
    }
}

impl Mul<Fq> for Projective {
    type Output = Projective;

    fn mul(self, rhs: Fq) -> Self::Output {
        scalar_point(self, &rhs)
    }
}

impl Mul<Projective> for Fq {
    type Output = Projective;

    fn mul(self, rhs: Projective) -> Self::Output {
        scalar_point(rhs, &self)
    }
}

pub const GENERATOR_X: Fr = Fr::one();
pub const GENERATOR_Y: Fr = Fr::new_unchecked([
    0x11b2dff1448c41d8,
    0x23d3446f21c77dc3,
    0xaa7b8cf435dfafbb,
    0x14b34cf69dc25d68,
]);
pub(crate) const PARAM_B: Fr = Fr::new_unchecked([
    0xdd7056026000005a,
    0x223fa97acb319311,
    0xcc388229877910c0,
    0x034394632b724eaa,
]);
pub const PARAM_B3: Fr = PARAM_B.add_const(PARAM_B).add_const(PARAM_B);

weierstrass_curve_operation!(
    Fq,
    Fr,
    PARAM_B,
    PARAM_B3,
    Affine,
    Projective,
    GENERATOR_X,
    GENERATOR_Y
);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrumpkinDriver;

impl CircuitDriver for GrumpkinDriver {
    const NUM_BITS: u16 = 254;
    type Affine = G1Affine;

    type Base = Fq;

    type Scalar = Fr;

    fn b3() -> Self::Scalar {
        FR_PARAM_B3
    }
}
