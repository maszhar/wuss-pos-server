FROM rust:1.66 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:11-slim
WORKDIR /app

COPY --from=builder /app/target/release/email-sender .

CMD [ "/app/email-sender" ]