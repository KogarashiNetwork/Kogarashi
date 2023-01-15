# Bls12 381 Curve [![crates.io badge](https://img.shields.io/crates/v/zero-bls12-381.svg)](https://crates.io/crates/zero-bls12-381)
Pairing friendly bls12-381 curve.

**This crate partly uses [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) and [https://github.com/dusk-network/bls12_381](https://github.com/dusk-network/bls12_381) implementation designed by Dusk-Network team and, @str4d and @ebfull.**
We replace field and curve implementation with `zero-crypto` to make this compatible with `Substrate`.

## Overview
This crate includes field and extension fields, curve implementation. There are two curve $G1$ and $G2$ described as following.

$G1: y^2 = x^3 + 4$  
$G2: y^2 = x^3 + 4(u + 1)$

These two group supports bilinearity by pairing. Let $G$ and $H$ be generator of $G1$, and $G2$, and $e$ be pairing function. The relationship is described as following.

$e(aG, bH) = e(G, H)^{ab}$
