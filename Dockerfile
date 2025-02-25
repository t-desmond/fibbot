FROM rust:alpine AS build-env

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN apk add --no-cache musl-dev upx

RUN cargo build --release

RUN upx --best target/release/fibbot

FROM scratch

COPY --from=build-env /app/target/release/fibbot .

ENTRYPOINT ["./fibbot"]
