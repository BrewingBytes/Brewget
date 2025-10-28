FROM rust:1.90-bookworm AS builder

WORKDIR /build

# Install only the necessary build dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Copy only the necessary files for dependency caching
COPY Cargo.toml Cargo.lock ./

COPY auth-service/Cargo.toml auth-service/
COPY auth-service/build.rs auth-service/
COPY email-service/Cargo.toml email-service/
COPY email-service/build.rs email-service/
COPY settings-service/Cargo.toml settings-service/
COPY settings-service/build.rs settings-service/
COPY shared-types shared-types/

COPY proto/ proto/

# Create dummy source files for dependency caching
RUN mkdir -p auth-service/src email-service/src settings-service/src \
    && echo "fn main() {}" > auth-service/src/main.rs \
    && echo "fn main() {}" > email-service/src/main.rs \
    && echo "fn main() {}" > settings-service/src/main.rs

# Build dependencies only
RUN cargo build --release

# Copy actual source code
COPY . .

# Build the actual binaries
RUN cargo build --release
