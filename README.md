# Kogarashi Network
[![Merged Check](https://github.com/KogarashiNetwork/Kogarashi/actions/workflows/merged.yml/badge.svg)](https://github.com/KogarashiNetwork/Kogarashi/actions/workflows/merged.yml) [![crates.io badge](https://img.shields.io/crates/v/zero-network.svg)](https://crates.io/crates/zero-network) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/Kogarashi/branch/master/graph/badge.svg?token=QDWPAPMKLT)](https://codecov.io/gh/KogarashiNetwork/Kogarashi) [![Discord](https://dcbadge.vercel.app/api/server/BYJuZB9e?style=social&compact=true)](https://discord.gg/BYJuZB9e)

<div align="center">
    <img alt="image" src="https://user-images.githubusercontent.com/39494661/231044282-e3f6f4a1-347c-4eb9-b5ca-04c5da4b3d08.png">
</div>

The `Kogarashi Network` is a public blockchain capable of privacy transfers and smart contracts. These functionalities rely on only the **cryptographic hardness assumption** instead `L2 technologies`, `TEE` and `centralized security assumption`. All public blockchains information as in users balances are literally public and the privacy is a missing piece of blockchain. `Kogarashi Network` hides all transactions information by cryptographic scheme mainly **ElGamal Encryption** and **Plonk**. The cryptographic schemes are following.

## Library

All users balances are encrypted by the `homomorphic encryption` by default and all transactions executions are proved by the `non-interactive zero knowledge proof`. The blockchain runtime is optimized its structure and execution environment for improving encryption scheme. This blockchain supports the privacy and simplicity of use at the same time. Users balances are encrypted as default and transactions are verified by zero knowledge proof on chain.

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

## Documentation

We describe technical stuff and how to use libraries in [here](https://zero-network.github.io/zero/).  
You can also check with markdown [document](./book/SUMMARY.md).  
Original grant proposal is [here](https://github.com/w3f/Grants-Program/blob/master/applications/zero-network.md).

## Test

```shell
$ git submodule update --init --recursive
$ cargo test --release --all --all-features
```

## Progress
**We are in research and development phase and this is alpha quality software. Please use at your own risk**.

We are supporting the confidential transfers now and working on confidential contracts executions. We are also planning to support the `anonymous` and `rollup` transactions.

## License
Copyright 2023-2024 The Invers INC.

This software is under the `Apache License`.
You can check more detail [here](./LICENSE).
