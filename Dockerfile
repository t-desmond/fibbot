FROM rust:1.75

WORKDIR /app

RUN rustup install 1.81.0 && rustup default 1.81.0

COPY . .

RUN cargo build --release

ENTRYPOINT ["/app/target/release/fibbot"]
