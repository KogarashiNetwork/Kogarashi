# Zero Network
[![Merged Check](https://github.com/zero-network/zero/actions/workflows/merged.yml/badge.svg)](https://github.com/zero-network/zero/actions/workflows/merged.yml) [![Repository](https://img.shields.io/badge/github-zero-blueviolet?logo=github)](https://github.com/zero-network/zero) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)  

The Zero Network is a public blockchain capable of confidential transfers and confidential smart contracts. These functionalities build upon only the **cryptographic hardness assumption** not `L2 technologies`, `TEE` and `centralized security assumption`.

## Motivation
All public blockchains information for example users balances are literally public and can be seen by someone whoever want so the privacy is a missing piece of blockchain. On **Zero Network**, all transactions information are totally hided by cryptographic scheme. All users balances are encrypted by the `homomorphic encryption` by default and all transactions executions are proved by the `non-interactive zero knowledge proof`. The blockchain runtime is optimized its structure and execution environment for improving encryption scheme. This blockchain supports the privacy and simplicity of use at the same time.

## Plan
This is going to be deployed as [`Polkadot`](https://polkadot.network/) parachain. We are also implementing `plonk` which has compatible with `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec) and, is optimized by assembly and latest algorithm. We are going to support following functionalities.

<center>
<img width="500" alt="architecture" src="https://user-images.githubusercontent.com/39494661/163749008-3ad6fa47-9771-419b-98de-7a85cedaa2c7.jpg">
</center>

- Confidential Transfers
- Confidential Smart Contracts
- Zero Knowledge Contract Development Tool
- Create Proof for Contract Constraints
- Client Wallet

## Progress
**We are in research and development phase and this is alpha quality software. Please use at your own risk**.

We are supporting the confidential transactions for transfers and contracts executions. We are focusing on ensuring the `security assumption` and `privacy system` so after the research amd development phase, we are going to improve the performance by optimization of bytecode and libraries. We are also planning to support the `anonymous` and `rollup` transactions.

## Test
The `cargo-make` we are using.

- Unit Test
```
git submodule init
git submodule update
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
- [Zerochain](https://github.com/LayerXcom/zero-chain)

## License
Copyright 2020-2023 The Invers INC.

This software is under the `Apache License`.
You can check more detail [here](./LICENSE).
