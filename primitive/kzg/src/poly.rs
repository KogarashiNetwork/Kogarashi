use zero_crypto::behave::FftField;

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<F>(pub(crate) Vec<F>);

impl<F: FftField> Polynomial<F> {
    pub fn commit(&self, domain: Vec<F>) -> F {
        assert_eq!(self.0.len(), domain.len());

        domain
            .iter()
            .rev()
            .zip(self.0.iter())
            .fold(F::zero(), |acc, (a, b)| acc + *a * *b)
    }

    pub fn evaluate(&self, at: F) -> F {
        self.0
            .iter()
            .rev()
            .fold(F::zero(), |acc, coeff| acc * at + *coeff)
    }

    // no remainder polynomial division with at
    // f(x) - f(at) / x - at
    pub fn divide(&self, at: F) -> Self {
        Self(
            self.0
                .iter()
                .skip(1)
                .scan(self.0[0], |w, coeff| {
                    let tmp = *w;
                    *w *= at;
                    *w += *coeff;
                    Some(tmp)
                })
                .collect::<Vec<_>>(),
        )
    }
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

    fn naive_multiply<F: PrimeField>(a: Vec<F>, b: Vec<F>) -> Vec<F> {
        let mut c = vec![F::default(); a.len() + b.len() - 1];
        a.iter().enumerate().for_each(|(i_a, coeff_a)| {
            b.iter().enumerate().for_each(|(i_b, coeff_b)| {
                c[i_a + i_b] += *coeff_a * *coeff_b;
            })
        });
        c
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1))]
        #[test]
        fn polynomial_evaluation_test(at in arb_fr(), poly in arb_poly(10)) {
            let mut naive_eval = Fr::zero();
            let mut exp = Fr::one();

            // naive polynomial evaluation
            poly.0.iter().for_each(|coeff| {
                naive_eval += coeff * &exp;
                exp *= at;
            });

            // polynomial evaluation
            let eval = poly.evaluate(at);

            assert_eq!(naive_eval, eval);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn polynomial_commit_and_evaluation_test(randomness in arb_fr(), poly in arb_poly(10)) {
            let k = 10;

            // polynomial evaluation domain
            // r^0, r^1, r^2, ..., r^n
            let domain = (0..(1 << k)).scan(Fr::one(), |w, _| {
                let tw = *w;
                *w *= randomness;
                Some(tw)
            }).collect::<Vec<_>>();

            // polynomial commitment with domain
            let commitment = poly.commit(domain);

            // evaluate polynomial with at
            let evaluation = poly.evaluate(randomness);

            // assert_eq!(commitment, evaluation);
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn polynomial_division_test(at in arb_fr(), divisor in arb_poly(10)) {
            // dividend = divisor * quotient
            let factor_poly = vec![Fr::one(), -at];

            // divisor * (x - at) = dividend
            let poly_a = Polynomial(naive_multiply(divisor.0, factor_poly.clone()));

            // dividend / (x - at) = quotient
            let quotient = poly_a.divide(at);

            // quotient * (x - at) = divident
            let original = Polynomial(naive_multiply(quotient.0, factor_poly));

            assert_eq!(poly_a.0, original.0);
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
