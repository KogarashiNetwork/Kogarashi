use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::error::Groth16Error;
use crate::groth16::params::Groth16Params;
use crate::groth16::prover::Prover;
use crate::groth16::Groth16;
use crate::keypair::Keypair;
use core::marker::PhantomData;
use zkstd::common::{vec, CurveGroup, FftField, Group, OsRng, Pairing, Ring, RngCore, Vec};

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
        let powers_of_tau = PowersOfTau::<P>::new(cs.m(), alpha, beta, tau);

        let gamma_inverse = gamma.invert().ok_or(Groth16Error::General)?;
        let delta_inverse = delta.invert().ok_or(Groth16Error::General)?;

        let vk = VerifyingKey::<P> {
            alpha_g1: powers_of_tau.alpha_g1[1],
            beta_g1: powers_of_tau.beta_g1[1],
            beta_g2: powers_of_tau.beta_g2_shift,
            gamma_g2: (g2 * gamma).into(),
            delta_g1: (g1 * delta).into(),
            delta_g2: (g2 * delta).into(),
            ic: vec![],
        };

        todo!()
    }
}

#[derive(Clone)]
pub struct VerifyingKey<P: Pairing> {
    // alpha in g1 for verifying and for creating A/C elements of
    // proof. Never the point at infinity.
    pub alpha_g1: P::G1Affine,

    // beta in g1 and g2 for verifying and for creating B/C elements
    // of proof. Never the point at infinity.
    pub beta_g1: P::G1Affine,
    pub beta_g2: P::G2Affine,

    // gamma in g2 for verifying. Never the point at infinity.
    pub gamma_g2: P::G2Affine,

    // delta in g1/g2 for verifying and proving, essentially the magic
    // trapdoor that forces the prover to evaluate the C element of the
    // proof with only components from the CRS. Never the point at
    // infinity.
    pub delta_g1: P::G1Affine,
    pub delta_g2: P::G2Affine,

    // Elements of the form (beta * u_i(tau) + alpha v_i(tau) + w_i(tau)) / gamma
    // for all public inputs. Because all public inputs have a dummy constraint,
    // this is the same size as the number of inputs, and never contains points
    // at infinity.
    pub ic: Vec<P::G1Affine>,
}

pub struct PowersOfTau<P: Pairing> {
    g1: Vec<P::G1Affine>,
    g2: Vec<P::G2Affine>,
    alpha_g1: Vec<P::G1Affine>,
    beta_g1: Vec<P::G1Affine>,
    beta_g2_shift: P::G2Affine,
}

impl<P: Pairing> PowersOfTau<P> {
    pub fn new(m: usize, alpha: P::ScalarField, beta: P::ScalarField, tau: P::ScalarField) -> Self {
        let mut powers_of_tau = vec![P::ScalarField::zero(); 2 * m - 1];
        let mut current_pow_of_tau = P::ScalarField::one();
        for x in powers_of_tau.iter_mut() {
            *x = current_pow_of_tau;
            current_pow_of_tau *= tau;
        }

        let g1: Vec<P::G1Affine> = powers_of_tau[..=2 * m - 1]
            .iter()
            .map(|t| (P::G1Affine::ADDITIVE_GENERATOR * t).into())
            .collect();

        let g2: Vec<P::G2Affine> = powers_of_tau[..=m]
            .iter()
            .map(|t| (P::G2Affine::ADDITIVE_GENERATOR * t).into())
            .collect();

        let alpha_g1 = g1.iter().take(m).map(|x| (*x * alpha).into()).collect();
        let beta_g1 = g1.iter().take(m).map(|x| (*x * beta).into()).collect();

        Self {
            g1,
            g2,
            alpha_g1,
            beta_g1,
            beta_g2_shift: (P::G2Affine::ADDITIVE_GENERATOR * beta).into(),
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
