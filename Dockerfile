###     Builder     ###
FROM --platform=$BUILDPLATFORM rust AS builder

WORKDIR /app

COPY Cargo.lock .
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

###     Runtime     ###
FROM debian
RUN apt-get update && \
    apt-get install --yes --no-install-recommends \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/fast /
CMD ["/fast"]
