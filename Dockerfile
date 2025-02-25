FROM rust:1.75

WORKDIR /app

COPY . .

RUN echo "$1 and $2" >> $GITHUB_OUTPUT

RUN cargo build --release

ENTRYPOINT ["/app/target/release/fibbot"]
