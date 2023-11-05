# Tate Pairing
[![CI](https://github.com/KogarashiNetwork/pairing/actions/workflows/ci.yml/badge.svg)](https://github.com/KogarashiNetwork/pairing/actions/workflows/ci.yml) [![crates.io badge](https://img.shields.io/crates/v/ec-pairing.svg)](https://crates.io/crates/ec-pairing) [![Documentation](https://docs.rs/ec-pairing/badge.svg)](https://docs.rs/ec-pairing) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/pairing/branch/master/graph/badge.svg?token=RA1AA9EGYK)](https://codecov.io/gh/KogarashiNetwork/pairing) [![dependency status](https://deps.rs/crate/ec-pairing/latest/status.svg)](https://deps.rs/crate/ec-pairing/latest)

This crate provides pairing arithmetic and also supports fully `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec).

## Specification

$$
e(aG, bH) = e(G, H)^{ab}\ where\ a,b \in F_r,~G \in G_1,~H \in G_2
$$

## Test

```shell
$ cargo test
```
