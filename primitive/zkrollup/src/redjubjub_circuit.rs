use ec_pairing::TatePairing;
use jub_jub::JubjubAffine;
use zero_plonk::prelude::*;
use zkstd::behave::Ring;
use zkstd::common::{CurveGroup, Pairing, SigUtils};

use red_jubjub::{
    constant::{sapling_base_point, sapling_redjubjub_cofactor},
    Signature,
};

/// Confidential transfer circuit
#[derive(Debug, PartialEq, Default)]
pub struct RedJubjubCircuit {
    public_key: JubjubAffine,
    signature: Signature,
    msg_hash: JubjubScalar,
}

impl RedJubjubCircuit {
    /// Init confidential tranfer circuit
    #[allow(dead_code)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(public_key: JubjubAffine, signature: Signature, msg_hash: JubjubScalar) -> Self {
        Self {
            public_key,
            msg_hash,
            signature,
        }
    }
}

pub(crate) fn check_signature<P: Pairing>(
    composer: &mut Builder<P>,
    public_key: P::JubjubAffine,
    signature: Signature,
    msg_hash: P::JubjubScalar,
) -> Result<(), Error> {
    let r = match P::JubjubAffine::from_bytes(signature.r()) {
        Some(r) => composer.append_point(r),
        None => return Err(Error::ProofVerificationError),
    };
    let s = match P::JubjubScalar::from_bytes(signature.s()) {
        Some(s) => composer.append_witness(s),
        None => return Err(Error::ProofVerificationError),
    };

    let msg_hash = composer.append_witness(msg_hash);
    let public_key = composer.append_point(public_key);

    let sapling_base_point = composer.append_constant_point(sapling_base_point::<P>());
    let sapling_redjubjub_cofactor =
        composer.append_constant(sapling_redjubjub_cofactor::<P::ScalarField>());
    let neg = composer.append_witness(-P::JubjubScalar::one());

    let s_bp = composer.component_mul_point(s, sapling_base_point);
    let hash_pub_key = composer.component_mul_point(msg_hash, public_key);
    let s_bp_neg = composer.component_mul_point(neg, s_bp);
    let s_bp_neg_r = composer.component_add_point(s_bp_neg, r);
    let s_bp_neg_r_hash_pub_key = composer.component_add_point(s_bp_neg_r, hash_pub_key);
    let finalized =
        composer.component_mul_point(sapling_redjubjub_cofactor, s_bp_neg_r_hash_pub_key);

    composer.assert_equal_public_point(finalized, P::JubjubExtended::ADDITIVE_IDENTITY);

    Ok(())
}

impl Circuit<TatePairing> for RedJubjubCircuit {
    fn circuit(&self, composer: &mut Builder<TatePairing>) -> Result<(), Error> {
        check_signature(composer, self.public_key, self.signature, self.msg_hash)
    }
}

#[cfg(test)]
mod tests {

    use ec_pairing::TatePairing;
    use jub_jub::Fp;
    use poly_commit::KzgParams;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use zero_plonk::prelude::*;
    use zkstd::common::{Group, SigUtils};

    use red_jubjub::{sapling_hash, SecretKey};

    use super::RedJubjubCircuit;

    #[test]
    fn redjubjub_verification() {
        let n = 13;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);

        let mut pp = KzgParams::setup(n, BlsScalar::random(&mut rng));

        let msg = b"test";

        let priv_key = SecretKey::<TatePairing>::new(Fp::random(&mut rng));
        let sig = priv_key.sign(msg, &mut rng);
        let pub_key = priv_key.to_public_key();

        let redjubjub_circuit = RedJubjubCircuit::new(
            pub_key.inner().into(),
            sig,
            sapling_hash(&sig.r(), &pub_key.to_bytes(), msg),
        );
        let (prover, verifier) = Compiler::compile::<RedJubjubCircuit, TatePairing>(&mut pp, label)
            .expect("failed to compile circuit");
        let (proof, public_inputs) = prover
            .prove(&mut rng, &redjubjub_circuit)
            .expect("failed to prove");
        verifier
            .verify(&proof, &public_inputs)
            .expect("failed to verify proof");
    }
}
