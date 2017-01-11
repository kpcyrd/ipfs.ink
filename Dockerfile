FROM debian:jessie

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
RUN curl -fsOSL https://dist.ipfs.io/go-ipfs/v0.4.4/go-ipfs_v0.4.4_linux-amd64.tar.gz \
        && tar -C /go-ipfs -xzf go-ipfs_v0.4.4_linux-amd64.tar.gz \
        && rm go-ipfs_v0.4.4_linux-amd64.tar.gz \
        && cp go-ipfs/ipfs /usr/local/bin

RUN mkdir /project
WORKDIR /project

COPY ./ ./
RUN cargo build --release
ENTRYPOINT ["./target/release/ipfs-ink"]
