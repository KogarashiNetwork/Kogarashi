# Tutorial

In this section, we describe how to use the pallet for privacy-preserving transactions.

- [pallet-plonk](../tutorial/plonk_pallet.md)
- [pallet-encrypted-balance](../encrypted_balance.md)
- [confidential_transfer](../tutorial/confidential_transfer_pallet.md)

You can check [Frequent Errors](../frequent_errors.md) when the error happens.

## Abstract
The privacy-preserving transactions consists of several pallet components. We roughly explain what kind of role for each pallet has.

### [pallet-plonk](../tutorial/plonk_pallet.md)

`plonk` is a zk-Snarks scheme and allows us to prove that the computation was done correctly. We perform transaction on `off-chain` and generate the proof. The blockchain verifies the proof and approve the transaction. We define the constraints circuit for `confidential transfers` and `confidential smart contracts` by this pallet.

### [pallet-encrypted-balance](../encrypted_balance.md)

Users balances are encrypted by default. We use additive homomorphic arithmetic to hide the integer in transaction. Combining original [pallet-balance](https://github.com/paritytech/substrate/tree/v3.0.0/frame/balances) and [`ElGamal`](../elgamal.md) encryption and we implemented [pallet-encrypted-balance](../encrypted_balance.md). **This pallet can't be used only by this self, because this doesn't check the validity of additive homomorphic arithmetic**.

### [confidential_transfer](../tutorial/confidential_transfer_pallet.md)

Users can transfer without being known actual amount by others with this pallet. `plonk` checks the [`confidential transfer constraints`](../constraints/confidential_transfer_constraints.md) and [pallet-encrypted-balance](../encrypted_balance.md) performs the additive homomorphic state transition.
