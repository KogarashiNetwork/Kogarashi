FROM rust:1.63.0

WORKDIR /app

COPY . .

RUN rustup default nightly-2022-11-14 &&\
    rustup target add wasm32-unknown-unknown --toolchain nightly-2022-11-14

RUN git submodule update --init --recursive

CMD cargo test --release --all --all-features
