# Fintra Setup Guide for macOS

This guide will walk you through the process of setting up and running Fintra on your local machine using Docker.

## Prerequisites

- Docker Desktop for Mac
- Git
- macOS Terminal
- Homebrew (macOS package manager)

## Step 1: Initial Setup on macOS

### 1.1 Install Homebrew (if not installed)
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 1.2 Install Required Tools
```bash
# Install Docker Desktop
brew install --cask docker

# Install Git
brew install git

# Install jq for JSON processing
brew install jq
```

### 1.3 Start Docker Desktop
- Open Docker Desktop application
- Wait for Docker to start (whale icon in menu bar should stop animating)
- Verify Docker is running:
  ```bash
  docker --version
  docker-compose --version
  ```

## Step 2: Project Setup

### 2.1 Clone the Repository
```bash
# Create a directory for the project
mkdir -p ~/projects
cd ~/projects

# Clone the repository
git clone https://github.com/wasif1024/fintra.git
cd fintra

# Create and switch to dev branch
git checkout -b dev
git push -u origin dev
```

### 2.2 Build the Docker Image
```bash
# Build the Docker image
docker-compose build

# Verify the image was created
docker images | grep fintra-node
```

## Step 3: Running Fintra

### 3.1 Start the Node
```bash
# Start the node in development mode
docker-compose up -d

# Check the container status
docker-compose ps

# View the logs
docker-compose logs -f
```

### 3.2 Verify the Node is Running
```bash
# Check if the node is responding
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params": []}' http://localhost:9933
```

### 3.3 Connect to Polkadot.js Apps

1. **Open Polkadot.js Apps**:
   - Go to https://polkadot.js.org/apps/
   - Click on the network selector (top left)
   - Click "Add custom endpoint"

2. **Add Your Node**:
   - For local development:
     ```
     Name: Fintra Local
     URL: ws://localhost:9944
     ```
   - For production:
     ```
     Name: Fintra Production
     URL: ws://your-server-ip:9944
     ```

3. **Verify Connection**:
   - The node should appear in the network list
   - Click on it to connect
   - You should see the chain state and be able to interact with your node

## Step 4: Complete Deployment Pipeline

### 4.1 GitHub Repository Setup

1. **Clone the Repository**:
   ```bash
   # Create a directory for the project
   mkdir -p ~/projects
   cd ~/projects

   # Clone the repository
   git clone https://github.com/wasif1024/fintra.git
   cd fintra

   # Create and switch to dev branch
   git checkout -b dev
   git push -u origin dev
   ```

2. **Set Up GitHub Secrets**:
   - Go to https://github.com/wasif1024/fintra/settings/secrets/actions
   - Add the following secrets:
     ```
     DOCKERHUB_USERNAME: Your DockerHub username
     DOCKERHUB_TOKEN: Your DockerHub access token
     AWS_ACCESS_KEY_ID: Your AWS access key
     AWS_SECRET_ACCESS_KEY: Your AWS secret key
     EC2_SSH_KEY: Your EC2 .pem file content
     EC2_HOST: Your EC2 instance public IP/DNS
     ```

### 4.2 Development Workflow

1. **Create Feature Branch**:
   ```bash
   # Create a new feature branch from dev
   git checkout dev
   git pull origin dev
   git checkout -b feature/your-feature-name
   
   # Make your changes
   git add .
   git commit -m "Add your feature"
   
   # Push to GitHub
   git push origin feature/your-feature-name
   ```

2. **Create Pull Request**:
   - Go to https://github.com/wasif1024/fintra/pulls
   - Click "New Pull Request"
   - Select:
     - Base: `dev`
     - Compare: `feature/your-feature-name`
   - Create pull request

3. **CI/CD Pipeline**:
   - The pipeline will run automatically on:
     - Pull requests to `dev` branch
     - Pushes to `dev` branch
   - Deployment to EC2 happens automatically when:
     - Code is merged to `dev` branch
     - All tests pass
     - Security checks pass

4. **Monitor Pipeline**:
   - Go to https://github.com/wasif1024/fintra/actions
   - Watch the pipeline progress:
     1. Build and Test
     2. Security Audit
     3. Docker Build
     4. Deploy to EC2

### 4.3 AWS EC2 Setup

1. **Launch EC2 Instance**:
   - Go to AWS Console → EC2
   - Click "Launch Instance"
   - Choose Ubuntu Server 22.04 LTS
   - Select t2.micro (or larger)
   - Configure Security Group:
     ```
     SSH (22): Your IP
     Custom TCP (30333): 0.0.0.0/0
     Custom TCP (9933): 0.0.0.0/0
     Custom TCP (9944): 0.0.0.0/0
     Custom TCP (9615): 0.0.0.0/0
     ```
   - Create new key pair and download .pem file

2. **Prepare .pem File**:
   ```bash
   # Move .pem file to secure location
   mkdir -p ~/.ssh/aws
   mv /path/to/your-key.pem ~/.ssh/aws/
   
   # Set correct permissions
   chmod 400 ~/.ssh/aws/your-key.pem
   
   # Convert for GitHub Actions
   cat ~/.ssh/aws/your-key.pem | tr '\n' '~' | sed 's/~/\\n/g' > ~/.ssh/aws/key-oneline.txt
   ```

3. **Configure EC2 Instance**:
   ```bash
   # Connect to EC2
   ssh -i ~/.ssh/aws/your-key.pem ubuntu@your-ec2-ip
   
   # Update system
   sudo apt-get update
   sudo apt-get upgrade -y
   
   # Install Docker
   sudo apt-get install -y docker.io docker-compose
   
   # Add user to docker group
   sudo usermod -aG docker ubuntu
   
   # Create deployment directory
   mkdir -p /home/ubuntu/fintra
   
   # Exit SSH
   exit
   ```

### 4.4 Get RPC URL

1. **Local Development**:
   ```
   WebSocket: ws://localhost:9944
   HTTP: http://localhost:9933
   ```

2. **Production**:
   ```
   WebSocket: ws://your-ec2-ip:9944
   HTTP: http://your-ec2-ip:9933
   ```

3. **With Domain (Recommended)**:
   ```
   WebSocket: wss://your-domain.com
   HTTP: https://your-domain.com
   ```

### 4.5 Connect to Polkadot.js Apps

1. **Add Your Node**:
   - Go to https://polkadot.js.org/apps/
   - Click network selector
   - Click "Add custom endpoint"
   - Add your RPC URL:
     ```
     Name: Fintra Production
     URL: wss://your-domain.com (or ws://your-ec2-ip:9944)
     ```

2. **Verify Connection**:
   - The node should appear in the network list
   - Click on it to connect
   - You should see the chain state

### 4.6 Monitor Your Node

1. **Check Node Health**:
   ```bash
   # Using curl
   curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health", "params": []}' http://your-ec2-ip:9933
   
   # Using Polkadot.js Apps
   - Go to Network → Telemetry
   - Look for your node
   ```

2. **View Metrics**:
   - Access Prometheus metrics at `http://your-ec2-ip:9615/metrics`
   - Set up Grafana dashboard for visualization

3. **Check Logs**:
   ```bash
   # SSH into EC2
   ssh -i ~/.ssh/aws/your-key.pem ubuntu@your-ec2-ip
   
   # View logs
   sudo journalctl -u fintra -f
   ```

## Step 5: Monitoring and Management

### 5.1 View Logs
```bash
# View real-time logs
docker-compose logs -f

# View last 100 lines
docker-compose logs --tail=100

# View logs with timestamps
docker-compose logs -f --timestamps
```

### 5.2 Check Container Status
```bash
# View container status
docker-compose ps

# View container details
docker inspect fintra-node

# View container resource usage
docker stats fintra-node
```

### 5.3 Access the Node
- WebSocket: ws://localhost:9944
- RPC: http://localhost:9933
- P2P: localhost:30333
- Metrics: http://localhost:9615/metrics

## Step 6: Development Workflow

### 6.1 Rebuild After Changes
```bash
# Stop the containers
docker-compose down

# Rebuild the image
docker-compose build

# Start the containers
docker-compose up -d
```

### 6.2 Access the Container
```bash
# Get a shell in the container
docker-compose exec fintra-node /bin/sh

# Run commands in the container
docker-compose exec fintra-node fintra-node --version
```

## Step 7: Data Management

### 7.1 Backup Data
```bash
# Create backup directory
mkdir -p ~/backups/fintra

# Backup the data volume
docker run --rm -v fintra-data:/data -v ~/backups/fintra:/backup alpine tar czf /backup/fintra-data-$(date +%Y%m%d).tar.gz /data
```

### 7.2 Restore Data
```bash
# Restore from backup
docker run --rm -v fintra-data:/data -v ~/backups/fintra:/backup alpine sh -c "rm -rf /data/* && tar xzf /backup/your-backup-file.tar.gz -C /"
```

## Step 8: EC2/Linux Setup Guide

### 8.1 Install Required Tools on Linux

#### For Ubuntu/Debian:
```bash
# Update package list
sudo apt-get update
sudo apt-get upgrade -y

# Install Docker
sudo apt-get install -y docker.io docker-compose

# Install Git
sudo apt-get install -y git

# Install jq for JSON processing
sudo apt-get install -y jq
```

#### For CentOS/RHEL/Fedora:
```bash
# Update package list
sudo dnf update -y

# Install Docker
sudo dnf install -y docker docker-compose

# Install Git
sudo dnf install -y git

# Install jq for JSON processing
sudo dnf install -y jq
```

#### For Amazon Linux 2:
```bash
# Update package list
sudo yum update -y

# Install Docker
sudo amazon-linux-extras install docker -y
sudo yum install -y docker-compose

# Install Git
sudo yum install -y git

# Install jq for JSON processing
sudo yum install -y jq
```

#### For Arch Linux:
```bash
# Update package list
sudo pacman -Syu

# Install Docker
sudo pacman -S docker docker-compose

# Install Git
sudo pacman -S git

# Install jq for JSON processing
sudo pacman -S jq
```

# Add your user to the docker group (for all distributions)
sudo usermod -aG docker $USER

# Apply the new group membership
newgrp docker

# Start and enable Docker service (for all distributions)
sudo systemctl start docker
sudo systemctl enable docker
```

### 8.2 Verify Docker Installation
```bash
# Check Docker version
docker --version
docker-compose --version

# Verify Docker is running
sudo systemctl status docker
```

### 8.3 Project Setup on EC2
```bash
# Create project directory
mkdir -p ~/projects
cd ~/projects

# Clone the repository
git clone https://github.com/wasif1024/fintra.git
cd fintra

# Create and switch to dev branch
git checkout -b dev
git push -u origin dev
```

### 8.4 Build and Run on EC2
```bash
# Build the Docker image
docker-compose build

# Start the node in development mode
docker-compose up -d

# Check the container status
docker-compose ps

# View the logs
docker-compose logs -f
```

### 8.5 EC2 Security Group Configuration
Make sure to configure your EC2 security group to allow the following ports:
- SSH (22): Your IP
- Custom TCP (30333): 0.0.0.0/0 (P2P)
- Custom TCP (9933): 0.0.0.0/0 (HTTP RPC)
- Custom TCP (9944): 0.0.0.0/0 (WebSocket)
- Custom TCP (9615): 0.0.0.0/0 (Metrics)

### 8.6 EC2 System Maintenance
```bash
# Check system resources
df -h  # Check disk space
free -h  # Check memory usage
top  # Check CPU and process status

# Check Docker resources
docker system df  # Check Docker disk usage
docker stats  # Monitor container resource usage

# Clean up unused Docker resources
docker system prune -a  # Remove unused containers, networks, images
```

### 8.7 EC2 Backup and Restore
```bash
# Create backup directory
mkdir -p ~/backups/fintra

# Backup the data volume
docker run --rm -v fintra-data:/data -v ~/backups/fintra:/backup alpine tar czf /backup/fintra-data-$(date +%Y%m%d).tar.gz /data

# Restore from backup
docker run --rm -v fintra-data:/data -v ~/backups/fintra:/backup alpine sh -c "rm -rf /data/* && tar xzf /backup/your-backup-file.tar.gz -C /"
```

### 8.8 EC2 Monitoring
```bash
# Install monitoring tools
sudo apt-get install -y htop iotop

# Monitor system resources
htop  # Interactive process viewer
iotop  # Monitor disk I/O

# Check Docker logs
docker-compose logs -f --tail=100

# Monitor container health
docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
```

## Troubleshooting

### Common Issues

1. **Container Won't Start**
   ```bash
   # Check container logs
   docker-compose logs
   
   # Check container status
   docker-compose ps
   
   # Check system resources
   docker system df
   ```

2. **Port Conflicts**
   ```bash
   # Check if ports are in use
   lsof -i :30333
   lsof -i :9933
   lsof -i :9944
   lsof -i :9615
   ```

3. **Permission Issues**
   ```bash
   # Fix volume permissions
   docker-compose down
   docker volume rm fintra-data
   docker-compose up -d
   ```

## Maintenance

### Regular Tasks

1. **Update Docker Images**
   ```bash
   # Pull latest base images
   docker-compose pull
   
   # Rebuild with latest images
   docker-compose build --no-cache
   ```

2. **Clean Up Docker**
   ```bash
   # Remove unused containers
   docker container prune
   
   # Remove unused images
   docker image prune
   
   # Remove unused volumes
   docker volume prune
   ```

3. **Monitor Resources**
   ```bash
   # View resource usage
   docker stats
   
   # Check disk usage
   docker system df
   ```

## Security Best Practices

1. Keep Docker Desktop updated
2. Regularly update base images
3. Use non-root user in container
4. Implement proper volume permissions
5. Monitor container logs for suspicious activity
6. Use Docker's built-in security features

## Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Substrate Documentation](https://docs.substrate.io/)
- [Polkadot Documentation](https://wiki.polkadot.network/) 