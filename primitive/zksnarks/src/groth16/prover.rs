mod proof;

use super::constraint::Constraint;
use crate::circuit::Circuit;
use crate::constraint_system::ConstraintSystem;
use crate::error::Error;
use crate::groth16::error::Groth16Error;
use crate::groth16::key::Parameters;
use crate::groth16::params::Groth16Params;
use crate::groth16::Groth16;
use itertools::Itertools;
use poly_commit::{msm_curve_addtion, Fft, PointsValue};
pub use proof::Proof;
use zkstd::common::{CurveGroup, Group, Pairing, TwistedEdwardsCurve, Vec};

#[derive(Debug)]
pub struct Prover<P: Pairing> {
    pub params: Parameters<P>,
    pub constraints: Vec<Constraint<<P::JubjubAffine as TwistedEdwardsCurve>::Range>>,
    pub(crate) keypair: Groth16Params<P>,
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

        let r = P::ScalarField::from(35);
        let s = P::ScalarField::from(42);

        let fft = Fft::<P::ScalarField>::new(k as usize);

        let (left, h) = {
            let a = fft.idft(PointsValue(cs.a.clone()));
            let b = fft.idft(PointsValue(cs.b.clone()));
            let c = fft.idft(PointsValue(cs.c.clone()));

            let mut a = fft.coset_dft(a);
            let b = fft.coset_dft(b);
            let c = fft.coset_dft(c);

            a = &a * &b;
            a = &a - &c;

            let left = fft.coset_idft(a.clone());
            a = fft.divide_by_z_on_coset(a);
            let mut a = fft.coset_idft(a);
            a.0.truncate(fft.size() - 1);

            (left, a)
        };

        let h = msm_curve_addtion(&self.params.h, &h);

        let input_assignment = cs
            .instance
            .iter()
            .sorted()
            .map(|(_, x)| *x)
            .collect::<Vec<_>>();
        let aux_assignment = cs
            .witness
            .iter()
            .sorted()
            .map(|(_, x)| *x)
            .collect::<Vec<_>>();
        let l = msm_curve_addtion(&self.params.l, &aux_assignment);
        println!("H = {:#?}", P::G1Affine::from(h));
        println!("L = {:#?}", P::G1Affine::from(l));

        let a_inputs = msm_curve_addtion(&self.params.a, &input_assignment);
        let a_aux = msm_curve_addtion(&self.params.a, &aux_assignment);

        println!("A_inputs = {:#?}", P::G1Affine::from(a_inputs));
        println!("A_aux = {:#?}", P::G1Affine::from(a_aux));

        let b_g1_inputs = msm_curve_addtion(&self.params.b_g1, &input_assignment);
        let b_g1_aux = msm_curve_addtion(&self.params.b_g1, &aux_assignment);

        println!("B_g1_inputs = {:#?}", P::G1Affine::from(b_g1_inputs));
        println!("B_g1_aux = {:#?}", P::G1Affine::from(b_g1_aux));

        let b_g2_inputs = msm_curve_addtion(&self.params.b_g2, &input_assignment);
        let b_g2_aux = msm_curve_addtion(&self.params.b_g2, &aux_assignment);

        if vk.delta_g1.is_identity() || vk.delta_g2.is_identity() {
            // If this element is zero, someone is trying to perform a
            // subversion-CRS attack.
            return Err(Groth16Error::General.into());
        }

        let mut g_a = vk.delta_g1 * r + vk.alpha_g1;
        let mut g_b = vk.delta_g2 * s + vk.beta_g2;
        let mut g_c = vk.delta_g1 * r * s + (vk.alpha_g1 * s) + (vk.beta_g1 * r);

        println!("G_a = {:#?}", P::G1Affine::from(g_a));
        println!("G_b = {:#?}", P::G2Affine::from(g_b));
        println!("G_c = {:#?}", P::G1Affine::from(g_c));

        let a_answer = a_inputs + a_aux;
        g_a += a_answer;
        g_c += a_answer * s;

        let b1_answer = b_g1_inputs + b_g1_aux;
        let b2_answer = b_g2_inputs + b_g2_aux;

        g_b += b2_answer;
        g_c += b1_answer * r;
        g_c += l + h;

        // let point = P::ScalarField::random(OsRng);
        // let left_eval = left.evaluate(&point);
        // let h_eval = h.evaluate(&point);
        // let t_eval = fft.z_on_coset();
        // let right: P::ScalarField = h_eval * t_eval;
        //
        // let left_com = self.keypair.commitment_key.commit(&left);
        // let h_com = self.keypair.commitment_key.commit(&h);
        // let t_g2 = self.keypair.evaluation_key.h * t_eval;

        // assert_eq!(left_eval, right);

        Ok(Proof::<P> {
            a: g_a.into(),
            b: g_b.into(),
            c: g_c.into(),
        })
    }
}
