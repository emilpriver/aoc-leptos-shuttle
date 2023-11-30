FROM rust:1.74.0 as builder
WORKDIR /usr/src/app
COPY . .

RUN rustup target add wasm32-unknown-unknown

RUN cargo install --locked cargo-leptos
RUN cargo leptos build --release

RUN cp target/release/aoc /usr/src/app
RUN cp -r target/site /usr/src/site

ENV LEPTOS_OUTPUT_NAME="aoc"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"

FROM rust:1.74.0
COPY --from=builder /usr/src/app/aoc /usr/src/aoc/aoc
COPY --from=builder /usr/src/site /usr/src/aoc/target/site

WORKDIR /usr/src/aoc

CMD ["./aoc"]
