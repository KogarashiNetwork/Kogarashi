use codec::{Decode, Encode};
pub use rand_xorshift::XorShiftRng as FullcodecRng;
pub use zero_bls12_381::Fr;
pub use zero_jubjub::GENERATOR_EXTENDED;
pub use zero_plonk::prelude::{
    BlsScalar, Circuit, Constraint, Error as PlonkError, JubJubAffine, JubJubScalar, Proof,
    PublicParameters,
};

/// The struct for Merlin transcript and used for proof verify
#[derive(Debug, PartialEq, Clone, Encode)]
pub struct Transcript(pub &'static [u8]);

#[allow(unconditional_recursion)]
impl Decode for Transcript {
    fn decode<I: codec::Input>(input: &mut I) -> Result<Self, codec::Error> {
        Decode::decode(input)
    }
}
