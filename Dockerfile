ARG DIST_IMAGE=debian:buster-slim
ARG NODE_IMAGE=node:19.7.0-buster-slim
ARG RUST_IMAGE=rust:1.69.0-slim-buster

FROM $RUST_IMAGE AS backend
RUN cargo new --bin rust-docker-web
WORKDIR ./rust-docker-web
COPY backend/Cargo.lock ./Cargo.lock
COPY backend/Cargo.toml ./Cargo.toml
RUN apt-get update && apt-get install -y libc-dev libpq-dev && rustup component add clippy
RUN cargo build --release && rm src/*.rs && rm target/release/deps/conftube-*
ADD backend ./
RUN cargo build --release --offline  \
    && cargo clippy --no-deps \
    && cargo check --release

FROM $NODE_IMAGE AS frontend
WORKDIR /app
COPY frontend/package.* .
RUN npm install
COPY frontend .
RUN npm run build

FROM $DIST_IMAGE
RUN apt-get update && apt-get install -y libpq5 ca-certificates
EXPOSE 8080
WORKDIR /app
COPY --from=backend /rust-docker-web/target/release/conftube .
COPY --from=frontend /app/build ./public
CMD ["./conftube"]
