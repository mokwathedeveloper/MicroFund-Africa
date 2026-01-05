# Frontend Dockerfile
FROM rust:1.75 as builder

# Install trunk and wasm target
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN cd frontend && trunk build --release

FROM nginx:alpine
COPY --from=builder /app/frontend/dist /usr/share/nginx/html
COPY --from=builder /app/frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
