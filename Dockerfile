FROM alpine:latest

COPY ./ /project
WORKDIR /project

RUN apk add --no-cache llvm-libunwind \
    && apk add --no-cache --virtual .build rust cargo nodejs-npm \
    && cargo build --release \
    && mv target/release/ipfs-ink . \
    && rm -rf target/ ~/.cargo/ \
    && npm install \
    && node_modules/webpack/bin/webpack.js -p --config webpack.production.config.js \
    && rm -rf /usr/local/lib/node_modules/ node_modules/ ~/.npm \
    && apk del --purge .build

ENTRYPOINT ["./ipfs-ink"]
