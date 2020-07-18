###     Builder     ###
FROM arm64v8/rust as builder

WORKDIR /app
COPY Cargo.lock .
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

###     Runtime     ###
FROM woahbase/alpine-glibc:aarch64
RUN apk add --no-cache libressl-dev 

COPY --from=builder /app/target/release/fast /
CMD ["/fast"]