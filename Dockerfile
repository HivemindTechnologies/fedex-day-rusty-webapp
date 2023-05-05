FROM rust:1.69.0 as builder
WORKDIR /usr/src/rust-workshop
COPY . .
RUN cargo build --release

# FROM debian:bullseye-slim
# # RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
# RUN apt update && apt install -y ca-certificates && rm -rf /var/lib/apt/lists/*
# COPY --from=builder /usr/src/rust-workshop/target/release/rust-workshop /usr/local/bin/rust-workshop
# CMD ["rust-workshop"]


FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /usr/src/rust-workshop/target/release/rust-workshop /usr/local/bin/rust-workshop
CMD ["rust-workshop"]
