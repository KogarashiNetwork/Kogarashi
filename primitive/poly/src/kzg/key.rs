use zkstd::common::{CurveAffine, Decode, Encode, Pairing};

use crate::{msm_curve_addtion, Coefficients, Commitment};

#[derive(Clone, Debug, PartialEq, Decode, Encode, Default)]
pub struct CommitmentKey<C: CurveAffine> {
    pub(crate) bases: Vec<C>,
}

impl<C: CurveAffine> CommitmentKey<C> {
    pub fn trim(&self, truncated_degree: usize) -> Self {
        Self {
            bases: self.bases[..=truncated_degree].to_vec(),
        }
    }

    pub fn commit(&self, coeffs: &Coefficients<C::Scalar>) -> Commitment<C> {
        Commitment::new(msm_curve_addtion(&self.bases, coeffs))
    }
}

/// Evaluation Key is used to verify opening proofs made about a committed
/// polynomial.
#[derive(Clone, Debug, Eq, Decode, Encode, PartialEq, Default)]
pub struct EvaluationKey<P: Pairing> {
    /// Kzg G1 generator
    pub g: P::G1Affine,
    /// Kzg G2 generator
    pub h: P::G2Affine,
    /// \beta times the above generator of G2.
    pub beta_h: P::G2Affine,
    /// The generator of G2, prepared for use in pairings
    pub prepared_h: P::G2PairngRepr,
    /// \beta times the above generator of G2, prepared for use in pairings
    pub prepared_beta_h: P::G2PairngRepr,
}

impl<P: Pairing> EvaluationKey<P> {
    pub fn new(g: P::G1Affine, h: P::G2Affine, beta_h: P::G2Affine) -> Self {
        let prepared_h = P::G2PairngRepr::from(h);
        let prepared_beta_h = P::G2PairngRepr::from(beta_h);
        Self {
            g,
            h,
            beta_h,
            prepared_h,
            prepared_beta_h,
        }
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod test {
    use crate::{Coefficients, KzgParams};
    use bls_12_381::Fr as BlsScalar;
    use ec_pairing::TatePairing;
    use rand_core::OsRng;
    use zkstd::common::Group;

    #[test]
    fn test_basic_commit() {
        let degree = 2;
        let r = BlsScalar::random(OsRng);
        let keypair = KzgParams::<TatePairing>::setup(degree as u64, r);
        let point = BlsScalar::from(10);

        let z_poly = Coefficients::rand(degree, &mut OsRng);

        let witness = keypair.create_witness(&z_poly, point);

        let z_ok = witness.verify();
        assert!(z_ok);
    }
}
