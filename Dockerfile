FROM rust:1.78 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

ENV RUST_LOG info
ENV PORT 80

COPY --from=builder /app/target/release/nsfw_detector .

CMD ["./nsfw_detector"]
