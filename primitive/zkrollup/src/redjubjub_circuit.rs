use jub_jub::JubjubAffine;
use red_jubjub::RedJubjub;
use red_jubjub::{
    constant::{sapling_base_point, sapling_redjubjub_cofactor},
    Signature,
};
use zkplonk::prelude::*;
use zkstd::common::{RedDSA, Ring};
use zkstd::common::{SigUtils, TwistedEdwardsCurve};

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

pub(crate) fn check_signature<P: RedDSA>(
    composer: &mut Plonk<P::Affine>,
    public_key: P::Affine,
    signature: Signature,
    msg_hash: P::Scalar,
) -> Result<(), Error> {
    let r = match P::Affine::from_bytes(signature.r()) {
        Some(r) => composer.append_point(r),
        None => return Err(Error::ProofVerificationError),
    };
    let s = match P::Scalar::from_bytes(signature.s()) {
        Some(s) => composer.append_witness(s),
        None => return Err(Error::ProofVerificationError),
    };

    let msg_hash = composer.append_witness(msg_hash);
    let public_key = composer.append_point(public_key);

    let sapling_base_point = composer.append_constant_point(sapling_base_point::<P>());
    let sapling_redjubjub_cofactor =
        composer.append_constant(sapling_redjubjub_cofactor::<P::Range>());
    let neg = composer.append_witness(-P::Scalar::one());

    let s_bp = composer.component_mul_point(s, sapling_base_point);
    let hash_pub_key = composer.component_mul_point(msg_hash, public_key);
    let s_bp_neg = composer.component_mul_point(neg, s_bp);
    let s_bp_neg_r = composer.component_add_point(s_bp_neg, r);
    let s_bp_neg_r_hash_pub_key = composer.component_add_point(s_bp_neg_r, hash_pub_key);
    let finalized =
        composer.component_mul_point(sapling_redjubjub_cofactor, s_bp_neg_r_hash_pub_key);

    composer.assert_equal_public_point(finalized, P::Extended::ADDITIVE_IDENTITY);

    Ok(())
}

impl Circuit<JubjubAffine> for RedJubjubCircuit {
    type ConstraintSystem = Plonk<JubjubAffine>;
    fn synthesize(&self, composer: &mut Plonk<JubjubAffine>) -> Result<(), Error> {
        check_signature::<RedJubjub>(composer, self.public_key, self.signature, self.msg_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::RedJubjubCircuit;

    use ec_pairing::TatePairing;
    use jub_jub::Fp;
    use rand::rngs::StdRng;
    use rand_core::SeedableRng;
    use red_jubjub::{sapling_hash, RedJubjub, SecretKey};
    use zkplonk::prelude::*;
    use zksnarks::keypair::Keypair;
    use zksnarks::plonk::PlonkParams;
    use zksnarks::public_params::PublicParameters;
    use zkstd::common::{Group, SigUtils};

    #[test]
    fn redjubjub_verification() {
        let n = 13;
        let label = b"verify";
        let mut rng = StdRng::seed_from_u64(8349u64);

        let mut pp = PlonkParams::setup(n, &mut rng);

        let msg = b"test";

        let priv_key = SecretKey::<RedJubjub>::new(Fp::random(&mut rng));
        let sig = priv_key.sign(msg, &mut rng);
        let pub_key = priv_key.to_public_key();

        let redjubjub_circuit = RedJubjubCircuit::new(
            pub_key.inner().into(),
            sig,
            sapling_hash(&sig.r(), &pub_key.to_bytes(), msg),
        );
        let (prover, verifier) =
            PlonkKey::<TatePairing, JubjubAffine, RedJubjubCircuit>::compile(&mut pp)
                .expect("failed to compile circuit");
        let (proof, public_inputs) = prover
            .create_proof(&mut rng, &redjubjub_circuit)
            .expect("failed to prove");
        verifier
            .verify(&proof, &public_inputs)
            .expect("failed to verify proof");
    }
}
