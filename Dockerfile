FROM rust:1.75

# WORKDIR /app

COPY . /app

RUN cargo build

CMD ["/app/target/debug/fibbot"]