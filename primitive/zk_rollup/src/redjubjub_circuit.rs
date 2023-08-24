use ec_pairing::TatePairing;
use jub_jub::JubjubAffine;
use zero_plonk::prelude::*;
use zkstd::common::{CurveGroup, SigUtils};

use red_jubjub::{
    constant::{SAPLING_BASE_POINT, SAPLING_REDJUBJUB_COFACTOR},
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

impl Circuit<TatePairing> for RedJubjubCircuit {
    fn circuit<C>(&self, composer: &mut C) -> Result<(), Error>
    where
        C: Composer<TatePairing>,
    {
        let r = match JubjubAffine::from_bytes(self.signature.r()) {
            Some(r) => composer.append_point(r),
            None => return Err(Error::ProofVerificationError),
        };
        let s = match JubjubScalar::from_bytes(self.signature.s()) {
            Some(s) => composer.append_witness(s),
            None => return Err(Error::ProofVerificationError),
        };

        let msg_hash = composer.append_witness(self.msg_hash);
        let public_key = composer.append_point(self.public_key);

        let sapling_base_point = composer.append_constant_point(SAPLING_BASE_POINT);
        let sapling_redjubjub_cofactor = composer.append_constant(SAPLING_REDJUBJUB_COFACTOR);
        let neg = composer.append_witness(-JubjubScalar::one());

        let s_bp = composer.component_mul_point(s, sapling_base_point);
        let hash_pub_key = composer.component_mul_point(msg_hash, public_key);
        let s_bp_neg = composer.component_mul_point(neg, s_bp);
        let s_bp_neg_r = composer.component_add_point(s_bp_neg, r);
        let s_bp_neg_r_hash_pub_key = composer.component_add_point(s_bp_neg_r, hash_pub_key);
        let finalized =
            composer.component_mul_point(sapling_redjubjub_cofactor, s_bp_neg_r_hash_pub_key);

        composer.assert_equal_public_point(finalized, JubjubExtended::ADDITIVE_IDENTITY);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use ec_pairing::TatePairing;
    use jub_jub::Fp;
    use poly_commit::KeyPair;
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

        let mut pp = KeyPair::setup(n, BlsScalar::random(&mut rng));

        let msg = b"test";

        let priv_key = SecretKey::new(Fp::random(&mut rng));
        let sig = priv_key.sign(msg, &mut rng);
        let pub_key = priv_key.to_public_key();

        let redjubjub_circuit = RedJubjubCircuit::new(
            pub_key.inner().into(),
            sig,
            sapling_hash(&sig.r(), &pub_key.to_bytes(), msg),
        );
        let prover = Compiler::compile::<RedJubjubCircuit, TatePairing>(&mut pp, label)
            .expect("failed to compile circuit");
        prover
            .0
            .prove(&mut rng, &redjubjub_circuit)
            .expect("failed to prove");
    }
}
