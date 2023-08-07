# RedDSA Tutorial

In this tutorial, we are going to send transactions through RedDSA wallet and check whether it was processed correctly. We assume that you already unserstand what [RedDSA Signature](../technical/reddsa_signature.md) is.

The steps are following.

1. Run RedDSA compatible Substrate Node
2. Compile client wallet
3. Generate Wallet
4. Check Balance
5. Transfer Asset

Final goal is to check whether RedDSA client wallet works through RPC call to Substrate Node.

You can find wallet client [here](https://github.com/KogarashiNetwork/Kogarashi/tree/master/node/client) and RedDSA compatible Substrate Node [here](https://github.com/KogarashiNetwork/zksubstrate).
We already implemented RedDSA runtime [here](https://github.com/KogarashiNetwork/Kogarashi/tree/master/zkruntime) so you can import this runtime to your Substrate Node.

## 1. Run RedDSA compatible Substrate Node

First of all, we would like to run the RedDSA compatible Substrate Node. There are two ways to do this. We reccomend docker compose.

Let's clone the RedDSA Substrate Node with following command.

```sh
$ git clone git@github.com:KogarashiNetwork/Kogarashi.git
$ cd Kogarashi
$ git submodule update -i
```

After cloning repository and submodule dependencies, run the RedDSA Substrate Node with following command.

### docker-compose

```sh
$ docker-compose up
```

### Native

```sh
$ sh scripts/setup.sh
```

## 2. Compile client wallet

Next, we would like to setup RedDSA Wallet client in [here](https://github.com/KogarashiNetwork/Kogarashi/tree/master/node/client). We can setup with following command in root directly.

```sh
$ rustup target add wasm32-unknown-unknown
$ cd node/client
```

And after compilation, we can check whether wallet client is ready with following command.

```sh
$ cargo run list
```

If test accounts list is displayed, the wallet is ready.

## 3. Generate Wallet

We would like to generate wallet with following command.

```sh
$ cargo run init
```

If it's successfull for generating wallet, it would display your address and seed.

## 4. Check Balance

Let's check the account balance with following and whether it's zero.

```sh
$ cargo run balance
```

If you would like to check other account balance, you can do it with following command.

```sh
$ cargo run balance {Account Name}
```

`{Account Name}` is replaced with 'Alice', 'Bob' and displayed accounts with `cargo run list`.

## 5. Transfer Asset

Finally, we would like to transfer asset but you don't have any asset now so let's fund with following command.

```sh
$ cargo run fund
```

Right now, you have some asset and can check how much you want with following command.

```sh
$ cargo run balance
```

Then, let's transfer your asset. We need to decide who and how much we transfer. If you want to transfer 1000 to Bob, it would be following.

```sh
$ cargo run transfer Bob 1000
```

You can check whether Bob amount is increased with following command.

```sh
$ cargo run balance Bob
```

You can find sample RedDSA implementation [here](https://github.com/KogarashiNetwork/Kogarashi/blob/master/node/client/src/rpc/extrinsic.rs).

Happy hacking!

## Reference

[Zcash Protocol Specification, Version 2022.3.8](https://zips.z.cash/protocol/protocol.pdf#page=90)
