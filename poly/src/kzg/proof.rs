use crate::commitment::Commitment;
use zkstd::common::{Decode, Encode, Pairing};

/// Proof that a polynomial `p` was correctly evaluated at a point `z`
/// producing the evaluated point p(z).
#[derive(Clone, Debug, Decode, Encode)]
pub struct Proof<P: Pairing> {
    /// This is a commitment to the witness polynomial.
    pub commitment_to_witness: Commitment<P::G1Affine>,
    /// This is the result of evaluating a polynomial at the point `z`.
    pub evaluated_point: P::ScalarField,
    /// This is the commitment to the polynomial that you want to prove a
    /// statement about.
    pub commitment_to_polynomial: Commitment<P::G1Affine>,
}
