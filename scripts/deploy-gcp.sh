#!/bin/bash

# FintradeX Parachain GCP Deployment Script
# Instance: fintra-bc
# Zone: me-central1-c
# External IP: 34.18.76.207

set -e

echo "🚀 Starting FintradeX Parachain deployment on GCP..."

# Configuration
INSTANCE_NAME="fintra-bc"
ZONE="me-central1-c"
EXTERNAL_IP="34.18.76.207"
PROJECT_ID=$(gcloud config get-value project 2>/dev/null || echo "your-project-id")

echo "📍 Target Instance: $INSTANCE_NAME"
echo "📍 Zone: $ZONE"
echo "📍 External IP: $EXTERNAL_IP"
echo "📍 Project: $PROJECT_ID"

# Check if gcloud is installed
if ! command -v gcloud &> /dev/null; then
    echo "❌ gcloud CLI is not installed. Please install it first."
    exit 1
fi

# Install system dependencies and setup environment on the instance
# Based on official Polkadot SDK installation guide for Debian/Linux
echo "🔧 Setting up system dependencies on remote instance..."
gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command="
    # Update system
    sudo apt-get update
    
    # Install system dependencies as per official Polkadot SDK guide for Debian/Linux
    # https://docs.polkadot.com/develop/parachains/install-polkadot-sdk/
    sudo apt-get install -y \
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
        pkg-config
    
    # Install Rust as per official guide
    if ! command -v rustc &> /dev/null; then
        echo 'Installing Rust...'
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
    else
        echo 'Rust is already installed'
        source ~/.cargo/env
    fi
    
    # Configure Rust toolchain as per official guide
    rustup default stable
    rustup update
    rustup target add wasm32-unknown-unknown
    rustup component add rust-src
    
    # Install required tools as specified in README
    echo 'Installing chain-spec-builder...'
    cargo install --locked staging-chain-spec-builder@10.0.0
    
    echo 'Installing polkadot-omni-node...'
    cargo install --locked polkadot-omni-node@0.5.0
"

# Check if docker is installed on the instance
echo "🐳 Checking Docker installation on remote instance..."
gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command="which docker || echo 'Docker not found'"

# Install Docker on the instance if not present
echo "🐳 Installing Docker on the instance..."
gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command="
    if ! command -v docker &> /dev/null; then
        echo 'Installing Docker...'
        curl -fsSL https://get.docker.com -o get-docker.sh
        sudo sh get-docker.sh
        sudo usermod -aG docker \$USER
        sudo systemctl enable docker
        sudo systemctl start docker
    else
        echo 'Docker is already installed'
    fi
"

# Install Docker Compose on the instance if not present
echo "🐳 Installing Docker Compose on the instance..."
gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command="
    if ! command -v docker-compose &> /dev/null; then
        echo 'Installing Docker Compose...'
        sudo curl -L \"https://github.com/docker/compose/releases/latest/download/docker-compose-\$(uname -s)-\$(uname -m)\" -o /usr/local/bin/docker-compose
        sudo chmod +x /usr/local/bin/docker-compose
    else
        echo 'Docker Compose is already installed'
    fi
"

# Create firewall rules for the required ports
echo "🔥 Setting up firewall rules..."
gcloud compute firewall-rules create fintradex-parachain \
    --allow tcp:40333,tcp:50343,tcp:9944,tcp:9988,tcp:9615 \
    --source-ranges=0.0.0.0/0 \
    --description="FintradeX Parachain ports" \
    --quiet || echo "Firewall rule may already exist"

# Copy project files to the instance
echo "📁 Copying project files to the instance..."
gcloud compute scp --recurse . $INSTANCE_NAME:~/fintradex-parachain --zone=$ZONE

# SSH into the instance and deploy
echo "🚀 Deploying FintradeX Parachain..."
gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command="
    cd ~/fintradex-parachain
    
    # Source cargo environment
    source ~/.cargo/env
    
    echo '🔧 Building the project as specified in README...'
    cargo build --release --locked
    
    echo '📋 Generating chain specifications...'
    # Generate plain chain spec (you'll need to customize para-id and other parameters)
    chain-spec-builder --chain-spec-path ./fintradex_plain_chain_spec.json create \
      --relay-chain paseo \
      --para-id 4866 \
      --runtime target/release/wbuild/fintradex-runtime/fintradex_runtime.compact.compressed.wasm \
      named-preset local_testnet
    
    echo '📋 Converting to raw chain spec...'
    chain-spec-builder --chain-spec-path ./fintradex_raw_chain_spec.json convert-to-raw fintradex_plain_chain_spec.json
    
    echo '📋 Exporting genesis wasm...'
    polkadot-omni-node export-genesis-wasm --chain fintradex_raw_chain_spec.json para-wasm
    
    echo '📋 Exporting genesis state...'
    polkadot-omni-node export-genesis-head --chain fintradex_raw_chain_spec.json para-state
    
    echo '🔑 Generating node key...'
    polkadot-omni-node key generate-node-key --base-path data --chain fintradex_raw_chain_spec.json
    
    echo '🔧 Building Docker image...'
    docker build -t fintradex-collator .
    
    echo '📋 Creating docker-compose override for production...'
    cat > docker-compose.override.yml << 'EOF'
version: '3.8'
services:
  fintradex-collator:
    environment:
      - RUST_LOG=info
      - RUST_BACKTRACE=1
    restart: unless-stopped
    logging:
      driver: 'json-file'
      options:
        max-size: '10m'
        max-file: '3'
EOF
    
    echo '🚀 Starting the collator...'
    docker-compose up -d
    
    echo '⏳ Waiting for the node to start...'
    sleep 30
    
    echo '📊 Checking node status...'
    docker-compose logs --tail=50
"

echo "✅ Deployment completed!"
echo ""
echo "📊 Node Information:"
echo "   External IP: $EXTERNAL_IP"
echo "   Parachain RPC: http://$EXTERNAL_IP:9944"
echo "   Relay Chain RPC: http://$EXTERNAL_IP:9988"
echo "   Metrics: http://$EXTERNAL_IP:9615"
echo ""
echo "🔧 Useful Commands:"
echo "   View logs: gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command='cd ~/fintradex-parachain && docker-compose logs -f'"
echo "   Stop node: gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command='cd ~/fintradex-parachain && docker-compose down'"
echo "   Restart node: gcloud compute ssh $INSTANCE_NAME --zone=$ZONE --command='cd ~/fintradex-parachain && docker-compose restart'"
echo ""
echo "🔑 Next Steps:"
echo "   1. Edit the generated chain spec files with your specific parameters"
echo "   2. Register your parachain on Paseo using the exported wasm and state"
echo "   3. Insert your session key using the RPC endpoint"
echo "   4. Monitor the node logs for any issues"
echo "   5. Check sync status with the relay chain" 