# Runtime stage
FROM debian:bookworm-slim

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=base-image /build/target/release/email-service .

LABEL org.opencontainers.image.source=https://github.com/BrewingBytes/BrewGet
CMD [ "./email-service" ]
