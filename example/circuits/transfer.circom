pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/comparators.circom";

/*
   The transfer b from Alice to Bob.
   The Alice and Bob balances are provided as public input.
   The constraints check following condition.
   - Transfer amount is positive value
   - Alice after balance is positive value
*/

template Transfer() {

   signal input alice_balance;
   signal input bob_balance;
   signal input transfer_amount;

   signal output alice_after_balance;

   // Transfer amount is positive value
   component transfer_amount_comp = LessThan(252);

   transfer_amount_comp.in[0] <== 0;
   transfer_amount_comp.in[1] <== transfer_amount;
   transfer_amount_comp.out === 1;

   // Alice after balance is positive value
   component alice_balance_comp = LessThan(252);

   alice_balance_comp.in[0] <== transfer_amount;
   alice_balance_comp.in[1] <== alice_balance;
   alice_balance_comp.out === 1;

   alice_after_balance <== alice_balance - transfer_amount;
}

component main {public [alice_balance, bob_balance, transfer_amount]} = Transfer();
