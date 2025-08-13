FROM rust:1.89 AS builder

WORKDIR /build
COPY . .
RUN cargo build --release
