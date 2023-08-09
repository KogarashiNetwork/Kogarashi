# Primitive
The `Kogarashi Network` consist of cryptographic primitives which are compatible with `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec). Followings are short summary of primitives.

|Name|Crates.io|Documentation|Description|
|----|---------|-------------|-----------|
| [`zkstd`] | [![crates.io badge](https://img.shields.io/crates/v/zkstd.svg)](https://crates.io/crates/zkstd) | [![Documentation](https://docs.rs/zkstd/badge.svg)](https://docs.rs/zkstd)|The `zkstd` crate is in charge of basic cryptographic primitive. This includes `Field`, `Curve`, `ExtensionField` and so on, and allows us to easily setup cryptocraphy implementation without implementing actual algorithms and test automatically.|
| [`jub-jub`] | [![crates.io badge](https://img.shields.io/crates/v/jub-jub.svg)](https://crates.io/crates/jub-jub) | [![Documentation](https://docs.rs/jub-jub/badge.svg)](https://docs.rs/jub-jub)|The `jub-jub` crate is in charge of `Jubjub` curve arithmetic. This supports `Jubjub` rational point additive and scalar by finite field.|
| [`bls-12-381`] | [![crates.io badge](https://img.shields.io/crates/v/bls-12-381.svg)](https://crates.io/crates/bls-12-381) | [![Documentation](https://docs.rs/bls-12-381/badge.svg)](https://docs.rs/bls-12-381)|The `bls-12-381` crate is in charge of `Bls12 381` arithmetic. This supports `Bls12 381` G_1 and G_2 rational point additive and multiplicative, and scalar by finite field, and also supports F_q^2, F_q^6 and F_q^{12} extension field arithmetic.|
| [`she-elgamal`] | [![crates.io badge](https://img.shields.io/crates/v/she-elgamal.svg)](https://crates.io/crates/she-elgamal) | [![Documentation](https://docs.rs/she-elgamal/badge.svg)](https://docs.rs/she-elgamal)|The `she-elgamal` crate is in charge of additive homomorphic `ElGamal` arithmetic. This supports `ElGamal` encryption and decription.|
| [`ec-pairing`] | [![crates.io badge](https://img.shields.io/crates/v/ec-pairing.svg)](https://crates.io/crates/ec-pairing) | [![Documentation](https://docs.rs/ec-pairing/badge.svg)](https://docs.rs/ec-pairing)|The `ec-pairing` crate is in charge of `Tate Pairing` arithmetic. This supports miller loop algorithm and final exponentiation.|

[//]: # (primitive)

[`zkstd`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/zkstd
[`jub-jub`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/jubjub
[`bls-12-381`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/bls12_381
[`ec-pairing`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/pairing
[`she-elgamal`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/elgamal
[`poly-commit`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/poly
[`red-jubjub`]: https://github.com/KogarashiNetwork/Kogarashi/tree/master/primitive/redjubjub
