FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo self-update

RUN cargo build --release

CMD ["./target/release/fibbot"]
