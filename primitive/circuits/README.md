# Circuits
**This crate includes circuits implementation used for confidential transfer.**

## Confidential Transfer Circuit
`Confidential Transfer Circuit` checks following condition.

1. Transfer amount is encrypted by sender and recipient public key.
2. Sender remaining balance is calculated correctly.
3. Sender remaining balance and transfer amount are in valid range.
