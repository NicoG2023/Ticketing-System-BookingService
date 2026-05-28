FROM rust:1.88 AS build

WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli --locked

COPY Cargo.toml Cargo.lock ./
COPY src src
COPY assets assets
COPY public public
COPY Dioxus.toml Dioxus.toml

RUN dx build --release --platform web


FROM nginx:alpine

COPY --from=build /app/target/dx/ticket-reservation-frontend/release/web/public /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
