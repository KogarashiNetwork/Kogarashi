use crate::public_params::PublicParameters;
use zkstd::common::*;

/// Kate polynomial commitment params used for prover polynomial domain and proof verification
#[derive(Clone, Debug, PartialEq, Decode, Encode, Default)]
pub struct Groth16Params<P: Pairing> {
    pub(crate) generators: (P::G1Affine, P::G2Affine),
    pub(crate) toxic_waste: (
        P::ScalarField,
        P::ScalarField,
        P::ScalarField,
        P::ScalarField,
        P::ScalarField,
    ),
}

impl<P: Pairing> PublicParameters<P> for Groth16Params<P> {
    const ADDITIONAL_DEGREE: usize = 0;

    /// setup polynomial evaluation domain
    fn setup(_k: u64, mut r: impl RngCore) -> Self {
        Self {
            generators: (
                P::G1Affine::ADDITIVE_GENERATOR,
                P::G2Affine::ADDITIVE_GENERATOR,
            ),
            toxic_waste: generate_random_parameters(&mut r),
        }
    }
}

/// Generates a random common reference string for
/// a circuit.
pub fn generate_random_parameters<F: FftField, R>(mut rng: &mut R) -> (F, F, F, F, F)
where
    R: RngCore,
{
    let alpha = F::random(&mut rng);
    let beta = F::random(&mut rng);
    let gamma = F::random(&mut rng);
    let delta = F::random(&mut rng);
    let tau = F::random(&mut rng);

    (alpha, beta, gamma, delta, tau)
}
