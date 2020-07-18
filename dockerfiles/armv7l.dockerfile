###     Builder     ###
FROM arm32v7/rust as builder

WORKDIR /app
COPY Cargo.lock .
COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

###     Runtime     ###
FROM woahbase/alpine-glibc:armv7l
RUN apk add --no-cache libressl-dev 

COPY --from=builder /app/target/release/fast /
CMD ["/fast"]