FROM rust:alpine AS builder
WORKDIR /workspace
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo install --path .

FROM alpine
WORKDIR /auth-service
COPY --from=builder --chown=auth-user:auth-user /workspace/target/release/auth-service .

RUN adduser -D auth-service
USER auth-service

EXPOSE 8081
CMD [ "./auth-service" ]
