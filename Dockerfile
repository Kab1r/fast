###     Builder     ###
FROM rust as builder

WORKDIR /app
COPY Cargo.lock .
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release


###     Runtime     ###
FROM ubuntu:latest
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y \
      ca-certificates libssl-dev \
    && apt-get clean

COPY --from=builder /app/target/release/fast /
CMD ["/fast"]