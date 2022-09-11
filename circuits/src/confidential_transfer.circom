pragma circom 2.0.0;

/*This circuit template checks that c is the multiplication of a and b.*/

template ConfidentialTransfer() {

   // Declaration of signals.
   signal input a;
   signal input b;
   signal output c;

   // Constraints.
   c <== a * b;
}

component main = ConfidentialTransfer();
