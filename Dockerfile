FROM ekidd/rust-musl-builder:stable AS builder

RUN sudo chown -R rust:rust /home

RUN USER=root cargo new /home/vercel-ddns --bin
WORKDIR /home/vercel-ddns
## Caching dependencies
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml
RUN cargo build --bins --release --target x86_64-unknown-linux-musl
RUN rm src/*.rs

COPY src ./src
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/vercel*
RUN cargo build --bins --release --target x86_64-unknown-linux-musl

FROM alpine:latest
COPY --from=builder /home/vercel-ddns/target/x86_64-unknown-linux-musl/release/vercel-ddns /usr/local/bin/
CMD /usr/local/bin/vercel-ddns
