FROM rust:buster AS builder

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

RUN cargo build --release --offline

FROM debian:buster-slim

EXPOSE 8000

COPY --from=builder /code/target/release/store /store

CMD [ "/store" ]
