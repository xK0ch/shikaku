FROM rust:1.99.1-alpine AS build
WORKDIR /app

RUN rustup target add wasm32-unknown-unknown && \
    wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.22.1/trunk-x86_64-unknown-linux-musl.tar.gz \
    | tar -xz -C /usr/local/bin/

COPY Cargo.toml Cargo.lock ./
COPY .cargo ./.cargo
COPY src ./src
RUN cargo test --release --locked
COPY index.html favicon.svg ./
RUN trunk build --release --locked

FROM nginx:1.31.8-alpine
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
