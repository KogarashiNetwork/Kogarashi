# Kogarashi Network [![Discord](https://dcbadge.vercel.app/api/server/g3q7tsHKTd?style=social&compact=true)](https://discord.gg/g3q7tsHKTd)
[![Merged Check](https://github.com/KogarashiNetwork/Kogarashi/actions/workflows/merged.yml/badge.svg)](https://github.com/KogarashiNetwork/Kogarashi/actions/workflows/merged.yml) [![crates.io badge](https://img.shields.io/crates/v/zero-network.svg)](https://crates.io/crates/zero-network) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/Kogarashi/branch/master/graph/badge.svg?token=QDWPAPMKLT)](https://codecov.io/gh/KogarashiNetwork/Kogarashi) [![dependency status](https://deps.rs/crate/zero-network/0.1.10/status.svg)](https://deps.rs/crate/zero-network/0.1.10) 

<div align="center">
    <img alt="image" src="https://github.com/KogarashiNetwork/Kogarashi/assets/39494661/5a40d34b-8501-4fe4-a59e-2d097bde154d">
</div>

The `Kogarashi Network` is a public blockchain capable of privacy transfers and smart contracts. These functionalities rely on only the **cryptographic hardness assumption** instead `L2 technologies`, `TEE` and `centralized security assumption`. All public blockchains information as in users balances are literally public and the privacy is a missing piece of blockchain. `Kogarashi Network` hides all transactions information by cryptographic scheme mainly **ElGamal Encryption** and **Plonk**. The cryptographic schemes are following.

## Test

```shell
$ git submodule update --init --recursive
$ cargo test --all --release
```

## Library

All users balances are encrypted by the `homomorphic encryption` by default and all transactions executions are proved by the `non-interactive zero knowledge proof`. The blockchain runtime is optimized its structure and execution environment for improving encryption scheme. This blockchain supports the privacy and simplicity of use at the same time. Users balances are encrypted as default and transactions are verified by zero knowledge proof on chain.

### Crypto Primitives

|Name|Crates.io|Documentation|Description|
|----|---------|-------------|-----------|
|[zero-crypto](./primitive/crypto)|[![crates.io badge](https://img.shields.io/crates/v/zero-crypto.svg)](https://crates.io/crates/zero-crypto)|[![Documentation](https://docs.rs/zero-crypto/badge.svg)](https://docs.rs/zero-crypto)|$F_r,F_q,F_{q^2},F_{q^6},F_{q^{12}},E(F_p),E'(F_{q^2}),poly(F_r)$|
|[zero-jubjub](./primitive/jubjub)|[![crates.io badge](https://img.shields.io/crates/v/zero-jubjub.svg)](https://crates.io/crates/zero-jubjub)|[![Documentation](https://docs.rs/zero-jubjub/badge.svg)](https://docs.rs/zero-jubjub)|$ax^2 + y^2 = 1 + dx^2y^2,\ where\ a = -1,\ d=-\frac{10240}{10241}$|
|[zero-bls12-381](./primitive/bls12_381)|[![crates.io badge](https://img.shields.io/crates/v/zero-bls12-381.svg)](https://crates.io/crates/zero-bls12-381)|[![Documentation](https://docs.rs/zero-bls12-381/badge.svg)](https://docs.rs/zero-bls12-381)|$G_1:y^2 =x^3 + 4,\ G_2:y^2 = x^3+4(u+1)$|
|[zero-elgamal](./primitive/elgamal)|[![crates.io badge](https://img.shields.io/crates/v/zero-elgamal.svg)](https://crates.io/crates/zero-elgamal)|[![Documentation](https://docs.rs/zero-elgamal/badge.svg)](https://docs.rs/zero-elgamal)|$(g^{r + r'}, g^{a + c} * b^{r + r'})$|
|[zero-pairing](./primitive/pairing)|[![crates.io badge](https://img.shields.io/crates/v/zero-pairing.svg)](https://crates.io/crates/zero-pairing)|[![Documentation](https://docs.rs/zero-pairing/badge.svg)](https://docs.rs/zero-pairing)|$e(aG, bH) = e(G, H)^{ab}\ where\ a,b \in F_r,~G \in G_1,~H \in G_2$|

### Pallet Functionalities

|Name|Documentation|Description|
|----|-------------|-----------|
|[pallet-plonk](./pallets/plonk)|[Plonk Tutorial](https://kogarashinetwork.github.io/Kogarashi/plonk_pallet/)|$gen(d) \rightarrow srs,\ com(f, srs) \rightarrow commitment,\ V_{PC} \rightarrow acc\ or\ rej$|
|[pallet-encrypted-balance](./pallets/encrypted_balance)|-|$get(address) \rightarrow (g^{r + r'}, g^{a + c} * b^{r + r'})$|
|[confidential_transfer](./pallets/confidential_transfer)|[Confidential Transfer Tutorial](https://kogarashinetwork.github.io/Kogarashi/confidential_transfer/)|$C = g^{b^\star}y^r \land \hat C = g^{b^\star} \hat y^r \land D = g^r \land C_L/C = g^{b'}(C_R/D)^{sk} \land y = g^{sk} \land b^\star \in [0, MAX] \land b' \in [0,MAX] $|

## Documentation

We describe technical stuff and how to use libraries in [here](https://kogarashinetwork.github.io/Kogarashi/).

### Dev

```
$ mkdocs serve
```

### Build

```
$ mkdocs build
```

## Test

```shell
$ git submodule update --init --recursive
$ cargo test --release --all --all-features
```

## Progress
**We are in research and development phase and this is alpha quality software. Please use at your own risk**.

## License
Copyright 2023-2024 The Invers INC.

This software is under the `Apache License`.
You can check more detail [here](./LICENSE).
