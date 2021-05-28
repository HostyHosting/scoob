FROM rust:latest

WORKDIR /usr/src/scoob

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cross
RUN apt-get -y update && apt-get -y install musl-tools

RUN mkdir src/
RUN echo "fn main() {panic!();}" > src/main.rs

COPY Cargo.toml .
RUN cross fetch --target x86_64-unknown-linux-musl

COPY . .

RUN cross install --path . --target x86_64-unknown-linux-musl
