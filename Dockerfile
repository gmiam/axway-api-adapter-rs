FROM rust:1.48.0-slim-buster AS base
ENV USER=admin

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch

COPY src /code/src

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline

FROM gcr.io/distroless/cc-debian10

COPY --from=builder /code/target/release/axway-api-adapter-rs /usr/bin/axway-api-adapter-rs

EXPOSE 8280

ENTRYPOINT [ "/usr/bin/axway-api-adapter-rs" ]