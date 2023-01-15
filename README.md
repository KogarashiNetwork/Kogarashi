# Zero Network
[![Merged Check](https://github.com/zero-network/zero/actions/workflows/merged.yml/badge.svg)](https://github.com/zero-network/zero/actions/workflows/merged.yml) [![Repository](https://img.shields.io/badge/github-zero-blueviolet?logo=github)](https://github.com/zero-network/zero) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)  

The `Zero Network` is a public blockchain capable of confidential transfers and confidential smart contracts. These functionalities rely on only the **cryptographic hardness assumption** instead `L2 technologies`, `TEE` and `centralized security assumption`.

<div align="center">
    <img width="250" alt="architecture" src="https://user-images.githubusercontent.com/39494661/163749008-3ad6fa47-9771-419b-98de-7a85cedaa2c7.jpg">
</div>

## Abstract
All public blockchains information as in users balances are literally public and can be seen by someone whoever want, so the privacy is a missing piece of blockchain. `Zero Network` hides all transactions information by cryptographic scheme mainly **ElGamal Encryption** and **Plonk**. The cryptographic schemes are following.

|Lib|Description||
|---|---|---|
|[zero-crypto](https://crates.io/crates/zero-crypto)|Abstract algebra and zk-Snarks primitive implementation as in field, curve and extension field, pairing.|$F_r,F_q,F_{q^2},F_{q^6},F_{q^{12}},E(F_p),E'(F_{q^2}),poly(F_r)$|
|[zero-jubjub](https://crates.io/crates/zero-jubjub)|Jubjub curve implementation for circuit domain.|$ax^2 + y^2 = 1 + dx^2y^2,\ where\ a = -1,\ d=-\frac{10240}{10241}$|
|[zero-bls12-381](https://crates.io/crates/zero-bls12-381)|Bls12 381 implementation for Kate polynomial commitment domain.|$G_1:y^2 =x^3 + 4,\ G_2:y^2 = x^3+4(u+1)$|
|[zero-elgamal](https://crates.io/crates/zero-elgamal)|ElGamal encryption for encrypted number over Jubjub curve.|$(g^{r + r'}, g^{a + c} * b^{r + r'})$|
|[zero-pairing](https://crates.io/crates/zero-pairing)|Tate pairing implementation for zk-Snarks over Bls12 381 curve.|$e(aG, bH) = e(G, H)^{ab}\ where\ a,b \in F_r,~G \in G_1,~H \in G_2$|
|pallet-plonk|Plonk implementation for confidential transfer.|$gen(d) \rightarrow srs,\ com(f, srs) \rightarrow commitment,\ V_{PC} \rightarrow acc\ or\ rej$|
|pallet-encrypted-balance|Additive homomorphic encrypted balance implementation by ElGamal.|$get(address) \rightarrow (g^{r + r'}, g^{a + c} * b^{r + r'})$|
|confidential_transfer|Confidential transfer pallet implementation coupling plonk and ElGamal.|$C = g^{b^\star}y^r \land \hat C = g^{b^\star} \hat y^r \land D = g^r \land C_L/C = g^{b'}(C_R/D)^{sk} \land y = g^{sk} \land b^\star \in [0, MAX] \land b' \in [0,MAX] $|

All users balances are encrypted by the `homomorphic encryption` by default and all transactions executions are proved by the `non-interactive zero knowledge proof`. The blockchain runtime is optimized its structure and execution environment for improving encryption scheme. This blockchain supports the privacy and simplicity of use at the same time. Users balances are encrypted as default and transactions are verified by zero knowledge proof on chain. The following functionalities will be available.

- **Confidential Transfers**
- **Confidential Smart Contracts**
- **Anonymous Transfers**
- **Anonymous Smart Contract**
- **Rollup Transactions**

## Libraries

```toml
[dependencies]
zero-crypto = { version = "0.2.1" }
zero-jubjub = { version = "0.2.0" }
zero-bls12-381 = { version = "0.2.0" }
zero-elgamal = { version = "0.2.0" }
zero-pairing = { version = "0.2.0" }
```

## Progress
**We are in research and development phase and this is alpha quality software. Please use at your own risk**.

We are supporting the confidential transactions for transfers and contracts executions. We are focusing on ensuring the `security assumption` and `privacy system` so after the research amd development phase, we are going to improve the performance by optimization of bytecode and libraries. We are also planning to support the `anonymous` and `rollup` transactions. Every cryptographic libraries are totally compatible with `Substrate` runtime.

## Test

```shell
$ git submodule update --init --recursive
$ cargo test --release --all --all-features
```
or
```shell
$ docker-compose up
```

## Documentation

The white paper is work in progress. You can find the light paper and libraries documentation [here](https://zero-network.github.io/zero/).

## Reference

- [Polkadot](https://polkadot.network/)
- [Substrate](https://substrate.io/)
- [Plonk](https://eprint.iacr.org/2019/953.pdf)
- [Lifted-ElGamal Enctyption](https://github.com/herumi/mcl/blob/master/misc/she/she.pdf)
- [Zether](https://crypto.stanford.edu/~buenz/papers/zether.pdf)
- [Zexe](https://eprint.iacr.org/2018/962.pdf)

## License
Copyright 2023-2024 The Invers INC.

This software is under the `Apache License`.
You can check more detail [here](./LICENSE).
