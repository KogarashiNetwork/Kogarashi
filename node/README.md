# Kogarashi Node

## Environment

## Build

```sh
cargo build --release
```

## Run

### Dev

```sh
./target/release/node-template --dev
```

- Purge Cache

```sh
./target/release/node-template purge-chain --dev
```

- Logging

```sh
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```

### Prod

```sh
./target/release/node-template --release
```
