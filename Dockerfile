# builder stage
FROM rust:stable-slim AS builder
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
# fetch deps to leverage layer cache
RUN mkdir src && echo 'fn main(){}' > src/main.rs && \
    cargo fetch
COPY . .
RUN cargo test --all -- --nocapture