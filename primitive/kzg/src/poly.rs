use zero_crypto::behave::FftField;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<F>(pub(crate) Vec<F>);

impl<F: FftField> Polynomial<F> {
    pub fn commit(self, domain: Vec<F>) -> F {
        let mut commitment = F::one();
        domain
            .iter()
            .zip(self.0.iter())
            .for_each(|(a, b)| commitment += *a * *b);
        commitment
    }

    pub fn evaluate(self, at: F) -> F {
        self.0
            .iter()
            .rev()
            .fold(F::zero(), |acc, coeff| acc * at + *coeff)
    }

    // divide polynomial with at
    // f(x) - f(at) / x - at
    // pub fn divide(self, at: F) -> F {
    //     let divisor = self.evaluate(at);
    // }
}

#[cfg(test)]
mod tests {
    use super::Polynomial;
    use proptest::prelude::*;
    use rand::SeedableRng;
    use rand_xorshift::XorShiftRng;
    use zero_bls12_381::Fr;
    use zero_crypto::behave::PrimeField;

    prop_compose! {
        fn arb_fr()(bytes in [any::<u8>(); 16]) -> Fr {
            Fr::random(XorShiftRng::from_seed(bytes))
        }
    }

    prop_compose! {
        fn arb_poly(k: u32)(bytes in vec![[any::<u8>(); 16]; 1 << k as usize]) -> Polynomial<Fr> {
            Polynomial((0..(1 << k)).map(|i| Fr::random(XorShiftRng::from_seed(bytes[i]))).collect::<Vec<Fr>>())
        }
    }

    proptest! {
            #![proptest_config(ProptestConfig::with_cases(100))]
            #[test]
        fn polynomial_arithmetic_test(randomness in arb_fr(), at in arb_fr(), polynomial in arb_poly(10)) {
            let k = 10;

            // trusted setup
            let domain = (0..(1 << k)).scan(Fr::one(), |w, _| {
                let tw = *w;
                *w *= randomness;
                Some(tw)
            }).collect::<Vec<_>>();

            // commit polynomial
            let mut commitment = Fr::one();
            polynomial.commit(domain);

            // evaluate polynomial at a
            let evaluation = polynomial.evaluate(at);

            // quotient polynomial
            let quotient = polynomial;
        }
    }
}
