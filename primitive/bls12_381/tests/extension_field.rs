use paste::paste;
use rand_core::OsRng;
use zero_bls12_381::Fq2;
use zero_crypto::behave::PrimeField;

fn arb_ext_fq<F: PrimeField>() -> F {
    F::random(OsRng)
}

macro_rules! extension_field_test {
    ($test_name:ident, $ext_field:ident) => {
        paste! {
            #[test]
            fn [< $test_name _addition_test >]() {
                // a + a = a * 2
                let a = arb_ext_fq::<$ext_field>();
                let b = a + a;
                let c = a.double();

                assert_eq!(b, c);
            }
        }
    };
}

extension_field_test!(fq2_field, Fq2);
