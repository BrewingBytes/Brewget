FROM base-image

WORKDIR /app
COPY --from=base-image /build/target/release/auth-service .

LABEL org.opencontainers.image.source=https://github.com/BrewingBytes/BrewGet
CMD [ "./auth-service" ]
