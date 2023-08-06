#!/bin/sh

rustup target add wasm32-unknown-unknown
cd node && cargo build
./target/debug/kogarashi-node --dev & (cargo test && cargo test --features integration cli_test -- --nocapture)
