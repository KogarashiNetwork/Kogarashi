# Confidential Transfer Tutorial

In this tutorial, we are going to generate test data and test its functionalities. We assume that you already unserstand what [Confidential Transfer](../constraints/confidential_transfer_constraints.md) is.

The steps are following.

1. Define the `confidential_transfer` as depencencies
2. Generate test data used for `confidential_transfer`
3. Test funcitonalities

## 1. Define the confidential_transfer as depencencies
First of all, you need to define the `confidential_transfer`.

- <your-pallet>/Cargo.toml
```toml
confidential_transfer = { git = "https://github.com/KogarashiNetwork/Kogarashi", branch = "master", default-features = false }
pallet_encrypted_balance = { git = "https://github.com/KogarashiNetwork/Kogarashi", branch = "master", default-features = false }
pallet_plonk = { git = "https://github.com/KogarashiNetwork/Kogarashi", branch = "master", default-features = false }
she_elgamal = { git = "https://github.com/KogarashiNetwork/Kogarashi", branch = "master", default-features = false }
bls_12_381 = { git = "https://github.com/KogarashiNetwork/Kogarashi", branch = "master", default-features = false }
rand_core = {version="0.6", default-features = false }
```

The `confidential_transfer` depends on `rand_core` so please import it.

## 2. Generate test data used for `confidential_transfer`
Secondly, we would like like to setup the Alice and Bob account on testing runtime. Define the `new_test_ext` for genesis config and reflect the testing data for runtime storage.

```rust
fn new_test_ext(
    alice_address: u64,
    alice_private_key: Fp,
    alice_balance: u32,
    alice_radomness: Fp,
    bob_private_key: Fp,
    bob_address: u64,
    bob_balance: u32,
    bob_radomness: Fp,
) -> sp_io::TestExternalities {
    let alice_balance = EncryptedNumber::encrypt(alice_private_key, alice_balance, alice_radomness);
    let bob_balance = EncryptedNumber::encrypt(bob_private_key, bob_balance, bob_radomness);

    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<TestRuntime>()
        .unwrap();
    pallet_encrypted_balance::GenesisConfig::<TestRuntime> {
        balances: vec![(alice_address, alice_balance), (bob_address, bob_balance)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
```

Thirdly, we define `generate_default_test_data` to generate parameters used for `confidential_transfer`.

```rust
fn generate_default_test_data() -> (u64, Fp, u16, Fp, Fp, u64, u16, Fp, u16, u16, u16) {
    let mut rng = rand::thread_rng();

    let alice_address = rng.gen::<u64>();
    let alice_private_key = Fp::random(OsRng);
    let alice_balance = rng.gen::<u16>();
    let alice_radomness = Fp::random(OsRng);
    let bob_private_key = Fp::random(OsRng);
    let bob_address = rng.gen::<u64>();
    let bob_balance = rng.gen::<u16>();
    let bob_radomness = Fp::random(OsRng);
    let transfer_amount = rng.gen_range(0..alice_balance);
    let alice_after_balance = alice_balance - transfer_amount;
    let bob_after_balance = bob_balance + transfer_amount;

    (
        alice_address,
        alice_private_key,
        alice_balance,
        alice_radomness,
        bob_private_key,
        bob_address,
        bob_balance,
        bob_radomness,
        transfer_amount,
        alice_after_balance,
        bob_after_balance,
    )
}
```

## 3. Test funcitonalities
Finally, we combine previous sections together and test functionalities.

```rust
fn main() {
    let k = 14;
    let label = b"verify";
    let mut rng = get_rng();
    let (
        alice_address,
        alice_private_key,
        alice_balance,
        alice_radomness,
        bob_private_key,
        bob_address,
        bob_balance,
        bob_radomness,
        transfer_amount,
        alice_after_balance,
        bob_after_balance,
        transfer_randomness,
    ) = generate_default_test_data();
    new_test_ext(
        alice_address,
        alice_private_key,
        alice_balance,
        alice_radomness,
        bob_private_key,
        bob_address,
        bob_balance,
        bob_radomness,
    )
    .execute_with(|| {
        // default balance decryption check
        let alice_encrypted_balance = ConfidentialTransfer::total_balance(&alice_address);
        let alice_raw_balance = alice_encrypted_balance.decrypt(alice_private_key);
        let bob_encrypted_balance = ConfidentialTransfer::total_balance(&bob_address);
        let bob_raw_balance = bob_encrypted_balance.decrypt(bob_private_key);

        assert_eq!(alice_raw_balance.unwrap() as u16, alice_balance);
        assert_eq!(bob_raw_balance.unwrap() as u16, bob_balance);

        // trusted setup check
        let result =
            ConfidentialTransfer::trusted_setup(Origin::signed(alice_address), k, rng.clone());
        assert_ok!(result);

        // proof generation
        let pp = Plonk::public_parameter().unwrap();
        let alice_public_key = GENERATOR_EXTENDED * alice_private_key;
        let bob_public_key = GENERATOR_EXTENDED * bob_private_key;
        let transfer_amount_scalar = Fp::from(transfer_amount as u64);
        let alice_after_balance_scalar = Fp::from(alice_after_balance as u64);

        let alice_balance =
            EncryptedNumber::encrypt(alice_private_key, alice_balance.into(), alice_radomness);
        let alice_transfer_amount = EncryptedNumber::encrypt(
            alice_private_key,
            transfer_amount.into(),
            transfer_randomness,
        );
        let bob_encrypted_transfer_amount =
            (GENERATOR_EXTENDED * transfer_amount_scalar) + (bob_public_key * transfer_randomness);
        let alice_public_key = JubJubAffine::from(alice_public_key);
        let bob_public_key = JubJubAffine::from(bob_public_key);
        let bob_encrypted_transfer_amount = JubJubAffine::from(bob_encrypted_transfer_amount);
        let bob_encrypted_transfer_amount_other = (GENERATOR_EXTENDED * transfer_randomness).into();

        let confidential_transfer_circuit = ConfidentialTransferCircuit::new(
            alice_public_key,
            bob_public_key,
            alice_balance,
            alice_transfer_amount,
            bob_encrypted_transfer_amount,
            alice_private_key,
            transfer_amount_scalar,
            alice_after_balance_scalar,
            transfer_randomness,
        );
        let prover = PlonkKey::new::<ConfidentialTransferCircuit>(&pp, label)
            .expect("failed to compile circuit");
        let proof = prover
            .0
            .create_proof(&mut rng, &confidential_transfer_circuit)
            .expect("failed to prove");

        // confidential transfer check
        let transaction_params = ConfidentialTransferTransaction::new(
            alice_public_key,
            bob_public_key,
            alice_transfer_amount,
            bob_encrypted_transfer_amount,
            bob_encrypted_transfer_amount_other,
        );
        let result = ConfidentialTransfer::confidential_transfer(
            Origin::signed(alice_address),
            bob_address,
            proof.0,
            transaction_params,
        );
        assert_ok!(result);

        // balance transition check
        let alice_balance = ConfidentialTransfer::total_balance(&alice_address);
        let alice_raw_balance = alice_balance.decrypt(alice_private_key);
        let bob_balance = ConfidentialTransfer::total_balance(&bob_address);
        let bob_raw_balance = bob_balance.decrypt(bob_private_key);

        assert_eq!(alice_raw_balance.unwrap() as u16, alice_after_balance);
        assert_eq!(bob_raw_balance.unwrap() as u16, bob_after_balance);
    })
}
```
With above tests, we can confirm that confidential transfer works correctly. You can check the `confidential_transfer` example [here](https://github.com/KogarashiNetwork/Kogarashi/confidential_transfer.rs). Happy hacking!

