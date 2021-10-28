FROM docker.io/paritytech/ci-linux:production as builder
LABEL description="Build stage"

WORKDIR /walchain
COPY . /walchain

RUN apt-get update && apt-get -y install ca-certificates

RUN cargo build --release

# ===== SECOND STAGE ======

FROM docker.io/library/ubuntu:20.04
LABEL description="Walchain node"

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /walchain/target/release/walchain /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /walchain walchain && \
    mkdir -p /walchain/.local/share && \
    mkdir /data && \
    chown -R walchain:walchain /data && \
    ln -s /data /walchain/.local/share/walchain && \
    rm -rf /usr/bin /usr/sbin

USER walchain
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

ENTRYPOINT ["/usr/local/bin/walchain"]
