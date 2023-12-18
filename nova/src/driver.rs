use bn_254::{params::PARAM_B3 as BN254_PARAM_B3, Fq, Fr, G1Affine};
use grumpkin::{params::PARAM_B3 as GRUMPKIN_PARAM_B3, Affine};
use num_bigint::{BigInt, Sign};
use zkstd::circuit::CircuitDriver;
use zkstd::common::{IntGroup, PrimeField, Ring};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrumpkinDriver;

impl CircuitDriver for GrumpkinDriver {
    const ORDER_STR: &'static str =
        "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47";
    const BASE_STR: &'static str =
        "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001";
    const NUM_BITS: u16 = 254;
    type Affine = Affine;

    type Base = Fr;

    type Scalar = Fq;

    fn b3() -> Self::Scalar {
        BN254_PARAM_B3
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bn254Driver;

impl CircuitDriver for Bn254Driver {
    const ORDER_STR: &'static str =
        "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001";
    const BASE_STR: &'static str =
        "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47";

    const NUM_BITS: u16 = 254;
    type Affine = G1Affine;

    type Base = Fq;

    type Scalar = Fr;

    fn b3() -> Self::Scalar {
        GRUMPKIN_PARAM_B3
    }
}

/// Convert a field element to a natural number
pub fn f_to_nat<F: PrimeField>(f: &F) -> BigInt {
    BigInt::from_bytes_le(Sign::Plus, &f.to_raw_bytes())
}

/// Convert a natural number to a field element.
pub fn nat_to_f<F: PrimeField>(n: &BigInt) -> F {
    let mut bytes = n.to_signed_bytes_le();
    if bytes.len() > 64 {
        panic!("Length exceed the field size");
    };
    bytes.resize(64, 0);

    let mut res = [0; 64];
    res[0..64].copy_from_slice(&bytes);

    F::from_bytes_wide(&res)
}

/// Compute the limbs encoding a natural number.
/// The limbs are assumed to be based the `limb_width` power of 2.
pub fn nat_to_limbs<F: PrimeField>(nat: &BigInt, limb_width: usize, n_limbs: usize) -> Vec<F> {
    let mask = int_with_n_ones(limb_width);
    let mut nat = nat.clone();
    if nat.bits() as usize <= n_limbs * limb_width {
        (0..n_limbs)
            .map(|_| {
                let r = &nat & &mask;
                nat >>= limb_width as u32;
                nat_to_f(&r)
            })
            .collect()
    } else {
        panic!("Wrong amount of bits");
    }
}

fn int_with_n_ones(n: usize) -> BigInt {
    let mut m = BigInt::from(1);
    m <<= n as u32;
    m -= 1;
    m
}

/// interpret scalar as base
pub fn scalar_as_base<C: CircuitDriver>(input: C::Scalar) -> C::Base {
    let input_bits = input.to_bits();
    let mut mult = C::Base::one();
    let mut val = C::Base::zero();
    for bit in input_bits.iter().rev() {
        if *bit == 1 {
            val += mult;
        }
        mult = mult + mult;
    }
    val
}

/// interpret base as scalar
pub fn base_as_scalar<C: CircuitDriver>(input: C::Base) -> C::Scalar {
    let input_bits = input.to_bits();
    let mut mult = C::Scalar::one();
    let mut val = C::Scalar::zero();
    for bit in input_bits.iter().rev() {
        if *bit == 1 {
            val += mult;
        }
        mult = mult + mult;
    }
    val
}

#[cfg(test)]
mod grumpkin_gadget_tests {
    use super::{Fq as Scalar, Fr as Base, GrumpkinDriver};

    use bn_254::G1Affine;
    use rand_core::OsRng;
    use zkstd::circuit::prelude::{FieldAssignment, PointAssignment, R1cs};
    use zkstd::common::{BNAffine, BNProjective, Group};

    #[test]
    fn range_proof_test() {
        for _ in 0..100 {
            let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
            let mut ncs = cs.clone();
            let bound = Scalar::from(10);

            let x_ass = FieldAssignment::instance(&mut cs, bound);
            let x_bits = FieldAssignment::to_bits(&mut cs, &x_ass);
            FieldAssignment::range_check(&mut cs, &x_bits, bound);
            assert!(cs.is_sat());

            let x_ass = FieldAssignment::instance(&mut ncs, bound + Scalar::one());
            let x_bits = FieldAssignment::to_bits(&mut ncs, &x_ass);
            FieldAssignment::range_check(&mut ncs, &x_bits, bound);
            assert!(!ncs.is_sat());
        }
    }

    #[test]
    fn field_add_test() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let mut ncs = cs.clone();
        let a = Scalar::random(OsRng);
        let b = Scalar::random(OsRng);
        let mut c = a + b;

        // a + b == c
        let x = FieldAssignment::instance(&mut cs, a);
        let y = FieldAssignment::witness(&mut cs, b);
        let z = FieldAssignment::instance(&mut cs, c);
        let sum = &x + &y;
        FieldAssignment::enforce_eq(&mut cs, &z, &sum);

        assert!(cs.is_sat());

        // a + b != c
        c += Scalar::one();
        let x = FieldAssignment::instance(&mut ncs, a);
        let y = FieldAssignment::witness(&mut ncs, b);
        let z = FieldAssignment::instance(&mut ncs, c);
        let sum = &x + &y;
        FieldAssignment::enforce_eq(&mut ncs, &z, &sum);

        assert!(!ncs.is_sat())
    }

    #[test]
    fn field_mul_test() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let mut ncs = cs.clone();
        let a = Scalar::random(OsRng);
        let b = Scalar::random(OsRng);
        let mut c = a * b;

        // a * b == c
        let x = FieldAssignment::instance(&mut cs, a);
        let y = FieldAssignment::witness(&mut cs, b);
        let z = FieldAssignment::instance(&mut cs, c);
        let product = FieldAssignment::mul(&mut cs, &x, &y);
        FieldAssignment::enforce_eq(&mut cs, &z, &product);

        assert!(cs.is_sat());

        // a * b != c
        c += Scalar::one();
        let x = FieldAssignment::instance(&mut ncs, a);
        let y = FieldAssignment::witness(&mut ncs, b);
        let z = FieldAssignment::instance(&mut ncs, c);
        let product = FieldAssignment::mul(&mut ncs, &x, &y);
        FieldAssignment::enforce_eq(&mut ncs, &z, &product);

        assert!(!ncs.is_sat())
    }

    #[test]
    fn field_ops_test() {
        let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
        let mut ncs = cs.clone();
        let input = Scalar::from(3);
        let c = Scalar::from(5);
        let out = Scalar::from(35);

        // x^3 + x + 5 == 35
        let x = FieldAssignment::witness(&mut cs, input);
        let c = FieldAssignment::constant(&c);
        let z = FieldAssignment::instance(&mut cs, out);
        let sym_1 = FieldAssignment::mul(&mut cs, &x, &x);
        let y = FieldAssignment::mul(&mut cs, &sym_1, &x);
        let sym_2 = &y + &x;
        FieldAssignment::enforce_eq(&mut cs, &z, &(&sym_2 + &c));

        assert!(cs.is_sat());

        // x^3 + x + 5 != 36
        let c = Scalar::from(5);
        let out = Scalar::from(36);
        let x = FieldAssignment::witness(&mut ncs, input);
        let c = FieldAssignment::constant(&c);
        let z = FieldAssignment::instance(&mut ncs, out);
        let sym_1 = FieldAssignment::mul(&mut ncs, &x, &x);
        let y = FieldAssignment::mul(&mut ncs, &sym_1, &x);
        let sym_2 = &y + &x;
        FieldAssignment::enforce_eq(&mut ncs, &z, &(&sym_2 + &c));

        assert!(!ncs.is_sat());
    }

    #[test]
    fn curve_double_test() {
        // Affine
        for _ in 0..100 {
            let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
            let point = G1Affine::random(OsRng);

            let circuit_double = PointAssignment::instance(&mut cs, point).double(&mut cs);

            let expected = point.to_extended().double();

            circuit_double.assert_equal_public_point(&mut cs, expected);

            assert!(cs.is_sat());
        }
    }

    #[test]
    fn curve_add_test() {
        // Identity addition test
        {
            let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
            let a = G1Affine::random(OsRng);
            let b = G1Affine::ADDITIVE_IDENTITY;

            let a_assignment = PointAssignment::instance(&mut cs, a);
            let b_assignment = PointAssignment::instance(&mut cs, b);

            let expected = a + b;

            let sum_circuit = a_assignment.add(&b_assignment, &mut cs);

            sum_circuit.assert_equal_public_point(&mut cs, expected);

            assert!(cs.is_sat());
        }

        for _ in 0..100 {
            let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
            let a = G1Affine::random(OsRng);
            let b = G1Affine::random(OsRng);

            let a_assignment = PointAssignment::instance(&mut cs, a);
            let b_assignment = PointAssignment::instance(&mut cs, b);

            let expected = a.to_extended() + b.to_extended();

            let sum_circuit = a_assignment.add(&b_assignment, &mut cs);

            sum_circuit.assert_equal_public_point(&mut cs, expected);

            assert!(cs.is_sat());
        }
    }

    #[test]
    fn curve_scalar_mul_test() {
        for _ in 0..100 {
            let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
            let x = Scalar::random(OsRng);
            let p = G1Affine::random(OsRng);

            let x_assignment = FieldAssignment::instance(&mut cs, x);
            let p_assignment = PointAssignment::instance(&mut cs, p);
            let expected = p * Base::from(x);

            let mul_circuit = p_assignment.scalar_point(&mut cs, &x_assignment);

            mul_circuit.assert_equal_public_point(&mut cs, expected);

            assert!(cs.is_sat());
        }
    }
}
