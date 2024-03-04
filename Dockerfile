FROM --platform=linux/amd64 ubuntu:20.04

RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY ./target/x86_64-unknown-linux-gnu/release/Minurl /app/Minurl

CMD ["/app/Minurl"]
