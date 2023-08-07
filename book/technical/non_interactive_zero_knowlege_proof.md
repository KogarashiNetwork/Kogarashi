# Non Interactive Zero Knowledge Proof

The `Non Interactive Zero Knowledge Proof` referred as to `NIZK` prove the statement without revealing any information about the statement. There are some types of `NIZK` for example `SNORKs`, `STARKs` and so on. In this section, we describe the `SNARKs` and especially [`Pinocchio Protocol`](https://eprint.iacr.org/2013/279.pdf). It's a little bit complicated technology so I divide into three parts.

## Abstract

The `SNARKs` converts the computation problems into the polynomial equations. We can not only hide the computation itself but also verify the computation faster than compute it again. That's why the `SNARKs` is also used for scaling solution for example `zk rollup`.

## Detail

The `SNARKs` has three steps.

1. [QAP](qap.md)

Converting computation which we want to prove without revealing additional information into polynomial equations. In [`Pinocchio Protocol`](https://eprint.iacr.org/2013/279.pdf), we need to generate polynonial equations for each computation. The polynomial equations are decided for corresponding computation.

2. [Polynomail Commitment](polynomial_commitment.md)

Hiding the secret as polynomial coefficients, opening with evaluating that polynomials at point, and we can prove the computation was done correctly.

3. [Homomorphic Encryption](homomorphic_encryption.md)

The polynomial coefficients are encrypted to keep the secret so we need perform evaluation with remaining encrypted. The `Homomorphic Encryption` can perform the multiple time addition and one time multiplication for encrypted number using elliptic curve and pairing.
