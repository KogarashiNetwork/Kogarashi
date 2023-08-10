pub mod arithmetic;
pub mod curve;
pub mod logic;
pub mod permutation;
pub mod range;

use crate::plonk::transcript::TranscriptProtocol;
use merlin::Transcript;
use zkstd::common::Pairing;

/// Verification Key
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct VerificationKey<P: Pairing> {
    /// Circuit size (not padded to a power of two).
    pub n: usize,
    /// VerificationKey for arithmetic gates
    pub arithmetic: arithmetic::VerificationKey<P::G1Affine>,
    /// VerificationKey for logic gates
    pub logic: logic::VerificationKey<P::G1Affine>,
    /// VerificationKey for range gates
    pub range: range::VerificationKey<P::G1Affine>,
    /// VerificationKey for fixed base curve addition gates
    pub fixed_base: curve::scalar::VerificationKey<P>,
    /// VerificationKey for variable base curve addition gates
    pub variable_base: curve::add::VerificationKey<P>,
    /// VerificationKey for permutation checks
    pub permutation: permutation::VerificationKey<P::G1Affine>,
}

impl<P: Pairing> VerificationKey<P> {
    /// Adds the circuit description to the transcript
    pub fn seed_transcript(&self, transcript: &mut Transcript) {
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_m",
            &self.arithmetic.q_m,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_l",
            &self.arithmetic.q_l,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_r",
            &self.arithmetic.q_r,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_o",
            &self.arithmetic.q_o,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_c",
            &self.arithmetic.q_c,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_4",
            &self.arithmetic.q_4,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_arith",
            &self.arithmetic.q_arith,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_range",
            &self.range.q_range,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_logic",
            &self.logic.q_logic,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_variable_group_add",
            &self.variable_base.q_variable_group_add,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_fixed_group_add",
            &self.fixed_base.q_fixed_group_add,
        );

        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"s_sigma_1",
            &self.permutation.s_sigma_1,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"s_sigma_2",
            &self.permutation.s_sigma_2,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"s_sigma_3",
            &self.permutation.s_sigma_3,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"s_sigma_4",
            &self.permutation.s_sigma_1,
        );

        // Append circuit size to transcript
        <Transcript as TranscriptProtocol<P>>::circuit_domain_sep(transcript, self.n as u64);
    }
}
