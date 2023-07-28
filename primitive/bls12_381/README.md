# Bls12 381 Curve
[![CI](https://github.com/KogarashiNetwork/bls12_381/actions/workflows/ci.yml/badge.svg)](https://github.com/KogarashiNetwork/bls12_381/actions/workflows/ci.yml) [![crates.io badge](https://img.shields.io/crates/v/bls-12-381.svg)](https://crates.io/crates/bls-12-381) [![Documentation](https://docs.rs/bls-12-381/badge.svg)](https://docs.rs/bls-12-381)  [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/bls12_381/branch/master/graph/badge.svg?token=W83P6U2QKE)](https://codecov.io/gh/KogarashiNetwork/bls12_381) [![dependency status](https://deps.rs/crate/zero-bls12-381/latest/status.svg)](https://deps.rs/crate/zero-bls12-381/latest)

Pairing friendly bls12-381 curve supports fully `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec).

## Overview
This crate includes field and extension fields, curve implementation. There are two curve $G1$ and $G2$ described as following.

$G1: y^2 = x^3 + 4$

$G2: y^2 = x^3 + 4(u + 1)$

These two group supports bilinearity by pairing. Let $G$ and $H$ be generator of $G1$, and $G2$, and $e$ be pairing function. The relationship is described as following.

$e(aG, bH) = e(G, H)^{ab}$

## Test

```shell
$ cargo test
```
