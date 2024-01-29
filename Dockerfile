FROM alpine:3.18.2

ENV PATH="$PATH:/root/.cargo/bin" \
    RUSTFLAGS="-C target-feature=-crt-static" \
    TOOLCHAIN="nightly-2022-11-14"

RUN apk add --no-cache --update-cache \
    curl clang15 clang15-dev git gcc g++ protoc llvm-dev bash openssl-dev && \
    curl https://sh.rustup.rs -sSf | \
    sh -s -- -y --profile minimal --default-toolchain $TOOLCHAIN &&\
    rustup target add wasm32-unknown-unknown --toolchain $TOOLCHAIN &&\
    rustup component add rustfmt --toolchain $TOOLCHAIN

WORKDIR /app

COPY ./sample .

# CMD cargo test --release
