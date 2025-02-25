FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo run

# CMD ["./target/release/fibbot"]