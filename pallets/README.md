# Pallet
The `Kogarashi Network` supports privacy-preserving transactions. These functionalities are powered by `pallets`. Followings are short summary of pallets.

- [pallet-plonk](../book/4_1_plonk.md) [![crates.io badge](https://img.shields.io/crates/v/plonk-pallet.svg)](https://crates.io/crates/plonk-pallet)

The `pallet-plonk` pallet is a wrapper of plonk library and in charge of proving and verifying the validity of computation.

- [pallet-encrypted-balance](../book/4_2_encrypted_balance.md)

The `pallet-encrypted-balance` pallet provides balance encryption by default. This replaces balance storage value with encrypted value.

- [confidential_transfer](../book/4_3_confidential_transfer.md)

The `confidential_transfer` pallet provides transfer function with hiding transfer amount. This pallet is coupling `pallet-plonk` and `pallet-encrypted-balance`, and changes the balance with encryped and checks the validity of computation.

You can import as adding dependencies to our crate.

```toml
[dependencies]
pallet-plonk = { version = "0.2.3" }
```