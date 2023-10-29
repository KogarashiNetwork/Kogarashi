use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::error::Groth16Error;
use crate::groth16::params::Groth16Params;
use crate::groth16::prover::Prover;
use crate::groth16::verifier::Verifier;
use crate::groth16::Groth16;
use crate::keypair::Keypair;
use core::marker::PhantomData;
use core::ops::MulAssign;
use poly_commit::{Coefficients, Fft, PointsValue};
use rand::rngs::OsRng;
use zkstd::common::{
    vec, CurveGroup, FftField, Group, Pairing, PrimeField, Ring, RngCore, Vec, WeierstrassAffine,
};

/// Generate the arguments to prove and verify a circuit
pub struct Groth16Key<P: Pairing, C: Circuit<P::JubjubAffine>> {
    c: PhantomData<C>,
    p: PhantomData<P>,
}

impl<P: Pairing, C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>>
    Keypair<P, C> for Groth16Key<P, C>
{
    type PublicParameters = Groth16Params<P>;
    type Prover = Prover<P>;
    type Verifier = Verifier<P>;
    type ConstraintSystem = Groth16<P::JubjubAffine>;

    fn compile(pp: &Self::PublicParameters) -> Result<(Self::Prover, Self::Verifier), Error> {
        Self::compile_with_circuit(pp, b"groth16", &C::default())
    }
}

impl<P: Pairing, C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>>
    Groth16Key<P, C>
{
    #[allow(clippy::type_complexity)]
    #[allow(unused_variables)]
    /// Create a new arguments set from a given circuit instance
    ///
    /// Use the provided circuit instead of the default implementation
    pub fn compile_with_circuit(
        pp: &Groth16Params<P>,
        _label: &[u8],
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

        // println!("Cs = {cs:#?}");

        let size = cs.m().next_power_of_two();
        let k = size.trailing_zeros();

        let fft = Fft::<P::ScalarField>::new(k as usize);

        let (alpha, beta, gamma, delta, tau) =
            generate_random_parameters::<P::ScalarField, OsRng>(&mut OsRng);

        let g1 = pp.commitment_key.bases[0];
        println!("G1 = {g1:#?}");
        let g2 = pp.evaluation_key.h;
        let mut powers_of_tau = PointsValue(vec![P::ScalarField::zero(); cs.m()]);

        let gamma_inverse = gamma.invert().ok_or(Groth16Error::General)?;
        let delta_inverse = delta.invert().ok_or(Groth16Error::General)?;

        let mut h: Vec<P::G1Affine> =
            vec![P::G1Affine::ADDITIVE_IDENTITY; powers_of_tau.0.len() - 1];

        let mut current_pow_of_tau = P::ScalarField::one();
        for x in powers_of_tau.0.iter_mut() {
            *x = current_pow_of_tau;
            current_pow_of_tau *= tau;
        }

        let mut coeff = fft.z(&tau);
        coeff.mul_assign(&delta_inverse);

        for (h, p) in h.iter_mut().zip(powers_of_tau.0.iter()) {
            *h = (g1 * (*p * coeff)).into();
        }

        // Use inverse FFT to convert powers of tau to Lagrange coefficients
        let powers_of_tau = fft.idft(powers_of_tau);

        let mut a: Vec<P::G1Affine> =
            vec![P::G1Affine::ADDITIVE_IDENTITY; cs.instance_len() + cs.witness_len()];
        let mut b_g1: Vec<P::G1Affine> =
            vec![P::G1Affine::ADDITIVE_IDENTITY; cs.instance_len() + cs.witness_len()];
        let mut b_g2: Vec<P::G2Affine> =
            vec![P::G2Affine::ADDITIVE_IDENTITY; cs.instance_len() + cs.witness_len()];
        let mut ic: Vec<P::G1Affine> = vec![P::G1Affine::ADDITIVE_IDENTITY; cs.instance_len()];
        let mut l: Vec<P::G1Affine> = vec![P::G1Affine::ADDITIVE_IDENTITY; cs.witness_len()];

        let (at_inputs, bt_inputs, ct_inputs) = cs.inputs_iter();
        let (at_aux, bt_aux, ct_aux) = cs.aux_iter();

        // Evaluate for inputs.
        eval::<P>(
            g1,
            g2,
            &powers_of_tau,
            &at_inputs,
            &bt_inputs,
            &ct_inputs,
            &mut a[0..cs.instance_len()],
            &mut b_g1[0..cs.instance_len()],
            &mut b_g2[0..cs.instance_len()],
            &mut ic,
            &gamma_inverse,
            &alpha,
            &beta,
        );

        // Evaluate for auxiliary variables.
        eval::<P>(
            g1,
            g2,
            &powers_of_tau,
            &at_aux,
            &bt_aux,
            &ct_aux,
            &mut a[cs.instance_len()..],
            &mut b_g1[cs.instance_len()..],
            &mut b_g2[cs.instance_len()..],
            &mut ic,
            &delta_inverse,
            &alpha,
            &beta,
        );

        // // Don't allow any elements be unconstrained, so that
        // // the L query is always fully dense.
        // for e in l.iter() {
        //     if e.is_identity() {
        //         return Err(Groth16Error::General.into());
        //     }
        // }

        let vk = VerifyingKey::<P> {
            alpha_g1: (P::G1Affine::ADDITIVE_GENERATOR * alpha).into(),
            beta_g1: (P::G1Affine::ADDITIVE_GENERATOR * beta).into(),
            beta_g2: (P::G2Affine::ADDITIVE_GENERATOR * beta).into(),
            gamma_g2: (P::G2Affine::ADDITIVE_GENERATOR * gamma).into(),
            delta_g1: (P::G1Affine::ADDITIVE_GENERATOR * delta).into(),
            delta_g2: (P::G2Affine::ADDITIVE_GENERATOR * delta).into(),
            ic,
        };

        println!("Vk = {vk:#?}");

        println!("H = {h:#?}");
        println!("L = {l:#?}");
        println!(
            "A = {:#?}",
            a.iter().filter(|e| !e.is_identity()).collect::<Vec<_>>()
        );
        println!(
            "B_G1 = {:#?}",
            b_g1.iter().filter(|e| !e.is_identity()).collect::<Vec<_>>()
        );
        println!(
            "B_G2 = {:#?}",
            b_g2.iter().filter(|e| !e.is_identity()).collect::<Vec<_>>()
        );

        Ok((
            Prover::<P> {
                constraints: cs.constraints,
                keypair: pp.clone(),
            },
            Verifier::<P> {
                opening_key: pp.evaluation_key.clone(),
            },
        ))
    }
}

#[allow(clippy::too_many_arguments)]
fn eval<P: Pairing>(
    g1: P::G1Affine,
    g2: P::G2Affine,

    // Lagrange coefficients for tau
    powers_of_tau: &Coefficients<P::ScalarField>,

    // QAP polynomials
    at: &[Vec<(P::ScalarField, usize)>],
    bt: &[Vec<(P::ScalarField, usize)>],
    ct: &[Vec<(P::ScalarField, usize)>],

    // Resulting evaluated QAP polynomials
    a: &mut [P::G1Affine],
    b_g1: &mut [P::G1Affine],
    b_g2: &mut [P::G2Affine],
    ext: &mut [P::G1Affine],

    // Inverse coefficient for ext elements
    inv: &P::ScalarField,

    // Trapdoors
    alpha: &P::ScalarField,
    beta: &P::ScalarField,
) {
    for ((((((a, b_g1), b_g2), ext), at), bt), ct) in a
        .iter_mut()
        .zip(b_g1.iter_mut())
        .zip(b_g2.iter_mut())
        .zip(ext.iter_mut())
        .zip(at.iter())
        .zip(bt.iter())
        .zip(ct.iter())
    {
        // Evaluate QAP polynomials at tau
        let mut at = eval_at_tau(powers_of_tau, at);
        let mut bt = eval_at_tau(powers_of_tau, bt);
        let ct = eval_at_tau(powers_of_tau, ct);

        // println!("At = {at:?}\nbt = {bt:?}\nct = {ct:?}");

        // Compute A query (in G1)
        if !at.is_zero() {
            *a = (g1 * at).into();
        }

        // Compute B query (in G1/G2)
        if !bt.is_zero() {
            *b_g1 = (g1 * bt).into();
            *b_g2 = (g2 * bt).into();
        }

        at *= beta;
        bt *= alpha;

        // let point = g1 * ((at + bt + ct) * inv);
        println!("E = {:?}", ((at + bt + ct) * inv));

        *ext = (g1 * ((at + bt + ct) * inv)).into();
    }
}

fn eval_at_tau<F: FftField>(powers_of_tau: &[F], p: &[(F, usize)]) -> F {
    let mut acc = F::zero();

    for &(ref coeff, index) in p {
        acc += powers_of_tau[index] * coeff;
    }
    acc
}

#[derive(Clone, Debug)]
pub struct Parameters<P: Pairing> {
    pub vk: VerifyingKey<P>,

    // Elements of the form ((tau^i * t(tau)) / delta) for i between 0 and
    // m-2 inclusive. Never contains points at infinity.
    pub h: Vec<P::G1Affine>,

    // Elements of the form (beta * u_i(tau) + alpha v_i(tau) + w_i(tau)) / delta
    // for all auxiliary inputs. Variables can never be unconstrained, so this
    // never contains points at infinity.
    pub l: Vec<P::G1Affine>,

    // QAP "A" polynomials evaluated at tau in the Lagrange basis. Never contains
    // points at infinity: polynomials that evaluate to zero are omitted from
    // the CRS and the prover can deterministically skip their evaluation.
    pub a: Vec<P::G1Affine>,

    // QAP "B" polynomials evaluated at tau in the Lagrange basis. Needed in
    // G1 and G2 for C/B queries, respectively. Never contains points at
    // infinity for the same reason as the "A" polynomials.
    pub b_g1: Vec<P::G1Affine>,
    pub b_g2: Vec<P::G2Affine>,
}

#[derive(Clone, Debug)]
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

/// Generates a random common reference string for
/// a circuit.
pub fn generate_random_parameters<F: FftField, R>(mut rng: &mut R) -> (F, F, F, F, F)
where
    R: RngCore,
{
    // let alpha = F::random(&mut rng);
    // let beta = F::random(&mut rng);
    // let gamma = F::random(&mut rng);
    // let delta = F::random(&mut rng);
    // let tau = F::random(&mut rng);

    // let g1 = E::G1::generator();
    // let g2 = E::G2::generator();
    let alpha = F::from(5);
    let beta = F::from(6);
    let gamma = F::from(7);
    let delta = F::from(8);
    let tau = F::from(9);

    (alpha, beta, gamma, delta, tau)
}
