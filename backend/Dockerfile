FROM rust:1.89 AS builder

WORKDIR /build
COPY . .

RUN apt update
RUN apt install -y protobuf-compiler

RUN cargo build --release
