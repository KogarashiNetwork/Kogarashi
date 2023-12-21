mod fr {
    use crate::fr::{INV, MODULUS, R2, S};
    use crate::{Fr, MULTIPLICATIVE_GENERATOR, ROOT_OF_UNITY};
    use ff::derive::subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
    use ff::PrimeField;
    use zkstd::arithmetic::bits_256::*;
    use zkstd::common::*;

    const ROOT_OF_UNITY_INV: Fr = Fr::to_mont_form([
        0x0ed3e50a414e6dba,
        0xb22625f59115aba7,
        0x1bbe587180f34361,
        0x048127174daabc26,
    ]);

    const DELTA: Fr = Fr::to_mont_form([
        0x870e56bbe533e9a2,
        0x5b5f898e5e963f25,
        0x64ec26aad4c86e71,
        0x09226b6e22c6f0ca,
    ]);

    const TWO_INV: Fr = Fr::to_mont_form([
        0xa1f0fac9f8000001,
        0x9419f4243cdcb848,
        0xdc2822db40c0ac2e,
        0x183227397098d014,
    ]);

    impl ConditionallySelectable for Fr {
        fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
            Fr([
                u64::conditional_select(&a.0[0], &b.0[0], choice),
                u64::conditional_select(&a.0[1], &b.0[1], choice),
                u64::conditional_select(&a.0[2], &b.0[2], choice),
                u64::conditional_select(&a.0[3], &b.0[3], choice),
            ])
        }
    }

    impl ConstantTimeEq for Fr {
        fn ct_eq(&self, other: &Self) -> Choice {
            self.0[0].ct_eq(&other.0[0])
                & self.0[1].ct_eq(&other.0[1])
                & self.0[2].ct_eq(&other.0[2])
                & self.0[3].ct_eq(&other.0[3])
        }
    }

    impl ff::Field for Fr {
        const ZERO: Self = Self::zero();
        const ONE: Self = Self::one();

        fn random(mut rng: impl RngCore) -> Self {
            <Self as Group>::random(&mut rng)
        }

        fn square(&self) -> Self {
            <Self as zkstd::common::PrimeField>::square(*self)
        }

        fn double(&self) -> Self {
            <Self as zkstd::common::PrimeField>::double(*self)
        }

        fn invert(&self) -> CtOption<Self> {
            let tmp = <Self as zkstd::common::Group>::invert(*self).unwrap_or_default();
            CtOption::new(tmp, !tmp.ct_eq(&Self::zero()))
        }

        fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
            ff::helpers::sqrt_ratio_generic(num, div)
        }
    }

    impl ff::PrimeField for Fr {
        type Repr = [u8; 32];

        fn from_repr(repr: Self::Repr) -> CtOption<Self> {
            let mut tmp = Fr([0, 0, 0, 0]);

            tmp.0[0] = u64::from_le_bytes(repr[0..8].try_into().unwrap());
            tmp.0[1] = u64::from_le_bytes(repr[8..16].try_into().unwrap());
            tmp.0[2] = u64::from_le_bytes(repr[16..24].try_into().unwrap());
            tmp.0[3] = u64::from_le_bytes(repr[24..32].try_into().unwrap());

            tmp = Fr(to_mont_form(tmp.0, R2, MODULUS, INV));

            CtOption::new(tmp, Choice::from(1))
        }

        fn to_repr(&self) -> Self::Repr {
            let tmp = self.montgomery_reduce();

            let mut res = [0; 32];
            res[0..8].copy_from_slice(&tmp[0].to_le_bytes());
            res[8..16].copy_from_slice(&tmp[1].to_le_bytes());
            res[16..24].copy_from_slice(&tmp[2].to_le_bytes());
            res[24..32].copy_from_slice(&tmp[3].to_le_bytes());

            res
        }

        fn is_odd(&self) -> Choice {
            Choice::from(self.to_repr()[0] & 1)
        }

        const MODULUS: &'static str =
            "0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001";
        const NUM_BITS: u32 = 254;
        const CAPACITY: u32 = 253;
        const TWO_INV: Self = TWO_INV;
        const MULTIPLICATIVE_GENERATOR: Self = MULTIPLICATIVE_GENERATOR;
        const S: u32 = S as u32;
        const ROOT_OF_UNITY: Self = ROOT_OF_UNITY;
        const ROOT_OF_UNITY_INV: Self = ROOT_OF_UNITY_INV;
        const DELTA: Self = DELTA;
    }

    impl ff::PrimeFieldBits for Fr {
        type ReprBits = [u64; 4];

        fn to_le_bits(&self) -> ff::FieldBits<Self::ReprBits> {
            let bytes = self.to_repr();

            let limbs = [
                u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
                u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
                u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
                u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
            ];

            ff::FieldBits::new(limbs)
        }

        fn char_le_bits() -> ::ff::FieldBits<Self::ReprBits> {
            ff::FieldBits::new(MODULUS)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rand_core::OsRng;

        #[test]
        fn field_operations() {
            let mut rng = OsRng;
            let f = Fr::random(&mut rng);
            let inv_ff = <Fr as ff::Field>::invert(&f);
            let inv_zkstd = <Fr as Group>::invert(f);

            if inv_zkstd.is_none() {
                assert_eq!(inv_ff.is_none().unwrap_u8(), 0);
            } else {
                assert_eq!(inv_ff.unwrap(), inv_zkstd.unwrap());
            }
            assert_eq!(Fr::from_repr(f.to_repr()).unwrap(), f);
        }
    }
}

mod fq {
    use crate::fq::{GENERATOR, INV, MODULUS, R2};
    use crate::Fq;
    use ff::derive::subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
    use ff::PrimeField;
    use zkstd::arithmetic::bits_256::*;
    use zkstd::common::*;

    const TWO_INV: Fq = Fq::to_mont_form([
        0x9e10460b6c3e7ea4,
        0xcbc0b548b438e546,
        0xdc2822db40c0ac2e,
        0x183227397098d014,
    ]);

    /// `0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46`
    const ROOT_OF_UNITY: Fq = Fq::to_mont_form([
        0x3c208c16d87cfd46,
        0x97816a916871ca8d,
        0xb85045b68181585d,
        0x30644e72e131a029,
    ]);

    /// `0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd46`
    const ROOT_OF_UNITY_INV: Fq = Fq::to_mont_form([
        0x3c208c16d87cfd46,
        0x97816a916871ca8d,
        0xb85045b68181585d,
        0x30644e72e131a029,
    ]);

    // `0x9`
    const DELTA: Fq = Fq::to_mont_form([0x9, 0, 0, 0]);

    impl ConditionallySelectable for Fq {
        fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
            Fq([
                u64::conditional_select(&a.0[0], &b.0[0], choice),
                u64::conditional_select(&a.0[1], &b.0[1], choice),
                u64::conditional_select(&a.0[2], &b.0[2], choice),
                u64::conditional_select(&a.0[3], &b.0[3], choice),
            ])
        }
    }

    impl ConstantTimeEq for Fq {
        fn ct_eq(&self, other: &Self) -> Choice {
            self.0[0].ct_eq(&other.0[0])
                & self.0[1].ct_eq(&other.0[1])
                & self.0[2].ct_eq(&other.0[2])
                & self.0[3].ct_eq(&other.0[3])
        }
    }

    impl ff::Field for Fq {
        const ZERO: Self = Self::zero();
        const ONE: Self = Self::one();

        fn random(mut rng: impl RngCore) -> Self {
            <Self as Group>::random(&mut rng)
        }

        fn square(&self) -> Self {
            <Self as zkstd::common::PrimeField>::square(*self)
        }

        fn double(&self) -> Self {
            <Self as zkstd::common::PrimeField>::double(*self)
        }

        fn invert(&self) -> CtOption<Self> {
            let tmp = <Self as zkstd::common::Group>::invert(*self).unwrap_or_default();
            CtOption::new(tmp, !tmp.ct_eq(&Self::zero()))
        }

        fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
            ff::helpers::sqrt_ratio_generic(num, div)
        }
    }

    impl ff::PrimeField for Fq {
        type Repr = [u8; 32];

        fn from_repr(repr: Self::Repr) -> CtOption<Self> {
            let mut tmp = Fq([0, 0, 0, 0]);

            tmp.0[0] = u64::from_le_bytes(repr[0..8].try_into().unwrap());
            tmp.0[1] = u64::from_le_bytes(repr[8..16].try_into().unwrap());
            tmp.0[2] = u64::from_le_bytes(repr[16..24].try_into().unwrap());
            tmp.0[3] = u64::from_le_bytes(repr[24..32].try_into().unwrap());

            tmp = Fq(to_mont_form(tmp.0, R2, MODULUS, INV));

            CtOption::new(tmp, Choice::from(1))
        }

        fn to_repr(&self) -> Self::Repr {
            let tmp = self.montgomery_reduce();

            let mut res = [0; 32];
            res[0..8].copy_from_slice(&tmp[0].to_le_bytes());
            res[8..16].copy_from_slice(&tmp[1].to_le_bytes());
            res[16..24].copy_from_slice(&tmp[2].to_le_bytes());
            res[24..32].copy_from_slice(&tmp[3].to_le_bytes());

            res
        }

        fn is_odd(&self) -> Choice {
            Choice::from(self.to_repr()[0] & 1)
        }

        const MODULUS: &'static str =
            "0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47";
        const NUM_BITS: u32 = 254;
        const CAPACITY: u32 = 253;
        const TWO_INV: Self = TWO_INV;
        const MULTIPLICATIVE_GENERATOR: Self = Fq(GENERATOR);
        const S: u32 = 0;
        const ROOT_OF_UNITY: Self = ROOT_OF_UNITY;
        const ROOT_OF_UNITY_INV: Self = ROOT_OF_UNITY_INV;
        const DELTA: Self = DELTA;
    }

    impl ff::PrimeFieldBits for Fq {
        type ReprBits = [u64; 4];

        fn to_le_bits(&self) -> ff::FieldBits<Self::ReprBits> {
            let bytes = self.to_repr();

            let limbs = [
                u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
                u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
                u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
                u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
            ];

            ff::FieldBits::new(limbs)
        }

        fn char_le_bits() -> ::ff::FieldBits<Self::ReprBits> {
            ff::FieldBits::new(MODULUS)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rand_core::OsRng;

        #[test]
        fn field_operations() {
            let mut rng = OsRng;
            let f = Fq::random(&mut rng);
            let inv_ff = <Fq as ff::Field>::invert(&f);
            let inv_zkstd = <Fq as Group>::invert(f);

            if inv_zkstd.is_none() {
                assert_eq!(inv_ff.is_none().unwrap_u8(), 0);
            } else {
                assert_eq!(inv_ff.unwrap(), inv_zkstd.unwrap());
            }
            assert_eq!(Fq::from_repr(f.to_repr()).unwrap(), f);
        }
    }
}
