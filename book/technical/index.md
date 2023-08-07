# Abstract
In this section, we would like to explain about the cryptgraphic scheme used for other privacy project.

## [Stealth Address](stealth_address.md)
The `Stealth Address` hides the recipient address by creating one time address. Shortly, creating one time address from public key and recovering that private key with using [`Diffie-Hellman`](https://ieeexplore.ieee.org/document/1055638) key exchange, users can hide who exactly receives the assets. The [`Monero`](https://www.getmonero.org/) uses this technology.

## [Pedersen Commitment](pedersen_commitment.md)
The `Pedersen Commitment` hides the transfer amount by commitment scheme. Shortly, using zero testing and generating blind factors, users can prove the amount validity without revealing actual amount. The [`Monero`](https://www.getmonero.org/) uses this technology.

## [Non Interactive Zero Knowledge Proof](non_interactive_zero_knowlege_proof.md)
The `Non Interactive Zero Knowledge Proof` proves the computation validity without revealing information about the value used with computation. In this section, we describe the `zk-SNARKs`. The [`Zcash`](https://z.cash/) uses this technology. Shortly, the [QAP](qap.md) converts computation into polynomials, the [Polynomial Commitment](polynomial_commitment.md) proves the validity of polynomials and the [Homomorphic Encryption](homomorphic_encryption.md) evaluates the polynomials without revealing raw coefficients.
