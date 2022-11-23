/// the terminology bellow is aligned with the following paper
/// https://www.iacr.org/archive/asiacrypt2010/6477178/6477178.pdf
use rand_core::RngCore;
use zero_crypto::behave::FftField;

// a_n-1 , a_n-2, ... , a_0
#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<F>(pub(crate) Vec<F>);

pub struct Witness<F> {
    s_eval: F,
    a_eval: F,
    q_eval: F,
    denominator: F,
}

impl<F: FftField> Polynomial<F> {
    // polynomial evaluation domain
    // r^0, r^1, r^2, ..., r^n
    pub fn setup(k: usize, rng: impl RngCore) -> (F, Vec<F>) {
        let randomness = F::random(rng);
        (
            randomness,
            (0..(1 << k))
                .scan(F::one(), |w, _| {
                    let tw = *w;
                    *w *= randomness;
                    Some(tw)
                })
                .collect::<Vec<_>>(),
        )
    }

    // commit polynomial to domain
    pub fn commit(&self, domain: &Vec<F>) -> F {
        assert!(self.0.len() <= domain.len());
        let diff = domain.len() - self.0.len();

        self.0
            .iter()
            .zip(domain.iter().rev().skip(diff))
            .fold(F::zero(), |acc, (a, b)| acc + *a * *b)
    }

    // evaluate polynomial at
    pub fn evaluate(&self, at: F) -> F {
        self.0
            .iter()
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

    // create witness for f(a)
    pub fn create_witness(self, at: F, s: F, domain: Vec<F>) -> Witness<F> {
        // p(x) - p(at) / x - at
        let quotient = self.divide(at);
        // p(s)
        let s_eval = self.evaluate(s);
        // p(at)
        let a_eval = self.evaluate(at);
        // p(s) - p(at) / s - at
        let q_eval = quotient.evaluate(s);
        // s - at
        let denominator = s - at;

        Witness {
            s_eval,
            a_eval,
            q_eval,
            denominator,
        }
    }
}

impl<F: FftField> Witness<F> {
    // verify witness
    pub fn verify_eval(self) -> bool {
        self.q_eval * self.denominator == self.s_eval - self.a_eval
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
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn polynomial_evaluation_test(at in arb_fr(), poly in arb_poly(10)) {
            let mut naive_eval = Fr::zero();
            let mut exp = Fr::one();
            let factor_poly = Polynomial(vec![Fr::one(), -at]);
            let multiple_poly = Polynomial(naive_multiply(poly.clone().0, factor_poly.0.clone()));

            // naive polynomial evaluation
            poly.0.iter().rev().for_each(|coeff| {
                naive_eval += coeff * &exp;
                exp *= at;
            });

            // polynomial evaluation
            let eval = poly.evaluate(at);

            assert_eq!(naive_eval, eval);
            assert_eq!(factor_poly.evaluate(at), Fr::zero());
            assert_eq!(multiple_poly.evaluate(at), Fr::zero());
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]
        #[test]
        fn polynomial_commit_test(bytes in [any::<u8>(); 16], poly_a in arb_poly(8), poly_b in arb_poly(10)) {
            let k = 10;

            // polynomial evaluation domain
            let (randomness, domain) = Polynomial::setup(k, XorShiftRng::from_seed(bytes));

            // polynomial commitment with domain
            let commitment_a = poly_a.commit(&domain);
            let commitment_b = poly_b.commit(&domain);

            // evaluate polynomial with at
            let evaluation_a = poly_a.evaluate(randomness);
            let evaluation_b = poly_b.evaluate(randomness);

            assert_eq!(commitment_a, evaluation_a);
            assert_eq!(commitment_b, evaluation_b);
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
        fn kzg_scheme_test(bytes in [any::<u8>(); 16], at in arb_fr(), mut poly_part in arb_poly(10)) {
            let k = 10;
            let factor_poly = Polynomial(vec![Fr::one(), -at]);
            poly_part.0.remove(poly_part.0.len() - 1);

            // evaluation domain and s
            let (s, domain) = Polynomial::<Fr>::setup(k, XorShiftRng::from_seed(bytes));

            // polynomial to be verified
            let poly = Polynomial(naive_multiply(poly_part.0.clone(), factor_poly.0.clone()));

            // create witness
            let witness = poly.clone().create_witness(at, s, domain);

            // verify witness
            assert!(witness.verify_eval())
        }
    }
}
