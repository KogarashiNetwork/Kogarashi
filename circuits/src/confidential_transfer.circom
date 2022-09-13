pragma circom 2.0.0;

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
   signal input randomness;

   signal output alice_after_balance;

   var left = generator ** transfer_amount_b;
   var right = alice_public_key ** randomness;
   alice_left_encrypted_balance === left * right;
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
