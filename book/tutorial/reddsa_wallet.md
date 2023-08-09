# RedDSA Tutorial

In this tutorial, we are going to send transactions through the RedDSA wallet and check whether it was processed correctly. We assume that you already understand what [RedDSA Signature](../technical/reddsa_signature.md) is.

The steps are following.

1. Run RedDSA compatible Substrate Node
2. Compile client wallet
3. Generate Wallet
4. Check Balance
5. Transfer Asset

The final goal is to check whether the RedDSA client wallet works through an RPC call to Substrate Node.

You can find wallet client [here](https://github.com/KogarashiNetwork/Kogarashi/tree/master/node/client) and RedDSA compatible Substrate Node [here](https://github.com/KogarashiNetwork/zksubstrate).
We already implemented RedDSA runtime [here](https://github.com/KogarashiNetwork/Kogarashi/tree/master/zkruntime) so you can import this runtime to your Substrate Node.

## 1. Run RedDSA compatible Substrate Node

First of all, we would like to run the RedDSA-compatible Substrate Node..

Let's clone the RedDSA Substrate Node with the following command.

```sh
$ git clone git@github.com:KogarashiNetwork/Kogarashi.git
$ cd Kogarashi
$ git submodule update -i
```

After cloning the repository and submodule dependencies, run the RedDSA Substrate Node with the following command.

- Native

```sh
$ sh scripts/setup.sh
```

- Docker

```sh
$ docker-compose up
```

## 2. Compile client wallet

Next, we would like to set up RedDSA Wallet client in [here](https://github.com/KogarashiNetwork/Kogarashi/tree/master/node/client). We can setup with the following command in the root directly.

```sh
$ rustup target add wasm32-unknown-unknown
$ cd node/client
$ cargo build
```

And after compilation, we can check whether the wallet client is ready with the following command.

```sh
$ cargo run list
```

If the test accounts list is displayed, the wallet is ready.

## 3. Generate Wallet

We would like to generate a wallet with the following command.

```sh
$ cargo run init
```

If it's successful in generating a wallet, it would display your address and seed.

## 4. Check Balance

Let's check the account balance with the following and whether it's zero.

```sh
$ cargo run balance
```

If you would like to check other account balances, you can do it with the following command.

```sh
$ cargo run balance {Account Name}
```

`{Account Name}` is replaced with 'Alice', and 'Bob' and displayed accounts with `cargo run list`.

## 5. Transfer Asset

Finally, we would like to transfer assets but you don't have any assets now so let's fund with the following command.

```sh
$ cargo run fund
```

Right now, you have some assets and can check how much you want with the following command.

```sh
$ cargo run balance
```

Then, let's transfer your asset. We need to decide who and how much we transfer. If you want to transfer 1000 to Bob, it would be the following.

```sh
$ cargo run transfer Bob 1000
```

You can check whether Bob's amount is increased with the following command.

```sh
$ cargo run balance Bob
```

You can find a sample RedDSA implementation [here](https://github.com/KogarashiNetwork/Kogarashi/blob/master/node/client/src/rpc/extrinsic.rs).

Happy hacking!

## Reference

[Zcash Protocol Specification, Version 2022.3.8](https://zips.z.cash/protocol/protocol.pdf#page=90)
