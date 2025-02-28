FROM rust:1.75

WORKDIR /app

# Install and set Rust version to 1.81.0
RUN rustup install 1.81.0 && rustup default 1.81.0

COPY . .

RUN cargo build --release

ENTRYPOINT ["/app/target/release/fibbot"]
