use crate::{Gate, Witness};
use zkstd::common::{PrimeField, Vec};

#[derive(Default)]
pub struct Builder<F: PrimeField> {
    /// Constraint for each gate
    pub(crate) constraints: Vec<Gate<F>>,

    /// Witness values
    pub(crate) witnesess: Vec<F>,
}

impl<F: PrimeField> Builder<F> {
    fn append_witness(&mut self, witness: F) -> Witness {
        let n = self.witnesess.len();
        self.witnesess.push(witness);
        Witness::new(n)
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;
    use bls_12_381::Fr;

    #[test]
    fn add_test() {
        let a = Fr::one();
        let b = Fr::one();
        let mut builder = Builder::default();
        builder.append_witness(a);
        builder.append_witness(b);
    }
}
