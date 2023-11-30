FROM rust:1.74.0
WORKDIR /usr/src/app
COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos
RUN cargo leptos build --release

RUN cp target/release/aoc /usr/src/app
RUN cp -r target/site /usr/src/app

ENV LEPTOS_OUTPUT_NAME="aoc"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"

WORKDIR /usr/src/app

CMD ["./aoc"]
