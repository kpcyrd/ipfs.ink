FROM ubuntu:17.04

RUN DEBIAN_FRONTEND=noninteractive && \
    apt-get update && \
    apt-get install \
       -qqy \
       --no-install-recommends \
       ca-certificates \
       curl \
       gcc \
       libc6-dev \
       libssl-dev \
       nodejs \
       npm \
    && rm -rf /var/lib/apt/lists/*

ENV RUST_ARCHIVE=rust-1.17.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir /project
WORKDIR /project
COPY ./ ./

RUN mkdir /rust \
        && cd /rust \
        && curl -fsOSL $RUST_DOWNLOAD_URL \
        && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
        && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
        && rm $RUST_ARCHIVE \
        && ./install.sh \
        && cd /project \
        && cargo build --release \
        && mv target/release/ipfs-ink . \
        && /usr/local/lib/rustlib/uninstall.sh \
        && rm -rf /rust ~/.cargo target/

RUN npm install \
        && npm install -g webpack \
        && ln -s /usr/bin/nodejs /usr/bin/node \
        && webpack -p --config webpack.production.config.js \
        && rm -rf /usr/local/lib/node_modules/ node_modules/ ~/.npm

ENTRYPOINT ["./ipfs-ink"]
