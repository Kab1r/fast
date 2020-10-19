###     Builder     ###
FROM rust as builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.lock .
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

###     Runtime     ###
FROM ubuntu
RUN apt-get update && \
    apt-get install \
    --no-install-recommends \
    --yes \
    libssl-dev ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/fast /
CMD ["/fast"]
