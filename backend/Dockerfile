FROM rust:1.70

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build && \
    rm -rf src

COPY . .

RUN cargo build

EXPOSE 3000

CMD ["cargo", "run"]