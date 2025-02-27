# DevOps Flash Cards

A lightweight web application for learning DevOps concepts through flash cards. Covers Kubernetes, Docker, and CI/CD topics.

## Features

- 🎯 Interactive flash cards
- 📱 Responsive design
- 🌓 Dark mode support
- 🔄 Multiple DevOps topics
- 🚀 Docker-ready
- ⚡ Fast and lightweight

## Quick Start

### Local Development
```bash
# Clone the repository
git clone https://github.com/yourusername/flash-cards.git
cd flash-cards

# Open in browser
open index.html
```

### Docker
```bash
# Build the image
./setup.sh

# Run locally
docker run -d -p 8080:8080 flash-cards:latest

# Access the application
open http://localhost:8080
```

### Kubernetes
```bash
# Deploy to cluster
kubectl apply -f k8s/

# Access via ingress or port-forward
kubectl port-forward svc/flash-cards 8080:8080
```

## Architecture

The application uses a simple, static architecture:
- HTML5 for structure
- CSS3 for styling
- Vanilla JavaScript for interactivity
- Nginx for serving content
- Docker for containerization

## Topics Covered

- Kubernetes (K8s)
  - Architecture
  - Components
  - Workloads
  - Networking
  - Storage

- Docker
  - Containers
  - Images
  - Networking
  - Volumes
  - Security

- CI/CD
  - Continuous Integration
  - Continuous Delivery
  - Continuous Deployment
  - Tools and Practices

## Security

- Non-root container execution
- Security headers enabled
- Content Security Policy
- Asset caching
- HTTPS ready

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

MIT License - See [LICENSE](LICENSE) for details

## Deployment

### Kubernetes Deployment

The application can be deployed to Kubernetes using Kustomize:

```bash
# Deploy to development environment
./k8s/deploy.sh dev

# Deploy to production environment
./k8s/deploy.sh prod
```

#### Prerequisites

- Kubernetes cluster
- kubectl configured
- Ingress controller installed
- Docker registry access (if using private registry)

#### Configuration

The deployment can be customized through:
- `k8s/base/`: Base Kubernetes configurations
- `k8s/overlays/dev/`: Development-specific configurations
- `k8s/overlays/prod/`: Production-specific configurations
