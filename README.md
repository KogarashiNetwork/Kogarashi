# Kogarashi Network [![Discord](https://dcbadge.vercel.app/api/server/g3q7tsHKTd?style=social&compact=true)](https://discord.gg/g3q7tsHKTd)
[![Merged Check](https://github.com/KogarashiNetwork/Kogarashi/actions/workflows/merged.yml/badge.svg)](https://github.com/KogarashiNetwork/Kogarashi/actions/workflows/merged.yml) [![crates.io badge](https://img.shields.io/crates/v/zero-network.svg)](https://crates.io/crates/zero-network) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/Kogarashi/branch/master/graph/badge.svg?token=QDWPAPMKLT)](https://codecov.io/gh/KogarashiNetwork/Kogarashi) [![dependency status](https://deps.rs/crate/zero-network/0.1.10/status.svg)](https://deps.rs/crate/zero-network/0.1.10)

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
| [zero-crypto](./master/primitive/crypto) | [![crates.io](https://img.shields.io/crates/v/zero-crypto.svg)](https://crates.io/crates/zero-crypto) | [![Documentation](https://docs.rs/zero-crypto/badge.svg)](https://docs.rs/zero-crypto) | The `zero-crypto` crate is in charge of basic cryptographic primitive. This includes `Field`, `Curve`, `ExtensionField` and so on, and allows us to easily setup cryptocraphy implementation without implementing actual algorithms and test automatically.
| [zero-jubjub](./master/primitive/jubjub) | [![crates.io](https://img.shields.io/crates/v/zero-jubjub.svg)](https://crates.io/crates/zero-jubjub) | [![Documentation](https://docs.rs/zero-jubjub/badge.svg)](https://docs.rs/zero-jubjub) |The `zero-jubjub` crate is in charge of `Jubjub` curve arithmetic. This supports `Jubjub` rational point additive and scalar by finite field.
| [zero-bls12-381](./master/primitive/bls12_381) |  [![crates.io](https://img.shields.io/crates/v/zero-bls12-381.svg)](https://crates.io/crates/zero-bls12-381) | [![Documentation](https://docs.rs/zero-bls12-381/badge.svg)](https://docs.rs/zero-bls12-381) |The `zero-bls12-381` crate is in charge of `Bls12 381` arithmetic. This supports `Bls12 381` G_1 and G_2 rational point additive and multiplicative, and scalar by finite field, and also supports F_q^2, F_q^6 and F_q^{12} extension field arithmetic.
| [zero-elgamal](./master/primitive/elgamal) | [![crates.io](https://img.shields.io/crates/v/zero-elgamal.svg)](https://crates.io/crates/zero-elgamal) | [![Documentation](https://docs.rs/zero-elgamal/badge.svg)](https://docs.rs/zero-elgamal) | The `zero-elgamal` crate is in charge of additive homomorphic `ElGamal` arithmetic. This supports `ElGamal` encryption and decription.
| [zero-pairing](./master/primitive/pairing) | [![crates.io badge](https://img.shields.io/crates/v/zero-pairing.svg)](https://crates.io/crates/zero-pairing) | [![Documentation](https://docs.rs/zero-pairing/badge.svg)](https://docs.rs/zero-pairing) | The `zero-pairing` crate is in charge of `Tate Pairing` arithmetic. This supports miller loop algorithm and final exponentiation.

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
$ ./target/debug/node-template --dev
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
