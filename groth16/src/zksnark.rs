use crate::circuit::Circuit;
use crate::error::Error;
use crate::fft::Fft;
use crate::params::Parameters;
use crate::poly::{Coefficients, PointsValue};
use crate::prover::Prover;
use crate::verifier::{Verifier, VerifyingKey};

use bls_12_381::{Fr, G1Affine, G2Affine};
use r1cs::R1cs;
use zkstd::common::{vec, Group, MulAssign, PrimeField, RngCore, Vec};

/// Generate the arguments to prove and verify a circuit
pub struct ZkSnark {}

impl ZkSnark {
    pub fn setup<C: Circuit>(mut r: impl RngCore) -> Result<(Prover, Verifier), Error> {
        let circuit = C::default();
        let mut cs = R1cs::default();

        circuit.synthesize(&mut cs)?;

        let size = cs.m().next_power_of_two();
        let k = size.trailing_zeros();
        let fft = Fft::<Fr>::new(k as usize);

        // toxic waste
        let alpha = Fr::random(&mut r);
        let beta = Fr::random(&mut r);
        let gamma = Fr::random(&mut r);
        let delta = Fr::random(&mut r);
        let tau = Fr::random(&mut r);

        let g1 = G1Affine::ADDITIVE_GENERATOR;
        let g2 = G2Affine::ADDITIVE_GENERATOR;

        let gamma_inverse = gamma.invert().ok_or(Error::ProverInversionFailed)?;
        let delta_inverse = delta.invert().ok_or(Error::ProverInversionFailed)?;

        let mut h = vec![G1Affine::ADDITIVE_IDENTITY; cs.m() - 1];

        // Compute (1, tau, tau^2, ...)
        let mut powers_of_tau = PointsValue(vec![Fr::zero(); cs.m()]);
        let mut current_pow_of_tau = Fr::one();
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

        let mut a = vec![G1Affine::ADDITIVE_IDENTITY; cs.l() + cs.m_l_1()];
        let mut b_g1 = vec![G1Affine::ADDITIVE_IDENTITY; cs.l() + cs.m_l_1()];
        let mut b_g2: Vec<G2Affine> = vec![G2Affine::ADDITIVE_IDENTITY; cs.l() + cs.m_l_1()];
        let mut ic = vec![G1Affine::ADDITIVE_IDENTITY; cs.l()];
        let mut l = vec![G1Affine::ADDITIVE_IDENTITY; cs.m_l_1()];

        let ((at_inputs, bt_inputs, ct_inputs), (at_aux, bt_aux, ct_aux)) =
            cs.z_vectors(cs.l(), cs.m_l_1());

        // Evaluate for inputs.
        eval(
            g1,
            g2,
            &powers_of_tau,
            &at_inputs,
            &bt_inputs,
            &ct_inputs,
            &mut a[0..cs.l()],
            &mut b_g1[0..cs.l()],
            &mut b_g2[0..cs.l()],
            &mut ic,
            &gamma_inverse,
            &alpha,
            &beta,
        );

        // Evaluate for auxiliary variables.
        eval(
            g1,
            g2,
            &powers_of_tau,
            &at_aux,
            &bt_aux,
            &ct_aux,
            &mut a[cs.l()..],
            &mut b_g1[cs.l()..],
            &mut b_g2[cs.l()..],
            &mut l,
            &delta_inverse,
            &alpha,
            &beta,
        );

        let vk = VerifyingKey {
            alpha_g1: (G1Affine::ADDITIVE_GENERATOR * alpha).into(),
            beta_g1: (G1Affine::ADDITIVE_GENERATOR * beta).into(),
            beta_g2: (G2Affine::ADDITIVE_GENERATOR * beta).into(),
            gamma_g2: (G2Affine::ADDITIVE_GENERATOR * gamma).into(),
            delta_g1: (G1Affine::ADDITIVE_GENERATOR * delta).into(),
            delta_g2: (G2Affine::ADDITIVE_GENERATOR * delta).into(),
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

        Ok((Prover { params }, Verifier { vk: pvk }))
    }
}

fn eval(
    g1: G1Affine,
    g2: G2Affine,

    // Lagrange coefficients for tau
    powers_of_tau: &Coefficients<Fr>,

    // QAP polynomials
    at: &[Vec<(Fr, usize)>],
    bt: &[Vec<(Fr, usize)>],
    ct: &[Vec<(Fr, usize)>],

    // Resulting evaluated QAP polynomials
    a: &mut [G1Affine],
    b_g1: &mut [G1Affine],
    b_g2: &mut [G2Affine],
    ext: &mut [G1Affine],

    // Inverse coefficient for ext elements
    inv: &Fr,

    // Trapdoors
    alpha: &Fr,
    beta: &Fr,
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
