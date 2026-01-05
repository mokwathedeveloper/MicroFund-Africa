# Backend Dockerfile
FROM rust:1.75-slim-bookworm as builder

WORKDIR /app
COPY . .

# Install dependencies for SQLx and OpenSSL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cd backend && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libssl3 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/microfund-backend /app/microfund-backend
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/backend/.env.example /app/.env

EXPOSE 8080
CMD ["./microfund-backend"]

