FROM rust:1.50 as builder

WORKDIR /usr/src/restaurant-api

COPY . .

RUN apt-get install libsqlite3-dev pkg-config
RUN rustup override set nightly && rustup update && cargo update
RUN cargo install --path .



FROM debian:buster-slim

RUN apt-get update && apt-get install -y openssl libsqlite3-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /app/restaurant-api 

WORKDIR /app/restaurant-api

COPY --from=builder /usr/local/cargo/bin/restaurant-api .
COPY --from=builder /usr/src/restaurant-api/sql ./sql

ENV ROCKET_DATABASES="{sqlite_db = { url = "db.sqlite", pool_size = 20 }}"

RUN chmod u+x restaurant-api

EXPOSE 8000

CMD ["./restaurant-api"]

