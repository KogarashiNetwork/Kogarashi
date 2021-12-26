# Twisted Edwards Curve
[![crates.io badge](https://img.shields.io/crates/v/zero-jubjub.svg)](https://crates.io/crates/zero-jubjub)  
The [`Twisted Edwards Curve`](https://eprint.iacr.org/2008/013.pdf) implementation. This is totally `#![cfg_attr(not(feature = "std"), no_std)]`, support [`parity codec`](https://github.com/paritytech/parity-scale-codec). The arithmetics are optimized by the `assembly`.
