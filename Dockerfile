FROM joystream/rust-builder AS builder
LABEL description="compiles and caches dependencies, artifacts and node"
WORKDIR /walchain
COPY . /walchain

RUN cargo build --release

FROM debian:stretch
LABEL description="Walchain node"
WORKDIR /walchain
COPY --from=builder /walchain/target/release/walchain /walchain/walchain-node

EXPOSE 30333 9933 9944

# Use these volumes to persits chain state and keystore, eg.:
# --base-path /data
# optionally separate keystore (otherwise it will be stored in the base path)
# --keystore-path /keystore
# if base-path isn't specified, chain state is stored inside container in ~/.local/share/joystream-node/
# which is not ideal
VOLUME ["/data", "/keystore"]

ENTRYPOINT ["/walchain/walchain-node"]
