#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

# Version tag
VERSION=${1:-"one"}

# Image name
IMAGE_NAME="yakovav/flash-cards-ops"

echo "🔨 Building multi-architecture image..."

# Create buildx builder if it doesn't exist
if ! docker buildx inspect multiarch > /dev/null 2>&1; then
    docker buildx create --name multiarch --driver docker-container --use
fi

# Build and push multi-arch image
docker buildx build \
    --platform linux/amd64,linux/arm64,linux/arm/v7 \
    --tag "${IMAGE_NAME}:${VERSION}" \
    --push \
    .

echo -e "${GREEN}✓ Image built and pushed: ${IMAGE_NAME}:${VERSION}${NC}" 