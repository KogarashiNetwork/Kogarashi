# ZkStd
[![CI](https://github.com/KogarashiNetwork/zkstd/actions/workflows/ci.yml/badge.svg)](https://github.com/KogarashiNetwork/zkstd/actions/workflows/ci.yml) [![crates.io badge](https://img.shields.io/crates/v/zero-crypto.svg)](https://crates.io/crates/zkstd) [![Documentation](https://docs.rs/zkstd/badge.svg)](https://docs.rs/zkstd) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/zkstd/branch/master/graph/badge.svg?token=801ESOH5ZV)](https://codecov.io/gh/KogarashiNetwork/zkstd) [![dependency status](https://deps.rs/crate/zkstd/latest/status.svg)](https://deps.rs/crate/zkstd/latest)

This crate provides basic cryptographic implementation as in `Field`, `Curve` and `Pairing`, `Fft`, `Kzg`, and also supports fully `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec).

## Design

Cryptography libraries need to be applied optimization easily because computation cost affects users waiting time and on-chain gas cost. We design this library following two perspectives.

- The simplicity to replace with the latest algorithm
- The brevity of code by avoiding duplication

We divide arithmetic operation and interface. Arithmetic operation is concrete logic as in elliptic curve addition and so on, and the interface is trait cryptography primitive supports. And we combine them with macro. With this design, we can keep the finite field and elliptic curve implementation simple.

### Directory Structure

- [arithmetic](./src/arithmetic): the arithmetic operation of limbs, points and bit operation.
- [behave](./src/behave): the interface of cryptography components as in `Fft Field`, `Pairing Field` and so on.
- [dress](./src/dress): the macro used for implementation and in charge of combing `arithmetic` and `behave` together.
