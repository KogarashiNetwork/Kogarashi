# Twisted Edwards Curve
[![crates.io badge](https://img.shields.io/crates/v/zero-jubjub.svg)](https://crates.io/crates/zero-jubjub)  
The [`Twisted Edwards Curve`](https://eprint.iacr.org/2008/013.pdf) implementation. The arithmetics are optimized by the `assembly`.

## Feature

| Name | Description | Usage |
| ---- | ---- | ---- |
| std | Feature for client | Generate proof with parallelize |
| no_std | Feature for Substrate runtime | Implement runtime module |
| asm | Feature for intel x86 | Replace arithmetic with assembly |
| parity | Feature for pallet | Derive struct as SCALE |
