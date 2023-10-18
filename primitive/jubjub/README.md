# Jubjub Curve
[![CI](https://github.com/KogarashiNetwork/jubjub/actions/workflows/ci.yml/badge.svg)](https://github.com/KogarashiNetwork/jubjub/actions/workflows/ci.yml) [![crates.io badge](https://img.shields.io/crates/v/jub-jub.svg)](https://crates.io/crates/jub-jub) [![Documentation](https://docs.rs/jub-jub/badge.svg)](https://docs.rs/jub-jub) [![crates.io badge](https://img.shields.io/crates/v/jub-jub.svg)](https://crates.io/crates/jub-jub) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE) [![codecov](https://codecov.io/gh/KogarashiNetwork/jubjub/branch/master/graph/badge.svg?token=5NZWA26BXB)](https://codecov.io/gh/KogarashiNetwork/jubjub) [![dependency status](https://deps.rs/crate/jub-jub/latest/status.svg)](https://deps.rs/crate/jub-jub/latest)

This crate provides jubjub curve arithmetic and also supports fully `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec).

## Specification
The Jubjub curve is one of twisted edwards curve.

- Twisted Edwards Curve

$$
ax^2 + y^2 \equiv 1 + dx^2y^2
$$

- Addition Law

$$
(x_3 = \frac{x_1y_1 + y_1x_1}{1 + dx_1x_1y_1y_1}, y_3 = \frac{y_1y_1 + ax_1x_1}{1 - dx_1x_1y_1y_1})
$$

- Params

|Name|Value|
|---|---|
|prime|$0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001$|
|$a$|$0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000000$|
|order|$0xe7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb7$|
|$d$|$-(\frac{10240}{10241})$|

## Test

```shell
$ cargo test
```
