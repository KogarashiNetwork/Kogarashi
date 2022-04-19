# Zero Network
[![CI Check](https://github.com/zero-network/zero/actions/workflows/ci.yml/badge.svg)](https://github.com/zero-network/zero/actions/workflows/ci.yml) [![Repository](https://img.shields.io/badge/github-zero-blueviolet?logo=github)](https://github.com/zero-network/zero) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)  

<img width="500" alt="architecture" src="https://user-images.githubusercontent.com/39494661/163749008-3ad6fa47-9771-419b-98de-7a85cedaa2c7.jpg">

Zero Network is a privacy specialized blockchain. This allows us the confidential transfers and confidential smart contracts. These functionalities are designed relying on only the **cryptographic hardness assumptions** instead `L2 technologies`, `TEE` and `centralized security assumption`.

## Abstract
The **Zero Network** is the `substrate-based` blockchain and that transaction information are totally hided with cryptography. This is going to be deployed as [`Polkadot`](https://polkadot.network/) parachain. We are also implementing `plonk` which has compatible with `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec) and, is optimized by assembly and latest algorithm. We are going to support following functionalities.

- Confidential Transfers
- Confidential Smart Contracts
- Zero Knowledge Contract Development Tool
- Create Proof for Contract Constraints
- Client Wallet

## Progress
**We are in research and development phase and this is alpha quality software. Please use at your own risk**.

We are supporting the confidential transactions for transfers and contracts executions. We are focusing on ensuring the `security assumption` and `privacy system` so after the research amd development phase, we are going to improve the performance by optimization of bytecode and libraries. We are also planning to support the `anonymous` and `rollup` transactions.

## Directory Structure
- `/node`: The substrate-based blockchain implementation.
- `/primitive`: The primitive components implementations which is compatible with [`Polkadot`](https://polkadot.network/).
    - `/elgamal`: The `lifted-ElGamal` encrypiton implementation.
    - `/jubjub`: The `Twisted Edwards curve` implementation.
    - `/snarks`: The [`Polkadot`](https://polkadot.network/) friendly and high performance `plonk` research and development.
- `/pallets`: The `pallet` implementations which are used on runtime.
    - `/confidential_smart_contract`: The confidential smart contract execution pallet extention of [`contracts`](https://github.com/paritytech/substrate/tree/master/frame/contracts).
    - `/confidential_transfer`: The confidential transfer pallet using encrypted balance by `lifted-ElGamal` enctyption extention of [`balance`](https://github.com/paritytech/substrate/tree/master/frame/balances).
    - `/zkink`: The [`Substrate`](https://substrate.io/) confidential smart contract `eDSL` and compiler extention of [`ink!`](https://github.com/paritytech/ink/tree/v3.0.0).

## Test
The `cargo-make` we are using.

- Unit Test
```
makers test
```

## Documentation

The white paper is work in progress. You can find the light paper [here](https://zero-network.github.io/).

## Reference

- [Polkadot](https://polkadot.network/)
- [Substrate](https://substrate.io/)
- [Plonk](https://eprint.iacr.org/2019/953.pdf)
- [Lifted-ElGamal Enctyption](https://github.com/herumi/mcl/blob/master/misc/she/she.pdf)
- [Zether](https://crypto.stanford.edu/~buenz/papers/zether.pdf)
- [Zexe](https://eprint.iacr.org/2018/962.pdf)

## License
Copyright 2020-2022 The Artree LLC.

This software is under the `Apache License`.
You can check more detail [here](./LICENSE).
