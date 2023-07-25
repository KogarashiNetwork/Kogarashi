FROM rust:1.63.0

WORKDIR /app/node

COPY . .

RUN apt-get update &&\
    apt-get install llvm libclang-dev -y &&\
    rustup override set nightly-2022-11-14 &&\
    rustup target add wasm32-unknown-unknown --toolchain nightly-2022-11-14

CMD bash -c "cargo build && ./target/debug/kogarashi-node --dev"
