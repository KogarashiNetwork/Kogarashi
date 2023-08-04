FROM alpine:3.18.2

ENV PATH $PATH:/root/.cargo/bin

ENV RUSTFLAGS "-C target-feature=-crt-static"

ENV TOOLCHAIN nightly-2022-11-14

RUN apk add --no-cache --update-cache \
    curl clang15 clang15-dev musl-dev git gcc g++ protoc llvm-dev bash

RUN curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --profile minimal --default-toolchain $TOOLCHAIN &&\
    rustup target add wasm32-unknown-unknown --toolchain $TOOLCHAIN &&\
    rustup component add rustfmt --toolchain $TOOLCHAIN
