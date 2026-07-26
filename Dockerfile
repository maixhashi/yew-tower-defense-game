FROM rust:1.85-bookworm

RUN rustup target add wasm32-unknown-unknown \
  && cargo install trunk --locked

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY index.html trunk.toml ./
COPY src ./src

EXPOSE 8080

CMD ["trunk", "serve", "--address", "0.0.0.0", "--port", "8080"]
