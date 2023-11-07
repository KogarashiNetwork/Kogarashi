use crate::circuit::Circuit;
use crate::error::Error;
use crate::proof::Proof;
use crate::zksnark::Parameters;

use bls_12_381::Fr;
use poly_commit::{msm_curve_addition, Fft, PointsValue};
use r1cs::R1cs;
use zkstd::common::{CurveGroup, Group, RngCore};

#[derive(Debug)]
pub struct Prover {
    pub params: Parameters,
}

impl Prover {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn create_proof<C: Circuit, R: RngCore>(
        &mut self,
        rng: &mut R,
        circuit: C,
    ) -> Result<Proof, Error> {
        let mut cs = R1cs::default();
        circuit.synthesize(&mut cs)?;

        let size = cs.m().next_power_of_two();
        let k = size.trailing_zeros();
        let vk = self.params.vk.clone();

        let fft = Fft::<Fr>::new(k as usize);
        let (a, b, c) = cs.evaluate();

        // Do the calculation of H(X): A(X) * B(X) - C(X) == H(X) * T(X)
        let a = fft.idft(PointsValue(a));
        let a = fft.coset_dft(a);
        let b = fft.idft(PointsValue(b));
        let b = fft.coset_dft(b);
        let c = fft.idft(PointsValue(c));
        let c = fft.coset_dft(c);

        let mut h = &a * &b;
        h = &h - &c;

        let q = fft.divide_by_z_on_coset(h);
        let q = fft.coset_idft(q);

        // Blind evaluation at precalculated points.
        // From here we do all evaluations with `msm_curve_addition` to not give access to original values.
        let q = msm_curve_addition(&self.params.h, &q);

        let input_assignment = cs.x();
        let aux_assignment = cs.w();

        let l = msm_curve_addition(&self.params.l, &aux_assignment);

        let a_inputs = msm_curve_addition(&self.params.a, &input_assignment);
        let a_aux = msm_curve_addition(&self.params.a[cs.l()..], &aux_assignment);

        let b_g1_inputs = msm_curve_addition(&self.params.b_g1, &input_assignment);
        let b_g1_aux = msm_curve_addition(&self.params.b_g1[cs.l()..], &aux_assignment);

        let b_g2_inputs = msm_curve_addition(&self.params.b_g2, &input_assignment);
        let b_g2_aux = msm_curve_addition(&self.params.b_g2[cs.l()..], &aux_assignment);

        if vk.delta_g1.is_identity() || vk.delta_g2.is_identity() {
            return Err(Error::ProverSubVersionCrsAttack);
        }

        let r = Fr::random(&mut *rng);
        let s = Fr::random(&mut *rng);

        // Setup shift parameters r * delta and s * delta in A, B and C computations.
        let mut g_a = vk.delta_g1 * r + vk.alpha_g1;
        let mut g_b = vk.delta_g2 * s + vk.beta_g2;
        let mut g_c = vk.delta_g1 * r * s + (vk.alpha_g1 * s) + (vk.beta_g1 * r);

        // QAP evaluations for inputs and aux variables. In curve point form.
        let a_answer = a_inputs + a_aux;
        g_a += a_answer;
        // As
        g_c += a_answer * s;

        let b1_answer = b_g1_inputs + b_g1_aux;
        let b2_answer = b_g2_inputs + b_g2_aux;

        g_b += b2_answer;
        // rB
        g_c += b1_answer * r;
        // Evaluations for QAP polynomials with alpha and beta shift.
        g_c += q + l;

        Ok(Proof {
            a: g_a.into(),
            b: g_b.into(),
            c: g_c.into(),
        })
    }
}
