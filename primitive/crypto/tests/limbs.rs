mod test_data;

use rand_core::OsRng;
use test_data::{bls12_381_field, jubjub_field};

#[cfg(test)]
mod jubjub_limbs_tests {
    use super::*;
    use crate::jubjub_field::*;
    use zero_crypto::arithmetic::bits_256::*;

    fn arb_jubjub_fr() -> [u64; 4] {
        random(OsRng)
    }

    #[test]
    fn jubjub_field_add_test() {
        let a = arb_jubjub_fr();
        let b = a;
        let c = a;

        // a + a = a * 2
        let d = add(a, b, MODULUS);
        let e = double(c, MODULUS);
        assert_eq!(d, e);
    }

    #[test]
    fn jubjub_field_sub_test() {
        let a = arb_jubjub_fr();
        let b = a;
        let c = a;
        let d = a;

        // a - a = a * 2 - a * 2
        let e = sub(a, b, MODULUS);

        let cc = double(c, MODULUS);
        let dd = double(d, MODULUS);
        let f = sub(cc, dd, MODULUS);

        assert_eq!(e, f);
    }

    #[test]
    fn jubjub_field_mul_test() {
        let a = arb_jubjub_fr();
        let b = arb_jubjub_fr();
        let c = arb_jubjub_fr();

        // a * b + a * c
        let ab = mul(a, b, MODULUS, INV);
        let ac = mul(a, c, MODULUS, INV);
        let d = add(ab, ac, MODULUS);

        // a * (b + c)
        let bc = add(b, c, MODULUS);
        let e = mul(a, bc, MODULUS, INV);

        assert_eq!(d, e);
    }

    #[test]
    fn jubjub_field_square_test() {
        let a = arb_jubjub_fr();
        let b = arb_jubjub_fr();

        // (a * a) * (b * b)
        let aa = mul(a, a, MODULUS, INV);
        let bb = mul(b, b, MODULUS, INV);
        let c = mul(aa, bb, MODULUS, INV);

        // a^2 * b^2
        let aa = square(a, MODULUS, INV);
        let bb = square(b, MODULUS, INV);
        let d = mul(aa, bb, MODULUS, INV);

        assert_eq!(c, d);
    }

    #[test]
    fn jubjub_field_invert_test() {
        let a = arb_jubjub_fr();
        let one = from_raw([1, 0, 0, 0]);
        let inv = invert(a, sub(zero(), [2, 0, 0, 0], MODULUS), one, MODULUS, INV);

        if let Some(x) = inv {
            let b = mul(a, x, MODULUS, INV);
            assert_eq!(b, one)
        }
    }

    #[test]
    fn jubjub_field_power_test() {
        let a = arb_jubjub_fr();
        let one = from_raw([1, 0, 0, 0]);
        let identity = pow(a, sub(zero(), [1, 0, 0, 0], MODULUS), one, MODULUS, INV);
        let zero_power = pow(a, [0, 0, 0, 0], one, MODULUS, INV);

        assert_eq!(one, identity);
        assert_eq!(one, zero_power);
    }
}

#[cfg(test)]
mod bls12_381_limbs_tests {
    use super::*;
    use crate::bls12_381_field::*;
    use zero_crypto::arithmetic::bits_384::*;

    fn arb_bls12_381_fp() -> [u64; 6] {
        random(OsRng)
    }

    #[test]
    fn bls12_381_field_add_test() {
        let a = arb_bls12_381_fp();
        let b = a;
        let c = a;

        // a + a = a * 2
        let d = add(a, b, MODULUS);
        let e = double(c, MODULUS);
        assert_eq!(d, e);
    }

    #[test]
    fn bls12_381_field_sub_test() {
        let a = arb_bls12_381_fp();
        let b = a;
        let c = a;
        let d = a;

        // a - a = a * 2 - a * 2
        let e = sub(a, b, MODULUS);

        let cc = double(c, MODULUS);
        let dd = double(d, MODULUS);
        let f = sub(cc, dd, MODULUS);

        assert_eq!(e, f);
    }

    #[test]
    fn bls12_381_field_mul_test() {
        let a = arb_bls12_381_fp();
        let b = arb_bls12_381_fp();
        let c = arb_bls12_381_fp();

        // a * b + a * c
        let ab = mul(a, b, MODULUS, INV);
        let ac = mul(a, c, MODULUS, INV);
        let d = add(ab, ac, MODULUS);

        // a * (b + c)
        let bc = add(b, c, MODULUS);
        let e = mul(a, bc, MODULUS, INV);

        assert_eq!(d, e);
    }

    #[test]
    fn bls12_381_field_square_test() {
        let a = arb_bls12_381_fp();
        let b = arb_bls12_381_fp();

        // (a * a) * (b * b)
        let aa = mul(a, a, MODULUS, INV);
        let bb = mul(b, b, MODULUS, INV);
        let c = mul(aa, bb, MODULUS, INV);

        // a^2 * b^2
        let aa = square(a, MODULUS, INV);
        let bb = square(b, MODULUS, INV);
        let d = mul(aa, bb, MODULUS, INV);

        assert_eq!(c, d);
    }

    #[test]
    fn bls12_381_field_invert_test() {
        let a = arb_bls12_381_fp();
        let one = from_raw([1, 0, 0, 0, 0, 0]);
        let little_fermat = sub(MODULUS, [2, 0, 0, 0, 0, 0], MODULUS);
        let inv = invert(a, little_fermat, one, MODULUS, INV);

        if let Some(x) = inv {
            let b = mul(a, x, MODULUS, INV);
            assert_eq!(b, one)
        }
    }

    #[test]
    fn bls12_381_field_power_test() {
        let a = arb_bls12_381_fp();
        let one = from_raw([1, 0, 0, 0, 0, 0]);
        let identity = pow(
            a,
            sub(zero(), [1, 0, 0, 0, 0, 0], MODULUS),
            one,
            MODULUS,
            INV,
        );
        let zero_power = pow(a, [0, 0, 0, 0, 0, 0], one, MODULUS, INV);

        assert_eq!(one, identity);
        assert_eq!(one, zero_power);
    }
}
