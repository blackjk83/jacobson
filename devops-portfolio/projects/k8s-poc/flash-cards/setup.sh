#!/bin/bash
set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Function to print status
print_status() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ $1${NC}"
    else
        echo -e "${RED}✗ $1${NC}"
        exit 1
    fi
}

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}Error: Docker is not running${NC}"
    exit 1
fi

echo "🚀 Building Flash Cards application..."

# Build the Docker image
echo "Building Docker image..."
docker build -t flash-cards:latest . || {
    echo -e "${RED}Error: Docker build failed${NC}"
    exit 1
}
print_status "Docker image built"

# Optional: Run security scan
if command -v trivy &> /dev/null; then
    echo "Running security scan..."
    trivy image --severity HIGH,CRITICAL flash-cards:latest
    print_status "Security scan completed"
else
    echo -e "${YELLOW}Warning: Trivy not installed, skipping security scan${NC}"
fi

echo -e "\n${GREEN}Build completed successfully!${NC}"
echo "You can now:"
echo "1. Run locally: docker run -d -p 8080:8080 flash-cards:latest"
echo "2. Push to registry: docker push <registry>/flash-cards:latest"
echo "3. Deploy to Kubernetes: kubectl apply -f k8s/deployment.yaml"

