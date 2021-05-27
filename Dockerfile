FROM rust:latest

WORKDIR /usr/src/scoob

RUN mkdir src/
RUN echo "fn main() {panic!();}" > src/main.rs

COPY Cargo.toml .
RUN cargo fetch

COPY . .

RUN cargo install --path . --locked
