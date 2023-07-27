# Kogarashi Network [![Discord](https://dcbadge.vercel.app/api/server/g3q7tsHKTd?style=social&compact=true)](https://discord.gg/g3q7tsHKTd)
[![Merged Check](https://github.com/KogarashiNetwork/Kogarashi/actions/workflows/bench.yml/badge.svg)](https://github.com/KogarashiNetwork/Kogarashi/actions/workflows/bench.yml) [![crates.io badge](https://img.shields.io/crates/v/zero-network.svg)](https://crates.io/crates/zero-network) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/Kogarashi/branch/master/graph/badge.svg?token=QDWPAPMKLT)](https://codecov.io/gh/KogarashiNetwork/Kogarashi) [![dependency status](https://deps.rs/crate/zero-network/0.1.10/status.svg)](https://deps.rs/crate/zero-network/0.1.10)

<div align="center">
    <img alt="image" src="https://github.com/KogarashiNetwork/Kogarashi/assets/39494661/5a40d34b-8501-4fe4-a59e-2d097bde154d">
</div>

Our mission is to ralize a **virtual nation** providing **economic inclusion**. Bringing Internet and IoT data to blockchain allows Dapp to issue legally bound NFT by e-sign, manage physical assets, and trade digital infra. Just a smartphone will be citizenship for this virtual nation.

We supports following functionalities.

1. Auditable Privacy Preserving Transfers
2. Zk Rollups Transfer Scaling
3. Trustless Single Node Off-Chain Oracle
4. IoT TEE Device Remote Attestation

## Library

All users balances are encrypted by the `homomorphic encryption` by default and all transactions executions are proved by the `non-interactive zero knowledge proof`. The blockchain runtime is optimized its structure and execution environment for improving encryption scheme. This blockchain supports the privacy and simplicity of use at the same time. Users balances are encrypted as default and transactions are verified by zero knowledge proof on chain.

### Crypto Primitives

| Name        | Crates.io | Documentation | Description |
|-------------|-----------|-----------|-----------|
| [zkstd](./master/primitive/zkstd) | [![crates.io](https://img.shields.io/crates/v/zkstd.svg)](https://crates.io/crates/zkstd) | [![Documentation](https://docs.rs/zkstd/badge.svg)](https://docs.rs/zkstd) | The [zkstd](https://crates.io/crates/zkstd) crate is in charge of basic cryptographic primitive. This includes **Field**, **Curve**, **ExtensionField** and so on, and allows us to easily setup cryptocraphy implementation without implementing actual algorithms and test automatically.
| [jub-jub](./master/primitive/jubjub) | [![crates.io](https://img.shields.io/crates/v/jub-jub.svg)](https://crates.io/crates/jub-jub) | [![Documentation](https://docs.rs/jub-jub/badge.svg)](https://docs.rs/jub-jub) |The [jub-jub](https://crates.io/crates/jub-jub) crate is in charge of **Jubjub** curve arithmetic. This supports **Jubjub** rational point additive and scalar by finite field.
| [bls-12-381](./master/primitive/bls12_381) |  [![crates.io](https://img.shields.io/crates/v/bls-12-381.svg)](https://crates.io/crates/bls-12-381) | [![Documentation](https://docs.rs/bls-12-381/badge.svg)](https://docs.rs/bls-12-381) |The [bls-12-381](https://crates.io/crates/bls-12-381) crate is in charge of **Bls12 381** arithmetic. This supports **Bls12 381** G_1 and G_2 rational point additive and multiplicative, and scalar by finite field, and also supports F_q^2, F_q^6 and F_q^{12} extension field arithmetic.
| [ec-pairing](./master/primitive/pairing) | [![crates.io badge](https://img.shields.io/crates/v/ec-pairing.svg)](https://crates.io/crates/ec-pairing) | [![Documentation](https://docs.rs/ec-pairing/badge.svg)](https://docs.rs/ec-pairing) | The [ec-pairing](https://crates.io/crates/ec-pairing) crate is in charge of **Tate Pairing** arithmetic. This supports miller loop algorithm and final exponentiation.
| [she-elgamal](./master/primitive/elgamal) | [![crates.io](https://img.shields.io/crates/v/she-elgamal.svg)](https://crates.io/crates/she-elgamal) | [![Documentation](https://docs.rs/she-elgamal/badge.svg)](https://docs.rs/she-elgamal) | The [she-elgamal](https://crates.io/crates/she-elgamal) crate is in charge of additive homomorphic **ElGamal** arithmetic. This supports **ElGamal** encryption and decription.
| [poly-commit](./master/primitive/poly) | [![crates.io](https://img.shields.io/crates/v/poly-commit.svg)](https://crates.io/crates/poly-commit) | [![Documentation](https://docs.rs/poly-commit/badge.svg)](https://docs.rs/poly-commit) | The [poly-commit](https://crates.io/crates/poly-commit) crate is in charge of Kate polynomial commitment and polynomial operation used for [Plonk](https://eprint.iacr.org/2019/953.pdf).
| [red-jubjub](./master/primitive/red-jubjub) | [![crates.io](https://img.shields.io/crates/v/red-jubjub.svg)](https://crates.io/crates/red-jubjub) | [![Documentation](https://docs.rs/red-jubjub/badge.svg)](https://docs.rs/red-jubjub) | The [red-jubjub](https://crates.io/crates/red-jubjub) crate is in charge of auditable functionality for the confidential transfer through **reddsa** signature algorithm.

### Pallet Functionalities

|Name|Documentation|Description|
|----|-------------|-----------|
| [pallet-plonk](./master/pallets/plonk) | [Plonk Tutorial](https://kogarashinetwork.github.io/Kogarashi/plonk_pallet/) |$gen(d) \rightarrow srs,\ com(f, srs) \rightarrow commitment,\ V_{PC} \rightarrow acc\ or\ rej$|
| [pallet-encrypted-balance](./master/pallets/encrypted_balance)|-|$get(address) \rightarrow (g^{r + r'}, g^{a + c} * b^{r + r'})$|
| [confidential_transfer](./master/pallets/confidential_transfer) | [Confidential Transfer Tutorial](https://kogarashinetwork.github.io/Kogarashi/confidential_transfer/) |$C = g^{b^\star}y^r \land \hat C = g^{b^\star} \hat y^r \land D = g^r \land C_L/C = g^{b'}(C_R/D)^{sk} \land y = g^{sk} \land b^\star \in [0, MAX] \land b' \in [0,MAX] $|

## Setup Node

```shell
$ rustup override set nightly-2022-11-14
$ rustup target add wasm32-unknown-unknown
$ cd node
$ cargo build
$ ./target/debug/kogarashi-node --dev
```

or

```
$ docker-compose up
```

## Test

```shell
$ git submodule update --init --recursive
$ cargo test --all --release
```

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

## Status

**We are in research and development phase and this is alpha quality software. Please use at your own risk**.

## License
Copyright 2023-2024 The Invers INC.

This software is under the `Apache License`.
You can check more detail [here](./LICENSE).

## Follow Us

[Website](https://kogarashi-network.com/) | [Twitter](https://twitter.com/KogarashiCrypto) | [Discord](https://discord.gg/g3q7tsHKTd) | [Github](https://github.com/KogarashiNetwork) | [Documentation](https://kogarashinetwork.github.io/Kogarashi/)
