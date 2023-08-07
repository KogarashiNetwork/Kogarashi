# RedDSA Tutorial

In this tutorial, we are going to send transactions through RedDSA wallet and check whether it was processed correctly. We assume that you already unserstand what [RedDSA Signature](./reddsa_signature.md) is.

## Re-Randomization

We can generate one-time secret related to wallet private key and use it for performing some operation of transactions. After some operation, we can prove that the secret is related to private key and the operation was done correctly. In Zash Sapling, it's used for transaction auditability and proof delegation.

## Reference

[Zcash Protocol Specification, Version 2022.3.8](https://zips.z.cash/protocol/protocol.pdf#page=90)
