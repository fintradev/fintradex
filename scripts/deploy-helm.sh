#!/bin/bash

# FintradeX Parachain Helm Chart Deployment Script
# This script deploys the FintradeX collator using Helm

set -e

echo "🚀 Starting FintradeX Parachain Helm deployment..."

# Configuration
RELEASE_NAME="fintradex-collator"
NAMESPACE="fintradex"
CHART_PATH="./helm/fintradex-collator"

# Check if kubectl is installed
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl is not installed. Please install it first."
    exit 1
fi

# Check if helm is installed
if ! command -v helm &> /dev/null; then
    echo "❌ Helm is not installed. Please install it first."
    exit 1
fi

# Create namespace if it doesn't exist
echo "📁 Creating namespace..."
kubectl create namespace $NAMESPACE --dry-run=client -o yaml | kubectl apply -f -

# Build and push Docker image (if needed)
if [ "$1" = "--build" ]; then
    echo "🔧 Building Docker image..."
    docker build -t fintradex-collator:latest .
    
    echo "📤 Pushing Docker image..."
    # Uncomment and modify the following lines if you have a registry
    # docker tag fintradex-collator:latest your-registry/fintradex-collator:latest
    # docker push your-registry/fintradex-collator:latest
fi

# Create secrets for session key and node key (if provided)
if [ -n "$SESSION_KEY" ]; then
    echo "🔑 Creating session key secret..."
    kubectl create secret generic fintradex-session-key \
        --from-literal=session-key="$SESSION_KEY" \
        --namespace=$NAMESPACE \
        --dry-run=client -o yaml | kubectl apply -f -
fi

if [ -n "$NODE_KEY" ]; then
    echo "🔑 Creating node key secret..."
    kubectl create secret generic fintradex-node-key \
        --from-literal=node-key="$NODE_KEY" \
        --namespace=$NAMESPACE \
        --dry-run=client -o yaml | kubectl apply -f -
fi

# Create ConfigMap for chain specification
if [ -f "fintradex_raw_chain_spec.json" ]; then
    echo "📋 Creating chain specification ConfigMap..."
    kubectl create configmap fintradex-chain-spec \
        --from-file=fintradex_raw_chain_spec.json \
        --namespace=$NAMESPACE \
        --dry-run=client -o yaml | kubectl apply -f -
else
    echo "⚠️  Warning: fintradex_raw_chain_spec.json not found. Please create it first."
fi

# Deploy using Helm
echo "🚀 Deploying with Helm..."
helm upgrade --install $RELEASE_NAME $CHART_PATH \
    --namespace=$NAMESPACE \
    --set image.repository=fintradex-collator \
    --set image.tag=latest \
    --set collator.sessionKey.enabled=${SESSION_KEY:+true} \
    --set collator.nodeKey.enabled=${NODE_KEY:+true} \
    --set configMap.enabled=true \
    --set persistence.enabled=true \
    --set persistence.size=100Gi \
    --set service.type=LoadBalancer \
    --set resources.limits.cpu=4000m \
    --set resources.limits.memory=8Gi \
    --set resources.requests.cpu=2000m \
    --set resources.requests.memory=4Gi \
    --set monitoring.enabled=true \
    --set livenessProbe.enabled=true \
    --set readinessProbe.enabled=true \
    --wait --timeout=10m

echo "✅ Helm deployment completed!"
echo ""
echo "📊 Deployment Information:"
echo "   Namespace: $NAMESPACE"
echo "   Release: $RELEASE_NAME"
echo "   Chart: $CHART_PATH"
echo ""
echo "🔧 Useful Commands:"
echo "   View pods: kubectl get pods -n $NAMESPACE"
echo "   View logs: kubectl logs -f deployment/$RELEASE_NAME -n $NAMESPACE"
echo "   Get service: kubectl get svc -n $NAMESPACE"
echo "   Delete release: helm uninstall $RELEASE_NAME -n $NAMESPACE"
echo ""
echo "🔑 Next Steps:"
echo "   1. Wait for the pod to be ready: kubectl wait --for=condition=ready pod -l app.kubernetes.io/name=fintradex-collator -n $NAMESPACE"
echo "   2. Get the external IP: kubectl get svc $RELEASE_NAME -n $NAMESPACE"
echo "   3. Insert your session key using the RPC endpoint"
echo "   4. Monitor the node logs for any issues" 