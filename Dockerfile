FROM ubuntu:16.10

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

ENV RUST_ARCHIVE=rust-1.14.0-x86_64-unknown-linux-gnu.tar.gz
ENV RUST_DOWNLOAD_URL=https://static.rust-lang.org/dist/$RUST_ARCHIVE

RUN mkdir /rust
WORKDIR /rust

RUN curl -fsOSL $RUST_DOWNLOAD_URL \
        && curl -s $RUST_DOWNLOAD_URL.sha256 | sha256sum -c - \
        && tar -C /rust -xzf $RUST_ARCHIVE --strip-components=1 \
        && rm $RUST_ARCHIVE \
        && ./install.sh

RUN mkdir /go-ipfs
WORKDIR /go-ipfs
ARG IPFS_VERSION=v0.4.6
RUN curl -fsOSL https://dist.ipfs.io/go-ipfs/${IPFS_VERSION}/go-ipfs_${IPFS_VERSION}_linux-amd64.tar.gz \
        && tar -C /go-ipfs -xzf go-ipfs_${IPFS_VERSION}_linux-amd64.tar.gz \
        && rm go-ipfs_${IPFS_VERSION}_linux-amd64.tar.gz \
        && cp go-ipfs/ipfs /usr/local/bin

RUN mkdir /project
WORKDIR /project

COPY ./ ./
RUN cargo build --release

RUN npm install \
        && npm install -g webpack \
        && ln -s /usr/bin/nodejs /usr/bin/node \
        && webpack -p --config webpack.production.config.js

ENTRYPOINT ["./target/release/ipfs-ink"]
