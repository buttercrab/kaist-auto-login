FROM ekidd/rust-musl-builder:stable as build

ADD --chown=rust:rust ./server ./
RUN cargo build --release

FROM alpine:latest

ENV APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/server /app/server

RUN chown -R $APP_USER:$APP_USER /app

USER $APP_USER
WORKDIR /app
EXPOSE 80

CMD ["./server"]
