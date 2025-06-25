#!/bin/bash
set -e

# GCP Setup Script for FintradeX Parachain

echo "🚀 Setting up GCP environment for FintradeX Parachain..."

# Install Docker
if ! command -v docker &> /dev/null; then
    echo "📦 Installing Docker..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    sh get-docker.sh
    sudo usermod -aG docker $USER
    rm get-docker.sh
fi

# Install Google Cloud CLI
if ! command -v gcloud &> /dev/null; then
    echo "📦 Installing Google Cloud CLI..."
    curl https://sdk.cloud.google.com | bash
    exec -l $SHELL
fi

# Configure Docker for GCP
echo "🔧 Configuring Docker for GCP..."
gcloud auth configure-docker

# Create necessary directories
echo "📁 Creating directories..."
sudo mkdir -p /opt/fintradex/data
sudo mkdir -p /opt/fintradex/logs
sudo chown -R $USER:$USER /opt/fintradex

# Install monitoring tools
echo "📊 Installing monitoring tools..."
sudo apt-get update
sudo apt-get install -y prometheus prometheus-node-exporter grafana

# Configure firewall rules
echo "🔥 Configuring firewall..."
gcloud compute firewall-rules create fintradex-node \
    --allow tcp:30333,tcp:9933,tcp:9944,tcp:9615 \
    --source-ranges 0.0.0.0/0 \
    --description "FintradeX Parachain Node Ports"

echo "✅ GCP setup completed!"
echo "📝 Next steps:"
echo "1. Set up GitHub secrets (GCP_SA_KEY, GCP_SA_KEY_PROD)"
echo "2. Configure environment variables in GitHub"
echo "3. Push to develop branch to trigger first deployment" 