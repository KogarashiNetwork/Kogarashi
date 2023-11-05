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

## Crypto Primitives

| Name        | Crates.io | Documentation | Description |
|-------------|-----------|-----------|-----------|
| [`zkstd`] | [![crates.io](https://img.shields.io/crates/v/zkstd.svg)](https://crates.io/crates/zkstd) | [![Documentation](https://docs.rs/zkstd/badge.svg)](https://docs.rs/zkstd) | `Crypto primitives` of **zero knowledge proof** and **homomorphic encryption**. This includes **Field**, **Curve**, **ExtensionField** and **Pairing**, **Fft**. This allows us to easily setup cryptography implementation and test without implementing actual algorithms.
| [`jub-jub`] | [![crates.io](https://img.shields.io/crates/v/jub-jub.svg)](https://crates.io/crates/jub-jub) | [![Documentation](https://docs.rs/jub-jub/badge.svg)](https://docs.rs/jub-jub) |`Jubjub curve` implementation used for **zero knowledge proof** circuit domain. This includes **finite field** operations and point arithmetic interface of **twisted Edwards curve**.
| [`bls-12-381`] |  [![crates.io](https://img.shields.io/crates/v/bls-12-381.svg)](https://crates.io/crates/bls-12-381) | [![Documentation](https://docs.rs/bls-12-381/badge.svg)](https://docs.rs/bls-12-381) |`Bls12 381 curve` implementation used for **zero knowledge proof** polynomial operation domain. This includes finite field operations, point arithmetic interface of the **Weierstrass curve**, and pairing interface.
| [`ec-pairing`] | [![crates.io badge](https://img.shields.io/crates/v/ec-pairing.svg)](https://crates.io/crates/ec-pairing) | [![Documentation](https://docs.rs/ec-pairing/badge.svg)](https://docs.rs/ec-pairing) |`Tate Pairing` implementation. This includes the Miller loop algorithm and pairing construction with pairing-friendly curve.
| [`she-elgamal`] | [![crates.io](https://img.shields.io/crates/v/she-elgamal.svg)](https://crates.io/crates/she-elgamal) | [![Documentation](https://docs.rs/she-elgamal/badge.svg)](https://docs.rs/she-elgamal) | `ElGamal Encryption` implementation. This includes a public key encryption interface that supports **additive homomorphism**.
| [`poly-commit`] | [![crates.io](https://img.shields.io/crates/v/poly-commit.svg)](https://crates.io/crates/poly-commit) | [![Documentation](https://docs.rs/poly-commit/badge.svg)](https://docs.rs/poly-commit) | `Kate Polynomial Commitment` implementation. This includes polynomial commitment and operations used for **zero knowledge proof plonk**.

## Test

```shell
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

[`zkstd`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/zkstd
[`jub-jub`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/jubjub
[`bls-12-381`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/bls12_381
[`ec-pairing`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/pairing
[`she-elgamal`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/elgamal
[`poly-commit`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/poly
