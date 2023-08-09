# Pallet
The `Kogarashi Network` supports privacy-preserving transactions. These functionalities are powered by `pallets`. Followings are short summary of pallets.

## [pallet-plonk](https://github.com/KogarashiNetwork/Kogarashi/tree/master/pallets/plonk) [![crates.io badge](https://img.shields.io/crates/v/plonk-pallet.svg)](https://crates.io/crates/plonk-pallet)


The `pallet-plonk` pallet is a wrapper of plonk library and in charge of proving and verifying the validity of computation.

You can find tutorial [here](https://kogarashinetwork.github.io/Kogarashi/tutorial/plonk_pallet/).

## [pallet-encrypted-balance](https://github.com/KogarashiNetwork/Kogarashi/tree/master/pallets/encrypted_balance)

The `pallet-encrypted-balance` pallet provides balance encryption by default. This replaces balance storage value with encrypted value.

## [confidential_transfer](https://github.com/KogarashiNetwork/Kogarashi/tree/master/pallets/confidential_transfer)

The `confidential_transfer` pallet provides transfer function with hiding transfer amount. This pallet is coupling `pallet-plonk` and `pallet-encrypted-balance`, and changes the balance with encryped and checks the validity of computation.

You can find tutorial [here](https://kogarashinetwork.github.io/Kogarashi/tutorial/confidential_transfer_pallet/).
