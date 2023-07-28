#!/bin/sh

git submodule init
git submodule update
rustup override set nightly-2022-11-14
rustup target add wasm32-unknown-unknown
cd node
cargo build
./target/debug/kogarashi-node --dev
