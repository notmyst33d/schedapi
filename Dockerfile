FROM rust:alpine AS builder

WORKDIR /app
COPY . .

RUN apk update && apk add musl-dev
RUN --mount=type=cache,target=/app/target cargo build --release && cp target/release/schedapi .

FROM alpine:latest

WORKDIR /data
RUN mkdir /app
COPY --from=builder /app/schedapi /app

CMD ["/app/schedapi"]
