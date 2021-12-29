FROM rust:alpine AS builder

WORKDIR /phetch

COPY . .

RUN apk add openssl-dev

RUN cargo install --path .

FROM alpine

COPY --from=builder /usr/local/cargo/bin/phetch /usr/local/bin/

RUN chmod +x /usr/local/bin/phetch

ENTRYPOINT ["/usr/local/bin/phetch"]
