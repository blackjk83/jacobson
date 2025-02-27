#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Variables
IMAGE_NAME="yakovav/flash-cards-ops"
ENV=${1:-"dev"}
TAG=${2:-"latest"}

# Function to print status
print_status() {
    echo -e "${GREEN}✓ $1${NC}"
}

echo "🔄 Updating Flash Cards application..."

# 1. Build and test locally
echo "Building local test image..."
if docker buildx build --load -t flash-cards:test .; then
    print_status "Local build successful"
else
    echo -e "${RED}Local build failed${NC}"
    exit 1
fi

# 2. Run tests (if any)
echo "Running tests..."
# Add your test commands here

# 3. Build multi-arch and push
echo "Building and pushing multi-arch image..."
if ./build-image.sh "$TAG"; then
    print_status "Multi-arch build and push successful"
else
    echo -e "${RED}Multi-arch build failed${NC}"
    exit 1
fi

# 4. Update kustomization
echo "Updating kustomization..."
sed -i "s/newTag: .*/newTag: $TAG/" "k8s/overlays/$ENV/kustomization.yaml"
print_status "Kustomization updated"

# 5. Deploy update
echo "Deploying update..."
if ./k8s/deploy.sh update "$ENV"; then
    print_status "Deployment successful"
else
    echo -e "${RED}Deployment failed${NC}"
    exit 1
fi

echo -e "\n${GREEN}Update completed successfully!${NC}" 