FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo build

CMD ["./target/debug/fibbot"]