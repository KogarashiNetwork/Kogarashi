# Primitive
The `Zero Network` consist of cryptographic primitives which are compatible with `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec). Followings are short summary of primitives.

- [zero-crypto](../book/3_6_crypto.md) [![crates.io badge](https://img.shields.io/crates/v/zero-crypto.svg)](https://crates.io/crates/zero-crypto)

The `zero-crypto` crate is in charge of basic cryptographic primitive. This includes `Field`, `Curve`, `ExtensionField` and so on, and allows us to easily setup cryptocraphy implementation without implementing actual algorithms and test automatically.

- [zero-jubjub](../book/3_5_jubjub.md) [![crates.io badge](https://img.shields.io/crates/v/zero-jubjub.svg)](https://crates.io/crates/zero-jubjub)

The `zero-jubjub` crate is in charge of `Jubjub` curve arithmetic. This supports `Jubjub` rational point additive and scalar by finite field.

- [zero-bls12-381](../book/3_3_bls12_381.md) [![crates.io badge](https://img.shields.io/crates/v/zero-bls12-381.svg)](https://crates.io/crates/zero-bls12-381)

The `zero-bls12-381` crate is in charge of `Bls12 381` arithmetic. This supports `Bls12 381` $G_1$ and $G_2$ rational point additive and multiplicative, and scalar by finite field, and also supports $F_q^2$, $F_q^6$ and $F_q^{12}$ extension field arithmetic.

- [zero-elgamal](../book/3_4_elgamal.md) [![crates.io badge](https://img.shields.io/crates/v/zero-elgamal.svg)](https://crates.io/crates/zero-elgamal)

The `zero-elgamal` crate is in charge of additive homomorphic `ElGamal` arithmetic. This supports `ElGamal` encryption and decription.

- [zero-pairing](../book/) [![crates.io badge](https://img.shields.io/crates/v/zero-pairing.svg)](https://crates.io/crates/zero-pairing)

The `zero-pairing` crate is in charge of `Tate Pairing` arithmetic. This supports miller loop algorithm and final exponentiation.

You can import as adding dependencies to our crate.

```toml
[dependencies]
zero-crypto = { version = "0.2.1" }
zero-jubjub = { version = "0.2.0" }
zero-bls12-381 = { version = "0.2.0" }
zero-elgamal = { version = "0.2.0" }
zero-pairing = { version = "0.2.0" }
```
