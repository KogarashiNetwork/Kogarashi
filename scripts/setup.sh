#!/bin/sh

git submodule update -i
rustup target add wasm32-unknown-unknown
cd node
cargo build
./target/debug/kogarashi-node --dev
