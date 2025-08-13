FROM base-image

WORKDIR /app
COPY --from=base-image /build/target/release/auth-service .

CMD [ "./auth-service" ]
