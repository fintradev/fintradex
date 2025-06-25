# FintradeX Parachain CI/CD Setup

## Quick Setup

### 1. GCP Service Account Setup
```bash
# Create service account
gcloud iam service-accounts create fintradex-ci \
    --display-name="FintradeX CI/CD Service Account"

# Grant necessary permissions
gcloud projects add-iam-policy-binding fintradex-parachain \
    --member="serviceAccount:fintradex-ci@fintradex-parachain.iam.gserviceaccount.com" \
    --role="roles/compute.admin"

gcloud projects add-iam-policy-binding fintradex-parachain \
    --member="serviceAccount:fintradex-ci@fintradex-parachain.iam.gserviceaccount.com" \
    --role="roles/storage.admin"

# Create and download key
gcloud iam service-accounts keys create ~/fintradex-ci-key.json \
    --iam-account=fintradex-ci@fintradex-parachain.iam.gserviceaccount.com
```

### 2. GitHub Secrets
Add these secrets in your GitHub repository:

- `GCP_SA_KEY`: Base64 encoded service account key for development
- `GCP_SA_KEY_PROD`: Base64 encoded service account key for production
- `GCP_INSTANCE_IP`: Your GCP VM external IP (34.18.110.23)

### 3. VM Setup
Run on your GCP VM:
```bash
chmod +x scripts/setup-gcp.sh
./scripts/setup-gcp.sh
```

## Workflow Overview

### Automatic Deployments
- **Develop Branch**: Auto-deploys to development environment
- **Main Branch**: Auto-deploys to production environment

### Manual Deployments
Use GitHub Actions → Manual Deployment workflow for:
- Deploy specific versions
- Rollback to previous versions
- Restart services

### Monitoring
- Prometheus metrics on port 9615
- Grafana dashboard on port 3000
- Health checks after deployment

## Environment Variables
Update in `.github/workflows/ci-cd.yml`:
```yaml
GCP_PROJECT_ID: "your-project-id"
GCP_ZONE: "me-central1-c"
GCP_INSTANCE_NAME: "fintra-bc"
```

## Ports Used
- 30333: P2P networking
- 9933: HTTP RPC
- 9944: WebSocket RPC
- 9615: Prometheus metrics

## Troubleshooting

### Common Issues
1. **Permission Denied**: Check service account permissions
2. **Port Already in Use**: Stop existing containers first
3. **Build Failures**: Check Rust version (1.86 required)

### Logs
```bash
# Check service status
sudo systemctl status fintradex-node

# View logs
sudo journalctl -u fintradex-node -f

# Check container logs
docker logs fintradex-node
```

### Rollback
```bash
# Manual rollback
docker tag gcr.io/fintradex-parachain/fintradex-node:backup-YYYYMMDD-HHMMSS gcr.io/fintradex-parachain/fintradex-node:latest
sudo systemctl restart fintradex-node
``` 