FROM rust:1.95.0-alpine AS build
WORKDIR /app

RUN rustup target add wasm32-unknown-unknown && \
    curl -sSL https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-gnu.tar.gz \
    | tar -xz -C /usr/local/bin/

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html ./
RUN trunk build --release

FROM nginx:1.29.8-alpine
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html
EXPOSE 80
