#[derive(Debug, Clone, Copy)]
pub enum Error {
    ProverSubVersionCrsAttack,
    ProverInversionFailed,
    UnsupportedWNAF2k,
    ProofVerificationError,
    InconsistentPublicInputsLen {
        expected: usize,
        provided: usize,
    }
}
