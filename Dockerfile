FROM rust:1.69.0 as builder
WORKDIR /usr/src/rust-workshop
COPY . .
RUN apt-get update && apt-get install -y cmake
RUN cargo build --release

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/rust-workshop/target/release/rust-workshop /usr/local/bin/rust-workshop
CMD ["rust-workshop"]
