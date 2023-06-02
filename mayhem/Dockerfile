# Build Fuzzers
FROM rustlang/rust:nightly as builder

RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y build-essential clang-11 llvm-11-tools cmake clang curl binutils-dev libbfd-dev libunwind8-dev libblocksruntime-dev pkg-config libudev-dev libssl-dev && \
    ln -s /usr/bin/llvm-config-11 /usr/bin/llvm-config

RUN rustup component add rust-src
RUN cargo install -f honggfuzz

ADD . /leftwm
WORKDIR /leftwm

RUN cd ./leftwm/src && sed -i 's/^mod config;$/pub mod config;/' lib.rs
RUN sed -i 's/fn absolute_path/pub fn absolute_path/g' config.rs
RUN cd ../../
RUN cd ./leftwm/fuzz && cargo +nightly hfuzz build

# Move Fuzzers to a clean image
FROM ubuntu:20.04

COPY --from=builder leftwm/leftwm/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/is_program_in_path /
COPY --from=builder leftwm/leftwm/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/absolute_path /