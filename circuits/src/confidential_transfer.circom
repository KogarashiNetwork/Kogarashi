pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/comparators.circom";

/*
   The transfer b from Alice to Bob.
   The Alice and Bob balances, public keys and cypher texts,
   and generator, randomness are provided as public input.
   The constraints check following condition.
   - Encrypted balance is encyption of value b
   - Transfer amount is positive value
   - Alice after balance is positive value
*/

template ConfidentialTransfer() {

   signal input alice_public_key;
   signal input bob_public_key;
   signal input alice_left_encrypted_balance;
   signal input alice_right_encrypted_balance;
   signal input alice_left_encrypted_transfer_amount;
   signal input alice_right_encrypted_transfer_amount;
   signal input bob_left_encrypted_transfer_amount;
   signal input bob_right_encrypted_transfer_amount;
   signal input generator;
   signal input alice_private_key;
   signal input transfer_amount_b;
   signal input alice_after_balance;
   signal input randomness;

   signal g_powered_by_balance;
   signal g_powered_by_randomness;
   signal g_powered_by_after_balance;
   signal alice_pk_powered_by_randomness;
   signal bob_pk_powered_by_randomness;
   signal left_encrypted_after_balance;
   signal right_encrypted_after_balance;
   signal culculated_pk;

   component alice_left_encrypted_balance_constraint = IsEqual();

   g_powered_by_balance <-- generator ** transfer_amount_b;
   alice_pk_powered_by_randomness <-- alice_public_key ** randomness;

   alice_left_encrypted_balance_constraint.in[0] <== g_powered_by_balance * alice_pk_powered_by_randomness;
   alice_left_encrypted_balance_constraint.in[1] <== alice_left_encrypted_balance;
   alice_left_encrypted_balance_constraint.out === 1;

   component bob_left_encrypted_balance_constraint = IsEqual();

   bob_pk_powered_by_randomness <-- bob_public_key ** randomness;

   bob_left_encrypted_balance_constraint.in[0] <== g_powered_by_balance * bob_pk_powered_by_randomness;
   bob_left_encrypted_balance_constraint.in[1] <== bob_left_encrypted_transfer_amount;
   bob_left_encrypted_balance_constraint.out === 1;

   component alice_right_encrypted_balance_constraint = IsEqual();

   g_powered_by_randomness <-- generator ** randomness;

   alice_right_encrypted_balance_constraint.in[0] <== g_powered_by_randomness;
   alice_right_encrypted_balance_constraint.in[1] <== alice_right_encrypted_balance;
   alice_right_encrypted_balance_constraint.out === 1;

   component alice_after_balance_constraint = IsEqual();

   g_powered_by_after_balance <-- generator ** alice_after_balance;
   left_encrypted_after_balance <-- alice_left_encrypted_balance / alice_left_encrypted_transfer_amount;
   right_encrypted_after_balance <-- (alice_right_encrypted_balance / alice_right_encrypted_transfer_amount) ** alice_private_key;

   alice_after_balance_constraint.in[0] <== g_powered_by_after_balance * right_encrypted_after_balance;
   alice_after_balance_constraint.in[1] <== left_encrypted_after_balance;
   alice_after_balance_constraint.out === 1;

   component alice_key_constraint = IsEqual();

   culculated_pk <-- generator ** alice_private_key;

   alice_key_constraint.in[0] <== culculated_pk;
   alice_key_constraint.in[1] <== alice_public_key;
   alice_key_constraint.out === 1;
}

component main {public [
               alice_public_key,
               bob_public_key,
               alice_left_encrypted_balance,
               alice_right_encrypted_balance,
               alice_left_encrypted_transfer_amount,
               alice_right_encrypted_transfer_amount,
               bob_left_encrypted_transfer_amount,
               bob_right_encrypted_transfer_amount,
               generator
               ]} = ConfidentialTransfer();
