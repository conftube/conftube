ARG DIST_IMAGE=alpine:3.17
ARG NODE_IMAGE=node:19.7-alpine3.17
ARG RUST_IMAGE=rust:1.68-alpine

FROM $RUST_IMAGE AS backend
RUN cargo new --bin rust-docker-web
WORKDIR ./rust-docker-web
COPY backend/Cargo.lock ./Cargo.lock
COPY backend/Cargo.toml ./Cargo.toml
RUN apk add libc-dev libpq && rustup component add clippy && cargo build --release
RUN rm src/*.rs
ADD backend ./
RUN  cargo clippy --no-deps \
    && cargo check \
    && cargo build -r \
RUN pwd

FROM $NODE_IMAGE AS frontend
WORKDIR /app
COPY frontend/package.* .
RUN npm install
COPY frontend .
RUN npm run build

FROM $DIST_IMAGE
EXPOSE 8080
WORKDIR /app
COPY --from=backend ~/rust-docker-web/target/release/conftube .
COPY --from=frontend /app/build ./public
CMD ["./conftube"]
