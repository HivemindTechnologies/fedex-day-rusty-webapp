FROM rust:1.69.0 as builder
WORKDIR /usr/src/rusty-webapp
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/rusty-webapp/target/release/fedex-day-rusty-webapp /usr/local/bin/rusty-webapp
CMD ["rusty-webapp"]
