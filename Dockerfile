# Multi-stage build for FintradeX Parachain
FROM docker.io/paritytech/ci-unified:latest as builder

WORKDIR /fintradex
COPY . /fintradex

# Install system dependencies as per official Polkadot SDK guide for Debian/Linux
# https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/
RUN apt-get update && apt-get install -y \
    git \
    clang \
    curl \
    libssl-dev \
    llvm \
    libudev-dev \
    make \
    protobuf-compiler \
    build-essential \
    cmake \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust as per official guide
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Configure Rust toolchain as per official guide
RUN rustup default stable && \
    rustup update && \
    rustup target add wasm32-unknown-unknown && \
    rustup component add rust-src

# Install required tools as specified in README
# Install chain-spec-builder
RUN cargo install --locked staging-chain-spec-builder@10.0.0

# Install polkadot-omni-node
RUN cargo install --locked polkadot-omni-node@0.5.0

# Build the project using the correct command from README
RUN cargo fetch
RUN cargo build --release --locked

# Production stage
FROM docker.io/parity/base-bin:latest

# Copy the polkadot-omni-node binary
COPY --from=builder /root/.cargo/bin/polkadot-omni-node /usr/local/bin/

# Copy chain-spec-builder for runtime chain spec operations
COPY --from=builder /root/.cargo/bin/chain-spec-builder /usr/local/bin/

# Create polkadot user and setup directories
USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /fintradex polkadot && \
    mkdir -p /data /fintradex/.local/share && \
    chown -R polkadot:polkadot /data && \
    ln -s /data /fintradex/.local/share/polkadot && \
    # Create directories for chain data
    mkdir -p /data/chains && \
    chown -R polkadot:polkadot /data/chains && \
    # Remove unnecessary binaries to minimize attack surface
    rm -rf /usr/bin /usr/sbin && \
    # Verify the binary works
    /usr/local/bin/polkadot-omni-node --version

# Switch to polkadot user
USER polkadot

# Expose ports for P2P, RPC, and WebSocket
EXPOSE 30333 40333 9933 9944 9615 9988

# Volume for persistent data
VOLUME ["/data"]

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:9944/health || exit 1

# Default command for collator mode
ENTRYPOINT ["/usr/local/bin/polkadot-omni-node"]

# Default arguments for GCP deployment
CMD ["--collator", \
     "--chain", "fintradex_raw_chain_spec.json", \
     "--base-path", "/data", \
     "--port", "40333", \
     "--rpc-port", "9944", \
     "--ws-port", "9944", \
     "--rpc-cors", "all", \
     "--unsafe-rpc-external", \
     "--unsafe-ws-external", \
     "--rpc-methods", "Unsafe", \
     "--force-authoring", \
     "--name", "fintradex-collator", \
     "--", \
     "--chain", "paseo", \
     "--port", "50343", \
     "--rpc-port", "9988"]
