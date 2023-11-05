use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::params::Groth16Params;
use crate::groth16::prover::Prover;
use crate::groth16::verifier::Verifier;
use crate::groth16::Groth16;
use crate::keypair::Keypair;
use core::marker::PhantomData;
use core::ops::{MulAssign, Neg};
use poly_commit::{Coefficients, Fft, PointsValue};
use zkstd::common::{
    vec, CurveGroup, Group, Pairing, PairingRange, PrimeField, Ring, TwistedEdwardsAffine, Vec,
};

/// Generate the arguments to prove and verify a circuit
pub struct Groth16Key<P: Pairing, A: TwistedEdwardsAffine<Range = P::ScalarField>, C: Circuit<A>> {
    c: PhantomData<C>,
    p: PhantomData<P>,
    a: PhantomData<A>,
}

impl<
        P: Pairing,
        A: TwistedEdwardsAffine<Range = P::ScalarField>,
        C: Circuit<A, ConstraintSystem = Groth16<A>>,
    > Keypair<P, A, C> for Groth16Key<P, A, C>
{
    type PublicParameters = Groth16Params<P>;
    type Prover = Prover<P, A>;
    type Verifier = Verifier<P>;
    type ConstraintSystem = Groth16<A>;

    fn compile(pp: &Self::PublicParameters) -> Result<(Self::Prover, Self::Verifier), Error> {
        Self::compile_with_circuit(pp, b"groth16", &C::default())
    }
}

impl<
        P: Pairing,
        A: TwistedEdwardsAffine<Range = P::ScalarField>,
        C: Circuit<A, ConstraintSystem = Groth16<A>>,
    > Groth16Key<P, A, C>
{
    #[allow(clippy::type_complexity)]
    /// Create a new arguments set from a given circuit instance
    ///
    /// Use the provided circuit instead of the default implementation
    pub fn compile_with_circuit(
        pp: &Groth16Params<P>,
        _label: &[u8],
        circuit: &C,
    ) -> Result<
        (
            <Self as Keypair<P, A, C>>::Prover,
            <Self as Keypair<P, A, C>>::Verifier,
        ),
        Error,
    > {
        let mut cs = Groth16::initialize();

        circuit.synthesize(&mut cs)?;

        let size = cs.m().next_power_of_two();
        let k = size.trailing_zeros();
        let fft = Fft::<P::ScalarField>::new(k as usize);

        let (alpha, beta, gamma, delta, tau) = pp.toxic_waste;

        let g1 = pp.generators.0;
        let g2 = pp.generators.1;

        let gamma_inverse = gamma.invert().ok_or(Error::UnexpectedIdentity)?;
        let delta_inverse = delta.invert().ok_or(Error::UnexpectedIdentity)?;

        let mut h = vec![P::G1Affine::ADDITIVE_IDENTITY; cs.m() - 1];

        // Compute (1, tau, tau^2, ...)
        let mut powers_of_tau = PointsValue(vec![P::ScalarField::zero(); cs.m()]);
        let mut current_pow_of_tau = P::ScalarField::one();
        for x in powers_of_tau.0.iter_mut() {
            *x = current_pow_of_tau;
            current_pow_of_tau *= tau;
        }

        let mut coeff = fft.z(&tau);
        // (tau^m - 1) / delta
        coeff.mul_assign(&delta_inverse);

        // Hide original values by converting to the curve points
        for (h, p) in h.iter_mut().zip(powers_of_tau.0.iter()) {
            *h = (g1 * (*p * coeff)).into();
        }

        // Use inverse FFT to convert powers of tau to Lagrange coefficients
        let powers_of_tau = fft.idft(powers_of_tau);

        let mut a = vec![P::G1Affine::ADDITIVE_IDENTITY; cs.instance_len() + cs.witness_len()];
        let mut b_g1 = vec![P::G1Affine::ADDITIVE_IDENTITY; cs.instance_len() + cs.witness_len()];
        let mut b_g2: Vec<P::G2Affine> =
            vec![P::G2Affine::ADDITIVE_IDENTITY; cs.instance_len() + cs.witness_len()];
        let mut ic = vec![P::G1Affine::ADDITIVE_IDENTITY; cs.instance_len()];
        let mut l = vec![P::G1Affine::ADDITIVE_IDENTITY; cs.witness_len()];

        let ((at_inputs, bt_inputs, ct_inputs), (at_aux, bt_aux, ct_aux)) = cs
            .constraints
            .z_vectors(cs.instance_len(), cs.witness_len());

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
            &mut l,
            &delta_inverse,
            &alpha,
            &beta,
        );

        let vk = VerifyingKey::<P> {
            alpha_g1: (P::G1Affine::ADDITIVE_GENERATOR * alpha).into(),
            beta_g1: (P::G1Affine::ADDITIVE_GENERATOR * beta).into(),
            beta_g2: (P::G2Affine::ADDITIVE_GENERATOR * beta).into(),
            gamma_g2: (P::G2Affine::ADDITIVE_GENERATOR * gamma).into(),
            delta_g1: (P::G1Affine::ADDITIVE_GENERATOR * delta).into(),
            delta_g2: (P::G2Affine::ADDITIVE_GENERATOR * delta).into(),
            ic,
        };

        let params = Parameters {
            vk,
            h,
            l,
            a,
            b_g1,
            b_g2,
        };

        let pvk = params.vk.prepare();

        Ok((
            Prover::<P, A> {
                params,
                _mark: PhantomData,
            },
            Verifier::<P> { vk: pvk },
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
    assert_eq!(a.len(), at.len());
    assert_eq!(a.len(), bt.len());
    assert_eq!(a.len(), ct.len());
    assert_eq!(a.len(), b_g1.len());
    assert_eq!(a.len(), b_g2.len());
    assert_eq!(a.len(), ext.len());
    for ((((((a, b_g1), b_g2), ext), at), bt), ct) in a
        .iter_mut()
        .zip(b_g1.iter_mut())
        .zip(b_g2.iter_mut())
        .zip(ext.iter_mut())
        .zip(at.iter())
        .zip(bt.iter())
        .zip(ct.iter())
    {
        // Evaluate QAP polynomials at tau without
        let mut at = eval_at_tau(powers_of_tau, at);
        let mut bt = eval_at_tau(powers_of_tau, bt);
        let ct = eval_at_tau(powers_of_tau, ct);

        // Compute A query (in G1). Hiding the original values
        if !at.is_zero() {
            *a = (g1 * at).into();
        }

        // Compute B query (in G1/G2). Hiding the original value
        if !bt.is_zero() {
            *b_g1 = (g1 * bt).into();
            *b_g2 = (g2 * bt).into();
        }

        at *= beta;
        bt *= alpha;

        // Compute (beta * u_i(x)+ alpha * v_i(x)+ w_i(x)) / gamma for inputs
        // or  (beta * u_i(x)+ alpha * v_i(x)+ w_i(x)) / delta for aux
        *ext = (g1 * ((at + bt + ct) * inv)).into();
    }
}

fn eval_at_tau<F: PrimeField>(powers_of_tau: &[F], p: &[(F, usize)]) -> F {
    p.iter().fold(F::zero(), |acc, (coeff, index)| {
        acc + powers_of_tau[*index] * coeff
    })
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

impl<P: Pairing> VerifyingKey<P> {
    pub(crate) fn prepare(&self) -> PreparedVerifyingKey<P> {
        let gamma = self.gamma_g2.neg();
        let delta = self.delta_g2.neg();

        PreparedVerifyingKey {
            alpha_g1_beta_g2: P::multi_miller_loop(&[(
                self.alpha_g1,
                P::G2PairngRepr::from(self.beta_g2),
            )])
            .final_exp(),
            neg_gamma_g2: P::G2PairngRepr::from(gamma),
            neg_delta_g2: P::G2PairngRepr::from(delta),
            ic: self.ic.clone(),
        }
    }
}

#[derive(Debug)]
pub struct PreparedVerifyingKey<P: Pairing> {
    /// Pairing result of alpha*beta
    pub(crate) alpha_g1_beta_g2: <P::PairingRange as PairingRange>::Gt,
    /// -gamma in G2
    pub(crate) neg_gamma_g2: P::G2PairngRepr,
    /// -delta in G2
    pub(crate) neg_delta_g2: P::G2PairngRepr,
    /// Copy of IC from `VerifiyingKey`.
    pub(crate) ic: Vec<P::G1Affine>,
}
