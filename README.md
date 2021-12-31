# Zero Network
[![CI Check](https://github.com/zero-network/zero/actions/workflows/ci.yml/badge.svg)](https://github.com/zero-network/zero/actions/workflows/ci.yml) [![Repository](https://img.shields.io/badge/github-zero-blueviolet?logo=github)](https://github.com/zero-network/zero) [![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)  
Zero Network is a completely anonymous blockchain. This allows us the confidential transactions and private smart contracts. These features are realized without relying on `trusted party`, `sidechian technologies` and `TEE`, `optimistic assumption`.

## Abstract
Zero Network is the `substrate-based` blockchain and that transactions are totally concealed with cryptography. This is going to be deployed as [`Polkadot`](https://polkadot.network/) parachain. We are also implementing `zk-SNARKs` which has compatible with `no_std` and [`parity-scale-codec`](https://github.com/paritytech/parity-scale-codec) and, is optimized by assembly and algorithm. In particular, we are going to support following functionality.

- Confidential Transfers
- Private Smart Contracts
- Client Wallet with `Rust` and `Javascript`
- Rollup Transactions

## Directory Structure
- node  
The anonymous blockchain implementation.
- pallets  
The `pallet` implementations which are used in blockchain.
- snarks  
The optimized `plonk` research and development.

## Test
```
makers test
```
