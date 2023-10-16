use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::error::Groth16Error;
use crate::groth16::params::Groth16Params;
use crate::groth16::prover::Prover;
use crate::groth16::Groth16;
use crate::keypair::Keypair;
use core::marker::PhantomData;
use poly_commit::Coefficients;
use zkstd::common::{vec, CurveGroup, FftField, Group, OsRng, Pairing, RngCore};

/// Generate the arguments to prove and verify a circuit
pub struct PlonkKey<P: Pairing, C: Circuit<P::JubjubAffine>> {
    c: PhantomData<C>,
    p: PhantomData<P>,
}

impl<P: Pairing, C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>>
    Keypair<P, C> for PlonkKey<P, C>
{
    type PublicParameters = Groth16Params<P>;
    type Prover = Prover<P::ScalarField>;
    type Verifier = ();
    type ConstraintSystem = Groth16<P::JubjubAffine>;

    fn new(pp: &Self::PublicParameters) -> Result<(Self::Prover, Self::Verifier), Error> {
        Self::compile_with_circuit(pp, b"groth16", &C::default())
    }
}

impl<P: Pairing, C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>>
    PlonkKey<P, C>
{
    /// Create a new arguments set from a given circuit instance
    ///
    /// Use the provided circuit instead of the default implementation
    pub fn compile_with_circuit(
        pp: &Groth16Params<P>,
        label: &[u8],
        circuit: &C,
    ) -> Result<
        (
            <Self as Keypair<P, C>>::Prover,
            <Self as Keypair<P, C>>::Verifier,
        ),
        Error,
    > {
        let mut cs = Groth16::initialize();

        circuit.synthesize(&mut cs)?;

        let (alpha, beta, gamma, delta, tau) =
            generate_random_parameters::<P::ScalarField, OsRng>(&mut OsRng);

        let g1 = pp.commitment_key.bases[0];
        let g2 = pp.evaluation_key.h;
        let powers_of_tau = vec![P::ScalarField::zero(); cs.m()];
        let powers_of_tau = Coefficients::new(powers_of_tau);

        let gamma_inverse = gamma.invert().ok_or(Groth16Error::General)?;
        let delta_inverse = delta.invert().ok_or(Groth16Error::General)?;

        let h = vec![P::G1Affine::ADDITIVE_IDENTITY; powers_of_tau.as_ref().len() - 1];

        todo!()
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
