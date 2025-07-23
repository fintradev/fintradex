# FintradeX Collator Helm Chart

This Helm chart deploys the FintradeX parachain collator on Kubernetes.

## Prerequisites

- Kubernetes cluster (1.19+)
- Helm 3.0+
- kubectl configured
- Docker (for building images)

## Quick Start

### 1. Build and Deploy

```bash
# Build the Docker image
docker build -t fintradex-collator:latest .

# Deploy using the provided script
./scripts/deploy-helm.sh --build
```

### 2. Deploy with Custom Values

```bash
# Create a custom values file
cat > my-values.yaml << EOF
collator:
  chainSpec:
    paraId: 4866
  sessionKey:
    enabled: true
    secretName: fintradex-session-key
  nodeKey:
    enabled: true
    secretName: fintradex-node-key

persistence:
  size: 200Gi

resources:
  limits:
    cpu: 8000m
    memory: 16Gi
  requests:
    cpu: 4000m
    memory: 8Gi
EOF

# Deploy with custom values
helm upgrade --install fintradex-collator ./helm/fintradex-collator \
  --namespace fintradex \
  --values my-values.yaml
```

## Configuration

### Image Configuration

```yaml
image:
  repository: fintradex-collator
  tag: latest
  pullPolicy: IfNotPresent
  buildFromSource: true
```

### Collator Configuration

```yaml
collator:
  chainSpec:
    name: fintradex
    paraId: 4866
    relayChain: paseo
  
  node:
    name: fintradex-collator
    basePath: /data
    port: 40333
    rpcPort: 9944
    wsPort: 9944
    rpcCors: all
    unsafeRpcExternal: true
    unsafeWsExternal: true
    rpcMethods: Unsafe
    forceAuthoring: true
  
  relay:
    chain: paseo
    port: 50343
    rpcPort: 9988
```

### Persistence

```yaml
persistence:
  enabled: true
  storageClass: ""
  accessMode: ReadWriteOnce
  size: 100Gi
  mountPath: /data
```

### Resources

```yaml
resources:
  limits:
    cpu: 4000m
    memory: 8Gi
  requests:
    cpu: 2000m
    memory: 4Gi
```

### Service Configuration

```yaml
service:
  type: LoadBalancer
  annotations:
    service.beta.kubernetes.io/aws-load-balancer-type: nlb
    service.beta.kubernetes.io/aws-load-balancer-scheme: internet-facing
  ports:
    parachainP2P: 40333
    relayP2P: 50343
    parachainRPC: 9944
    relayRPC: 9988
    metrics: 9615
```

## Security

### Session Keys

To enable session key management:

```bash
# Create session key secret
kubectl create secret generic fintradex-session-key \
  --from-literal=session-key="your-session-key-here" \
  --namespace=fintradex

# Enable in values
collator:
  sessionKey:
    enabled: true
    secretName: fintradex-session-key
```

### Node Keys

To enable node key management:

```bash
# Create node key secret
kubectl create secret generic fintradex-node-key \
  --from-literal=node-key="your-node-key-here" \
  --namespace=fintradex

# Enable in values
collator:
  nodeKey:
    enabled: true
    secretName: fintradex-node-key
```

## Monitoring

### Prometheus Integration

```yaml
monitoring:
  enabled: true
  serviceMonitor:
    enabled: true
    interval: 30s
    scrapeTimeout: 10s
  prometheusRule:
    enabled: true
```

### Health Checks

```yaml
livenessProbe:
  enabled: true
  initialDelaySeconds: 60
  periodSeconds: 30
  timeoutSeconds: 10
  failureThreshold: 3

readinessProbe:
  enabled: true
  initialDelaySeconds: 30
  periodSeconds: 10
  timeoutSeconds: 5
  failureThreshold: 3
```

## Scaling

### Horizontal Pod Autoscaler

```yaml
hpa:
  enabled: true
  minReplicas: 1
  maxReplicas: 3
  targetCPUUtilizationPercentage: 80
  targetMemoryUtilizationPercentage: 80
```

### Vertical Pod Autoscaler

```yaml
vpa:
  enabled: true
  updatePolicy:
    updateMode: Auto
```

## Network Policies

```yaml
networkPolicy:
  enabled: true
  ingress:
    - from:
        - namespaceSelector: {}
      ports:
        - protocol: TCP
          port: 40333
        - protocol: TCP
          port: 50343
        - protocol: TCP
          port: 9944
        - protocol: TCP
          port: 9988
        - protocol: TCP
          port: 9615
```

## Troubleshooting

### Check Pod Status

```bash
kubectl get pods -n fintradex
kubectl describe pod -l app.kubernetes.io/name=fintradex-collator -n fintradex
```

### View Logs

```bash
kubectl logs -f deployment/fintradex-collator -n fintradex
```

### Check Services

```bash
kubectl get svc -n fintradex
kubectl describe svc fintradex-collator -n fintradex
```

### Access RPC

```bash
# Get external IP
EXTERNAL_IP=$(kubectl get svc fintradex-collator -n fintradex -o jsonpath='{.status.loadBalancer.ingress[0].ip}')

# Test RPC endpoint
curl -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"system_health","params":[],"id":1}' \
  http://$EXTERNAL_IP:9944
```

### Insert Session Key

```bash
# Get external IP
EXTERNAL_IP=$(kubectl get svc fintradex-collator -n fintradex -o jsonpath='{.status.loadBalancer.ingress[0].ip}')

# Insert session key
curl -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"author_insertKey","params":["aura","your-secret-phrase","your-public-key"],"id":1}' \
  http://$EXTERNAL_IP:9944
```

## Uninstall

```bash
# Uninstall the release
helm uninstall fintradex-collator -n fintradex

# Delete namespace (optional)
kubectl delete namespace fintradex
```

## Values Reference

| Parameter | Description | Default |
|-----------|-------------|---------|
| `image.repository` | Docker image repository | `fintradex-collator` |
| `image.tag` | Docker image tag | `latest` |
| `replicaCount` | Number of replicas | `1` |
| `service.type` | Service type | `LoadBalancer` |
| `persistence.enabled` | Enable persistence | `true` |
| `persistence.size` | Storage size | `100Gi` |
| `resources.limits.cpu` | CPU limit | `4000m` |
| `resources.limits.memory` | Memory limit | `8Gi` |
| `collator.chainSpec.paraId` | Parachain ID | `4866` |
| `collator.chainSpec.relayChain` | Relay chain | `paseo` |
| `monitoring.enabled` | Enable monitoring | `true` |
| `livenessProbe.enabled` | Enable liveness probe | `true` |
| `readinessProbe.enabled` | Enable readiness probe | `true` |

## Support

For issues and questions:

- GitHub Issues: [FintradeX Repository](https://github.com/fintradev/fintradex)
- Documentation: [FintradeX Docs](https://docs.fintradex.io)
- Community: [FintradeX Discord](https://discord.gg/fintradex) 