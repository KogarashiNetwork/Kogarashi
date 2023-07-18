# Bls12 381 Curve [![crates.io badge](https://img.shields.io/crates/v/zero-bls12-381.svg)](https://crates.io/crates/zero-bls12-381)
Pairing friendly bls12-381 curve.

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
