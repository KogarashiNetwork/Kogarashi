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

### [Crypto Primitives](https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive)

| Name        | Crates.io | Documentation | Description |
|-------------|-----------|-----------|-----------|
| [`zkstd`] | [![crates.io](https://img.shields.io/crates/v/zkstd.svg)](https://crates.io/crates/zkstd) | [![Documentation](https://docs.rs/zkstd/badge.svg)](https://docs.rs/zkstd) | `Crypto primitives` of **zero knowledge proof** and **homomorphic encryption**. This includes **Field**, **Curve**, **ExtensionField** and **Pairing**, **Fft**. This allows us to easily setup cryptography implementation and test without implementing actual algorithms.
| [`jub-jub`] | [![crates.io](https://img.shields.io/crates/v/jub-jub.svg)](https://crates.io/crates/jub-jub) | [![Documentation](https://docs.rs/jub-jub/badge.svg)](https://docs.rs/jub-jub) |`Jubjub curve` implementation used for **zero knowledge proof** circuit domain. This includes **finite field** operations and point arithmetic interface of **twisted Edwards curve**.
| [`bls-12-381`] |  [![crates.io](https://img.shields.io/crates/v/bls-12-381.svg)](https://crates.io/crates/bls-12-381) | [![Documentation](https://docs.rs/bls-12-381/badge.svg)](https://docs.rs/bls-12-381) |`Bls12 381 curve` implementation used for **zero knowledge proof** polynomial operation domain. This includes finite field operations, point arithmetic interface of the **Weierstrass curve**, and pairing interface.
| [`ec-pairing`] | [![crates.io badge](https://img.shields.io/crates/v/ec-pairing.svg)](https://crates.io/crates/ec-pairing) | [![Documentation](https://docs.rs/ec-pairing/badge.svg)](https://docs.rs/ec-pairing) |`Tate Pairing` implementation. This includes the Miller loop algorithm and pairing construction with pairing-friendly curve.
| [`she-elgamal`] | [![crates.io](https://img.shields.io/crates/v/she-elgamal.svg)](https://crates.io/crates/she-elgamal) | [![Documentation](https://docs.rs/she-elgamal/badge.svg)](https://docs.rs/she-elgamal) | `ElGamal Encryption` implementation. This includes a public key encryption interface that supports **additive homomorphism**.
| [`poly-commit`] | [![crates.io](https://img.shields.io/crates/v/poly-commit.svg)](https://crates.io/crates/poly-commit) | [![Documentation](https://docs.rs/poly-commit/badge.svg)](https://docs.rs/poly-commit) | `Kate Polynomial Commitment` implementation. This includes polynomial commitment and operations used for **zero knowledge proof plonk**.
| [`red-jubjub`] | [![crates.io](https://img.shields.io/crates/v/red-jubjub.svg)](https://crates.io/crates/red-jubjub) | [![Documentation](https://docs.rs/red-jubjub/badge.svg)](https://docs.rs/red-jubjub) | `Redjubjub` implementation. This includes **RedDSA** interfact and public key encryption algorithm.

### [Pallet Functionalities](https://github.com/KogarashiNetwork/Kogarashi/tree/master/pallets)

|Name|Documentation|Description|
|----|-------------|-----------|
| [`pallet-plonk`] | [Plonk Tutorial](https://kogarashinetwork.github.io/Kogarashi/plonk_pallet/) |$gen(d) \rightarrow srs,\ com(f, srs) \rightarrow commitment,\ V_{PC} \rightarrow acc\ or\ rej$|
| [`pallet-encrypted-balance`] |-|$get(address) \rightarrow (g^{r + r'}, g^{a + c} * b^{r + r'})$|
| [`confidential_transfer`] | [Confidential Transfer Tutorial](https://kogarashinetwork.github.io/Kogarashi/confidential_transfer/) |$C = g^{b^\star}y^r \land \hat C = g^{b^\star} \hat y^r \land D = g^r \land C_L/C = g^{b'}(C_R/D)^{sk} \land y = g^{sk} \land b^\star \in [0, MAX] \land b' \in [0,MAX] $|

## Setup Node

```shell
$ sh scripts/setup.sh
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

## Status

**We are in research and development phase and this is alpha quality software. Please use at your own risk**.

## License
Copyright 2023-2024 The Invers INC.

This software is under the `Apache License`.
You can check more detail [here](./LICENSE).

## Follow Us

[Website](https://kogarashi-network.com/) | [Twitter](https://twitter.com/KogarashiCrypto) | [Discord](https://discord.gg/g3q7tsHKTd) | [Github](https://github.com/KogarashiNetwork) | [Documentation](https://kogarashinetwork.github.io/)

[//]: # (crypto primitives)

[`zkstd`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/zkstd
[`jub-jub`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/jubjub
[`bls-12-381`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/bls12_381
[`ec-pairing`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/pairing
[`she-elgamal`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/elgamal
[`poly-commit`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/poly
[`red-jubjub`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/redjubjub

[//]: # (pallet functionalities)

[`pallet-plonk`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/pallets/plonk
[`pallet-encrypted-balance`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/pallets/encrypted_balance
[`confidential_transfer`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/pallets/confidential_transfer
