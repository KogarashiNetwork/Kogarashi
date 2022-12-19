use ark_bls12_381::{Bls12_381, Fr as BlsScalar};
use ark_ec::models::twisted_edwards_extended::GroupAffine;
use ark_ec::{AffineCurve, TEModelParameters};
use ark_ed_on_bls12_381::{EdwardsParameters as JubJubParameters, Fr as JubJubScalar};
use ark_ff::{BigInteger256, PrimeField};
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
        // bob_public_key: GroupAffine<P>,
        alice_left_encrypted_balance: GroupAffine<P>,
        // alice_right_encrypted_balance: GroupAffine<P>,
        alice_left_encrypted_transfer_amount: GroupAffine<P>,
        // alice_right_encrypted_transfer_amount: GroupAffine<P>,
        // bob_left_encrypted_transfer_amount: GroupAffine<P>,
        // bob_right_encrypted_transfer_amount: GroupAffine<P>,
        // generator: GroupAffine<P>,
        alice_private_key: P::ScalarField,
        transfer_amount_b: P::ScalarField,
        // alice_after_balance: P::ScalarField,
        randomness: P::ScalarField,
    }

    impl<F, P> Circuit<F, P> for ConfidentialTransferCircuit<F, P>
    where
        F: PrimeField,
        P: TEModelParameters<BaseField = F>,
    {
        const CIRCUIT_ID: [u8; 32] = [0xff; 32];

        fn gadget(&mut self, composer: &mut StandardComposer<F, P>) -> Result<(), Error> {
            let (x, y) = P::AFFINE_GENERATOR_COEFFS;
            let generator = GroupAffine::new(x, y);

            // let alice_left_encrypted_balance =
            //     composer.add_affine_to_circuit_description(self.alice_left_encrypted_balance);
            // let alice_right_encrypted_balance =
            //     composer.add_affine_to_circuit_description(self.alice_right_encrypted_balance);
            // let alice_left_encrypted_transfer_amount = composer
            //     .add_affine_to_circuit_description(self.alice_left_encrypted_transfer_amount);
            // let alice_right_encrypted_transfer_amount = composer
            //     .add_affine_to_circuit_description(self.alice_right_encrypted_transfer_amount);
            let alice_private_key =
                composer.add_input(from_embedded_curve_scalar::<F, P>(self.alice_private_key));
            let transfer_amount_b =
                composer.add_input(from_embedded_curve_scalar::<F, P>(self.transfer_amount_b));
            // let alice_after_balance =
            //     composer.add_input(from_embedded_curve_scalar::<F, P>(self.alice_after_balance));
            let randomness =
                composer.add_input(from_embedded_curve_scalar::<F, P>(self.randomness));
            // let one = composer.add_input(F::one());

            // Alice left encrypted transfer check
            let g_pow_balance = composer.fixed_base_scalar_mul(transfer_amount_b, generator);
            let alice_pk_powered_by_randomness =
                composer.fixed_base_scalar_mul(randomness, self.alice_public_key);
            let left_alice_transfer =
                composer.point_addition_gate(g_pow_balance, alice_pk_powered_by_randomness);
            composer.assert_equal_public_point(
                left_alice_transfer,
                self.alice_left_encrypted_transfer_amount,
            );

            // Bob left encrypted transfer check
            // let bob_pk_powered_by_randomness =
            //     composer.fixed_base_scalar_mul(randomness, self.bob_public_key);
            // let left_bob_transfer =
            //     composer.point_addition_gate(g_pow_balance, bob_pk_powered_by_randomness);
            // composer.assert_equal_public_point(
            //     left_bob_transfer,
            //     self.bob_left_encrypted_transfer_amount,
            // );

            // Alice right encrypted transfer check
            // let g_pow_randomness = composer.fixed_base_scalar_mul(randomness, generator);
            // composer.assert_equal_public_point(
            //     g_pow_randomness,
            //     self.alice_right_encrypted_transfer_amount,
            // );

            // Alice after balance is correct
            // let g_pow_after_balance =
            //     composer.fixed_base_scalar_mul(alice_after_balance, generator);
            // let alice_left_transfer_neg =
            //     composer.conditional_point_neg(one, alice_left_encrypted_transfer_amount);
            // let alice_right_transfer_neg =
            //     composer.conditional_point_neg(one, alice_right_encrypted_transfer_amount);
            // let left_after_balance =
            //     composer.point_addition_gate(alice_left_encrypted_balance, alice_left_transfer_neg);
            // let right_after_balance = {
            //     let right_after_balance = composer
            //         .point_addition_gate(alice_right_encrypted_balance, alice_right_transfer_neg);
            //     composer.variable_base_scalar_mul(alice_private_key, right_after_balance)
            // };
            // let x = composer.point_addition_gate(g_pow_after_balance, right_after_balance);
            // composer.assert_equal_point(left_after_balance, x);

            // Public key calculated correctly
            let calculated_pk = composer.fixed_base_scalar_mul(alice_private_key, generator);
            composer.assert_equal_public_point(calculated_pk, self.alice_public_key);

            Ok(())
        }

        fn padded_circuit_size(&self) -> usize {
            1 << 12
        }
    }

    // Generate CRS
    type PC = SonicKZG10<Bls12_381, DensePolynomial<BlsScalar>>;
    let pp = PC::setup(1 << 13, None, &mut OsRng).map_err(to_pc_error::<BlsScalar, PC>)?;

    let mut circuit = ConfidentialTransferCircuit::<BlsScalar, JubJubParameters>::default();
    // Compile the circuit
    let (pk_p, (vk, _pi_pos)) = circuit.compile::<PC>(&pp)?;

    let (x, y) = JubJubParameters::AFFINE_GENERATOR_COEFFS;
    let generator: GroupAffine<JubJubParameters> = GroupAffine::new(x, y);
    let alice_private_key = JubJubScalar::from(BigInteger256::new([
        0xc4676aa0f64d1a88,
        0x16699cf541a046f2,
        0x11325506c9fb4ad9,
        0x0084cc3fcc5453b,
    ]));

    let bob_private_key = JubJubScalar::from(BigInteger256::new([
        0x1278bc1c0481ffe7,
        0xdf2df249cebbda4c,
        0x23aee7c515fd95c1,
        0x019cf38261170bf7,
    ]));
    let alice_balance = JubJubScalar::from(1500);
    let transfer_amount_b = JubJubScalar::from(800);
    let alice_after_balance = JubJubScalar::from(700);
    let alice_original_randomness = 789;
    let randomness = JubJubScalar::from(123);

    let alice_public_key = AffineCurve::mul(&generator, alice_private_key).into();
    // let bob_public_key = AffineCurve::mul(&generator, bob_private_key).into();

    let alice_left_encrypted_balance = AffineCurve::mul(&generator, alice_balance)
        + AffineCurve::mul(&alice_public_key, alice_original_randomness);
    // let alice_right_encrypted_balance =
    //     AffineCurve::mul(&generator, alice_original_randomness).into();

    let alice_left_encrypted_transfer_amount = AffineCurve::mul(&generator, transfer_amount_b)
        + AffineCurve::mul(&alice_public_key, randomness);
    // let alice_right_encrypted_transfer_amount = AffineCurve::mul(&generator, randomness).into();

    // let bob_left_encrypted_transfer_amount = AffineCurve::mul(&generator, transfer_amount_b)
    //     + AffineCurve::mul(&bob_public_key, randomness);
    // let bob_right_encrypted_transfer_amount = AffineCurve::mul(&generator, randomness).into();

    // Prover POV
    let (proof, pi) = {
        let mut circuit: ConfidentialTransferCircuit<BlsScalar, JubJubParameters> =
            ConfidentialTransferCircuit {
                alice_public_key,
                // bob_public_key,
                alice_left_encrypted_balance: alice_left_encrypted_balance.into(),
                // alice_right_encrypted_balance,
                alice_left_encrypted_transfer_amount: alice_left_encrypted_transfer_amount.into(),
                // alice_right_encrypted_transfer_amount,
                // bob_left_encrypted_transfer_amount: bob_left_encrypted_transfer_amount.into(),
                // bob_right_encrypted_transfer_amount,
                // generator,
                alice_private_key,
                transfer_amount_b,
                // alice_after_balance,
                randomness,
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
