# FROM --platform=amd64 clux/muslrust:stable AS builder
# WORKDIR /usr/src/minurl

# COPY Cargo.toml Cargo.lock ./
# RUN mkdir src && echo "fn main() {}" > src/main.rs
# RUN cargo fetch
# RUN cargo build --release 
# RUN rm src/main.rs

# ADD --chown=rust:rust . .
# RUN rustup update
# RUN sudo apt-get update 
# RUN cargo build --release --target x86_64-unknown-linux-musl
# RUN ls target/x86_64-unknown-linux-musl

FROM alpine AS runtime
RUN apk update && apk add ca-certificates openssl
WORKDIR /app
# COPY --from=builder /usr/src/minurl/target/x86_64-unknown-linux-musl/release/minurl /app/minurl
COPY ./target/aarch64-unknown-linux-musl/release/minurl ./minurl
EXPOSE 8080
CMD ["/app/minurl"]
