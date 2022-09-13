#!/usr/bin/env bash

rustup default nightly-2022-07-27
rustup target add wasm32-unknown-unknown --toolchain nightly-2022-07-27
cargo build
