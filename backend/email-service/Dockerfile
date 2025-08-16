FROM base-image

WORKDIR /app
COPY --from=base-image /build/target/release/email-service .

LABEL org.opencontainers.image.source=https://github.com/BrewingBytes/BrewGet
CMD [ "./email-service" ]
