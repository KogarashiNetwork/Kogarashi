use ark_bls12_381::{Bls12_381, Fr as BlsScalar};
use ark_ec::models::twisted_edwards_extended::GroupAffine;
use ark_ec::TEModelParameters;
use ark_ed_on_bls12_381::EdwardsParameters as JubJubParameters;
use ark_ff::PrimeField;
use ark_poly::polynomial::univariate::DensePolynomial;
use ark_poly_commit::{sonic_pc::SonicKZG10, PolynomialCommitment};
use plonk::circuit::{verify_proof, Circuit};
use plonk::constraint_system::StandardComposer;
use plonk::error::to_pc_error;
use plonk::error::Error;
use plonk::prelude::*;
use rand_core::OsRng;

fn main() -> Result<(), Error> {
    #[derive(derivative::Derivative)]
    #[derivative(Debug(bound = ""), Default(bound = ""))]
    pub struct ConfidentialTransferCircuit<F, P>
    where
        F: PrimeField,
        P: TEModelParameters<BaseField = F>,
    {
        alice_public_key: GroupAffine<P>,
        bob_public_key: GroupAffine<P>,
        alice_left_encrypted_balance: GroupAffine<P>,
        alice_right_encrypted_balance: GroupAffine<P>,
        alice_left_encrypted_transfer_amount: GroupAffine<P>,
        alice_right_encrypted_transfer_amount: GroupAffine<P>,
        bob_left_encrypted_transfer_amount: GroupAffine<P>,
        bob_right_encrypted_transfer_amount: GroupAffine<P>,
        generator: GroupAffine<P>,
        alice_private_key: F,
        transfer_amount_b: P::ScalarField,
        alice_after_balance: P::ScalarField,
        randomness: F,
    }

    impl<F, P> Circuit<F, P> for ConfidentialTransferCircuit<F, P>
    where
        F: PrimeField,
        P: TEModelParameters<BaseField = F>,
    {
        const CIRCUIT_ID: [u8; 32] = [0xff; 32];

        fn gadget(&mut self, composer: &mut StandardComposer<F, P>) -> Result<(), Error> {
            let alice_public_key =
                composer.add_affine_to_circuit_description(self.alice_public_key);
            let bob_public_key = composer.add_affine_to_circuit_description(self.bob_public_key);
            let alice_left_encrypted_balance =
                composer.add_affine_to_circuit_description(self.alice_left_encrypted_balance);
            let alice_right_encrypted_balance =
                composer.add_affine_to_circuit_description(self.alice_right_encrypted_balance);
            let alice_left_encrypted_transfer_amount = composer
                .add_affine_to_circuit_description(self.alice_left_encrypted_transfer_amount);
            let alice_right_encrypted_transfer_amount = composer
                .add_affine_to_circuit_description(self.alice_right_encrypted_transfer_amount);
            let bob_left_encrypted_transfer_amount =
                composer.add_affine_to_circuit_description(self.bob_left_encrypted_transfer_amount);
            let bob_right_encrypted_transfer_amount = composer
                .add_affine_to_circuit_description(self.bob_right_encrypted_transfer_amount);
            let generator = composer.add_affine_to_circuit_description(self.generator);
            let alice_private_key = composer.add_input(self.alice_private_key);
            let transfer_amount_b =
                composer.add_input(from_embedded_curve_scalar::<F, P>(self.transfer_amount_b));
            let randomness = composer.add_input(self.randomness);
            let zero = composer.zero_var();

            let g_pow_balance = composer.fixed_base_scalar_mul(transfer_amount_b, self.generator);
            let alice_pk_powered_by_randomness =
                composer.fixed_base_scalar_mul(randomness, self.alice_public_key);

            Ok(())
        }

        fn padded_circuit_size(&self) -> usize {
            1 << 9
        }
    }

    // Generate CRS
    type PC = SonicKZG10<Bls12_381, DensePolynomial<BlsScalar>>;
    let pp = PC::setup(1 << 10, None, &mut OsRng).map_err(to_pc_error::<BlsScalar, PC>)?;

    let mut circuit = ConfidentialTransferCircuit::<BlsScalar, JubJubParameters>::default();
    // Compile the circuit
    let (pk_p, (vk, _pi_pos)) = circuit.compile::<PC>(&pp)?;

    let (x, y) = JubJubParameters::AFFINE_GENERATOR_COEFFS;
    let generator: GroupAffine<JubJubParameters> = GroupAffine::new(x, y);

    // Prover POV
    let (proof, pi) = {
        let mut circuit: ConfidentialTransferCircuit<BlsScalar, JubJubParameters> =
            ConfidentialTransferCircuit {
                alice_public_key: todo!(),
                bob_public_key: todo!(),
                alice_left_encrypted_balance: todo!(),
                alice_right_encrypted_balance: todo!(),
                alice_left_encrypted_transfer_amount: todo!(),
                alice_right_encrypted_transfer_amount: todo!(),
                bob_left_encrypted_transfer_amount: todo!(),
                bob_right_encrypted_transfer_amount: todo!(),
                generator,
                alice_private_key: todo!(),
                transfer_amount_b: todo!(),
                alice_after_balance: todo!(),
                randomness: todo!(),
            };
        circuit.gen_proof::<PC>(&pp, pk_p, b"ConfidentialTransfer")
    }?;

    // Verifier POV
    let verifier_data = VerifierData::new(vk, pi);
    verify_proof::<BlsScalar, JubJubParameters, PC>(
        &pp,
        verifier_data.key,
        &proof,
        &verifier_data.pi,
        b"ConfidentialTransfer",
    )
}
