pub mod arithmetic;
pub mod curve;
pub mod logic;
pub mod permutation;
pub mod range;
pub(crate) mod utils;

use crate::plonk::transcript::TranscriptProtocol;
use merlin::Transcript;
use poly_commit::PointsValue;
use zkstd::common::Pairing;

/// Verification Key
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct VerificationKey<P: Pairing> {
    /// Circuit size (not padded to a power of two).
    pub n: usize,
    /// inverse of n
    pub n_inv: P::ScalarField,
    /// generator of omega
    pub generator: P::ScalarField,
    /// inverse of generator
    pub generator_inv: P::ScalarField,
    /// arithmetic gates
    pub arithmetic: arithmetic::VerificationKey<P::G1Affine>,
    /// logic gates
    pub logic: logic::VerificationKey<P::G1Affine>,
    /// range gates
    pub range: range::VerificationKey<P::G1Affine>,
    /// fixed base curve addition gates
    pub curve_scalar: curve::scalar::VerificationKey<P>,
    /// variable base curve addition gates
    pub curve_addtion: curve::add::VerificationKey<P>,
    /// permutation checks
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
            &self.curve_addtion.q_variable_group_add,
        );
        <Transcript as TranscriptProtocol<P>>::append_commitment(
            transcript,
            b"q_fixed_group_add",
            &self.curve_scalar.q_fixed_group_add,
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

/// Proving Key.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ProvingKey<P: Pairing> {
    /// Circuit size
    pub n: usize,
    /// ProvingKey for arithmetic gate
    pub arithmetic: arithmetic::ProvingKey<P::ScalarField>,
    /// ProvingKey for logic gate
    pub logic: logic::ProvingKey<P::ScalarField>,
    /// ProvingKey for range gate
    pub range: range::ProvingKey<P::ScalarField>,
    /// ProvingKey for fixed base curve addition gates
    pub curve_scalar: curve::scalar::ProvingKey<P>,
    /// ProvingKey for variable base curve addition gates
    pub curve_addtion: curve::add::ProvingKey<P>,
    /// ProvingKey for permutation checks
    pub permutation: permutation::ProvingKey<P::ScalarField>,
    // Pre-processes the 8n PointsValue for the vanishing polynomial, so
    // they do not need to be computed at the proving stage.
    // Note: With this, we can combine all parts of the quotient polynomial
    // in their evaluation phase and divide by the quotient
    // polynomial without having to perform IFFT
    pub v_h_coset_8n: PointsValue<P::ScalarField>,
}

impl<P: Pairing> ProvingKey<P> {
    pub fn v_h_coset_8n(&self) -> &PointsValue<P::ScalarField> {
        &self.v_h_coset_8n
    }
}
