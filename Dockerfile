ARG DIST_IMAGE=alpine:3.17
ARG NODE_IMAGE=node:19.7-alpine3.17
ARG RUST_IMAGE=rust:1.68-alpine

FROM $RUST_IMAGE AS backend
WORKDIR /app
COPY backend/ .
RUN apk add libc-dev && rustup component add clippy \
    && cargo clippy --no-deps \
    && cargo check \
    && cargo build -r

FROM $NODE_IMAGE AS frontend
WORKDIR /app
COPY frontend/package.* .
RUN npm install
COPY frontend .
RUN npm run build

FROM $DIST_IMAGE
EXPOSE 8080
WORKDIR /app
COPY --from=backend /app/target/release/conftube .
COPY --from=frontend /app/build ./public
CMD ["./conftube"]
