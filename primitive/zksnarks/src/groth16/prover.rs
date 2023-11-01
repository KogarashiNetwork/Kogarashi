mod proof;

use super::{constraint::Constraint, matrix::Element};
use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::error::Groth16Error;
use crate::groth16::key::Parameters;
use crate::groth16::Groth16;
use itertools::Itertools;
use poly_commit::{msm_curve_addition, Fft, PointsValue};
pub use proof::Proof;
use rand::rngs::OsRng;
use zkstd::common::{CurveGroup, Group, Pairing, TwistedEdwardsCurve, Vec};

#[derive(Debug)]
pub struct Prover<P: Pairing> {
    pub params: Parameters<P>,
    pub constraints: Vec<Constraint<<P::JubjubAffine as TwistedEdwardsCurve>::Range>>,
}

impl<P: Pairing> Prover<P> {
    /// Execute the gadget, and return whether all constraints were satisfied.
    pub fn create_proof<C>(&mut self, circuit: C) -> Result<Proof<P>, Error>
    where
        C: Circuit<P::JubjubAffine, ConstraintSystem = Groth16<P::JubjubAffine>>,
    {
        let mut cs = Groth16::<P::JubjubAffine>::initialize();
        circuit.synthesize(&mut cs)?;

        cs.eval_constraints();

        let size = cs.m().next_power_of_two();
        let k = size.trailing_zeros();
        let vk = self.params.vk.clone();

        let r = P::ScalarField::random(OsRng);
        let s = P::ScalarField::random(OsRng);

        let fft = Fft::<P::ScalarField>::new(k as usize);

        // Do the calculation of H(X): A(X) * B(X) - C(X) == H(X) * T(X)
        let a = fft.idft(PointsValue(cs.a.clone()));
        let a = fft.coset_dft(a);
        let b = fft.idft(PointsValue(cs.b.clone()));
        let b = fft.coset_dft(b);
        let c = fft.idft(PointsValue(cs.c.clone()));
        let c = fft.coset_dft(c);

        let mut h = &a * &b;
        h = &h - &c;

        let q = fft.divide_by_z_on_coset(h);
        let mut q = fft.coset_idft(q);
        q.0.truncate(fft.size() - 1);

        // Blind evaluation at precalculated points.
        // From here we do all evaluations with `msm_curve_addition` to not give access to original values.
        let q = msm_curve_addition(&self.params.h, &q);

        let input_assignment = cs
            .instance
            .iter()
            .sorted()
            .map(|Element(_, x)| *x)
            .collect::<Vec<_>>();
        let aux_assignment = cs
            .witness
            .iter()
            .sorted()
            .map(|Element(_, x)| *x)
            .collect::<Vec<_>>();

        let l = msm_curve_addition(&self.params.l, &aux_assignment);

        let a_inputs = msm_curve_addition(&self.params.a, &input_assignment);
        let a_aux = msm_curve_addition(&self.params.a[cs.instance_len()..], &aux_assignment);

        let b_g1_inputs = msm_curve_addition(&self.params.b_g1, &input_assignment);
        let b_g1_aux = msm_curve_addition(&self.params.b_g1[cs.instance_len()..], &aux_assignment);

        let b_g2_inputs = msm_curve_addition(&self.params.b_g2, &input_assignment);
        let b_g2_aux = msm_curve_addition(&self.params.b_g2[cs.instance_len()..], &aux_assignment);

        if vk.delta_g1.is_identity() || vk.delta_g2.is_identity() {
            // If this element is zero, someone is trying to perform a
            // subversion-CRS attack.
            // TODO: proper error
            return Err(Groth16Error::General.into());
        }

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

        Ok(Proof::<P> {
            a: g_a.into(),
            b: g_b.into(),
            c: g_c.into(),
        })
    }
}
