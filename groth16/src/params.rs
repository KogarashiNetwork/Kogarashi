use crate::verifier::VerifyingKey;

use bls_12_381::{G1Affine, G2Affine};
use zkstd::common::Vec;

#[derive(Clone, Debug)]
pub struct Parameters {
    pub vk: VerifyingKey,

    // Elements of the form ((tau^i * t(tau)) / delta) for i between 0 and
    // m-2 inclusive. Never contains points at infinity.
    pub h: Vec<G1Affine>,

    // Elements of the form (beta * u_i(tau) + alpha v_i(tau) + w_i(tau)) / delta
    // for all auxiliary inputs. Variables can never be unconstrained, so this
    // never contains points at infinity.
    pub l: Vec<G1Affine>,

    // QAP "A" polynomials evaluated at tau in the Lagrange basis. Never contains
    // points at infinity: polynomials that evaluate to zero are omitted from
    // the CRS and the prover can deterministically skip their evaluation.
    pub a: Vec<G1Affine>,

    // QAP "B" polynomials evaluated at tau in the Lagrange basis. Needed in
    // G1 and G2 for C/B queries, respectively. Never contains points at
    // infinity for the same reason as the "A" polynomials.
    pub b_g1: Vec<G1Affine>,
    pub b_g2: Vec<G2Affine>,
}
