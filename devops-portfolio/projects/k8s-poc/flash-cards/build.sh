#!/bin/bash

# Get host architecture
HOST_ARCH=$(uname -m)
case $HOST_ARCH in
    "x86_64")
        BUILD_PLATFORM="linux/amd64"
        ;;
    "aarch64")
        BUILD_PLATFORM="linux/arm64"
        ;;
    *)
        echo "Unsupported architecture: $HOST_ARCH"
        exit 1
        ;;
esac

# Default target platform
TARGET_PLATFORM="linux/arm64"

# Build image
echo "Building image for $TARGET_PLATFORM from $BUILD_PLATFORM..."

docker buildx build \
    --platform $TARGET_PLATFORM \
    --build-arg BUILDPLATFORM=$BUILD_PLATFORM \
    --build-arg TARGETPLATFORM=$TARGET_PLATFORM \
    --progress=plain \
    --push \
    -t yakovav/flash-cards-ops:latest \
    -t yakovav/flash-cards-ops:arm64 .

# Verify build
echo "Image built for $TARGET_PLATFORM from $BUILD_PLATFORM"
docker buildx imagetools inspect yakovav/flash-cards-ops:arm64 