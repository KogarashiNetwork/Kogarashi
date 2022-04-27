use sp_std::vec::Vec;
use zero_jubjub::Fr;

pub struct Polynomial {
    coeffs: Vec<Fr>,
}

impl Polynomial {
    fn add(&mut self, rhs: Polynomial) {
        assert_eq!(self.coeffs.len(), rhs.coeffs.len());
        self.coeffs
            .iter_mut()
            .zip(rhs.coeffs.iter())
            .for_each(|(a, b)| a.add_assign(*b))
    }

    fn evaluate(self, at: Fr) -> Fr {
        self.coeffs
            .iter()
            .rev()
            .fold(Fr::zero(), |sum, coeff| sum * at + *coeff)
    }
}

impl From<Vec<Fr>> for Polynomial {
    fn from(coeffs: Vec<Fr>) -> Polynomial {
        Polynomial { coeffs }
    }
}
