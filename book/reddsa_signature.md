# RedDSA Signature

RedDSA is a Schnorr-based signature scheme, optionally supporting key re-randomization.

## Re-Randomization

We can generate one-time secret related to wallet private key and use it for performing some operation of transactions. After some operation, we can prove that the secret is related to private key and the operation was done correctly. In Zash Sapling, it's used for transaction auditability and proof delegation.

## Library

RedDSA is digital signature algorithm as the same with ECDSA and EdDSA. When we use jubjub curve on RedDSA, it's called redjubjub. It's actual scheme as the same with secp256k1 and ed25519. We have redjubjub implementation [here](https://github.com/KogarashiNetwork/redjubjub).

## Reference

[Zcash Protocol Specification, Version 2022.3.8](https://zips.z.cash/protocol/protocol.pdf#page=90)
