# Zero Network
[![CI Check](https://github.com/zero-network/zero/actions/workflows/ci.yml/badge.svg)](https://github.com/zero-network/zero/actions/workflows/ci.yml) [![Repository](https://img.shields.io/badge/github-zero-blueviolet?logo=github)](https://github.com/zero-network/zero) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)  
Zero Network is a completely anonymous blockchain. This allows us the anonymous transfers and privacy preserving smart contracts. These functionalities are designed relying on only the **cryptographic hardness assumptions** instead `L2 technologies`, `TEE` and `centralized security assumption`.

## Abstract
The **Zero Network** is the `substrate-based` blockchain and that transaction information are totally hided with cryptography. This is going to be deployed as [`Polkadot`](https://polkadot.network/) parachain. We are also implementing `plonk` which has compatible with `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec) and, is optimized by assembly and latest algorithm. We are going to support following functionalities.

- Confidential Transfers
- Privacy Preserving Smart Contracts
- Create Transaction Validity Proof
- Client Wallet

## Progress
**We are in research and development phase and this is alpha quality software. Please use at your own risk**. We are supporting the confidential transactions for transfers and contracts executions. We are focusing on ensuring the `security assumption` and `privacy system` so after the research amd development phase, we are going to improve the performance by optimization of bytecode and libraries. We are also planning to support the `anonymous` transactions and `rollup` transactions.

## Directory Structure
- `node`  
The substrate-based blockchain implementation.
- `primitive`  
The primitive components implementations which is compatible with [`Polkadot`](https://polkadot.network/).
    - `snarks`  
    The [`Polkadot`](https://polkadot.network/) friendly and high performance `plonk` research and development.
- `pallets`  
The `pallet` implementations which are used in runtime.
    - `zkink`  
    The [`Substrate`](https://substrate.io/) privacy preserving smart contract `eDSL` and compiler forked from [`ink!`](https://github.com/paritytech/ink/tree/v3.0.0).
    - `confidential_transfer`  
    The balance encrypted by `lifted-ElGamal` enctyption.

## Test
The `cargo-make` we are using.

```
makers test
```

## License
Copyright 2020-2022 The Artree LLC.

This software is under the `Apache License`.
You can check it more detail [here](./LICENSE).
