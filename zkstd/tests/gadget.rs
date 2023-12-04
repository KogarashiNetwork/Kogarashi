mod grumpkin;

#[cfg(test)]
mod grumpkin_gadget_tests {
    use crate::grumpkin::{Affine, Fq as Base, Fr as Scalar, GrumpkinDriver};

    use rand_core::OsRng;
    use zkstd::circuit::prelude::{FieldAssignment, PointAssignment, R1cs};
    use zkstd::common::{BNAffine, BNProjective, CurveGroup, Group, PrimeField};

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
        for _ in 0..100 {
            let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
            let point = Affine::random(OsRng);

            let circuit_double = PointAssignment::instance(
                &mut cs,
                point.get_x(),
                point.get_y(),
                point.is_identity(),
            )
            .double(&mut cs);

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
            let a = Affine::random(OsRng);
            let b = Affine::ADDITIVE_IDENTITY;

            let a_assignment =
                PointAssignment::instance(&mut cs, a.get_x(), a.get_y(), a.is_identity());
            let b_assignment =
                PointAssignment::instance(&mut cs, b.get_x(), b.get_y(), b.is_identity());

            let expected = a + b;

            let sum_circuit = a_assignment.add(&b_assignment, &mut cs);

            sum_circuit.assert_equal_public_point(&mut cs, expected);

            assert!(cs.is_sat());
        }

        for _ in 0..100 {
            let mut cs: R1cs<GrumpkinDriver> = R1cs::default();
            let a = Affine::random(OsRng);
            let b = Affine::random(OsRng);

            let a_assignment =
                PointAssignment::instance(&mut cs, a.get_x(), a.get_y(), a.is_identity());
            let b_assignment =
                PointAssignment::instance(&mut cs, b.get_x(), b.get_y(), b.is_identity());

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
            let p = Affine::random(OsRng);

            let x_assignment = FieldAssignment::instance(&mut cs, x); // Fr
            let p_assignment =
                PointAssignment::instance(&mut cs, p.get_x(), p.get_y(), p.is_identity());
            let expected = p * Base::from(x);

            assert_eq!(x.to_bits(), Base::from(x).to_bits());

            let mul_circuit = p_assignment.scalar_point(&mut cs, &x_assignment);

            mul_circuit.assert_equal_public_point(&mut cs, expected);

            assert!(cs.is_sat());
        }
    }
}
