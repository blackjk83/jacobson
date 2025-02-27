const kubernetesFlashcards = [
    {
        question: "What is a container, and how does it differ from a virtual machine?",
        answer: "A container packages an application and its dependencies together. It shares the host OS kernel, making it lightweight and fast to start. A VM emulates an entire operating system, adding overhead but providing stronger isolation."
    },
    {
        question: "Explain the concept of container images and registries.",
        answer: "A container image is a read-only template used to create containers. It contains everything the application needs to run. A container registry (like Docker Hub) is a storage and distribution system for container images, allowing you to share and manage them."
    },
    {
        question: "What is Kubernetes, and what problems does it solve?",
        answer: "Kubernetes is an open-source container orchestration platform. It automates the deployment, scaling, and management of containerized applications. It solves problems like scaling, high availability, resource management, and service discovery."
    },
    {
        question: "What are the key components of a Kubernetes cluster?",
        answer: "Key components include the Control Plane (API server, scheduler, controller manager, etcd), Nodes (worker machines), Kubelet (agent on each node), and Kube-proxy (network proxy)."
    },
    {
        question: "What is a Pod in Kubernetes, and what is its relationship to containers?",
        answer: "A Pod is the smallest deployable unit in Kubernetes. It can contain one or more containers that share the same network namespace and storage volumes."
    },
    {
        question: "Deployment vs. StatefulSet?",
        answer: "Deployment: Manages stateless applications. StatefulSet: Manages stateful applications, providing stable network IDs and persistent storage."
    },
    {
        question: "How to expose a service externally?",
        answer: "Use NodePort, LoadBalancer, or Ingress. NodePort exposes on a static port. LoadBalancer creates a cloud load balancer. Ingress manages external access, typically HTTP/HTTPS."
    },
    {
        question: "ConfigMaps and Secrets?",
        answer: "ConfigMaps store non-sensitive configuration data. Secrets store sensitive data securely."
    },
    {
        question: "How does Kubernetes handle scaling?",
        answer: "Horizontal Pod Autoscaler (HPA) automatically adjusts the number of Pods based on metrics like CPU utilization."
    },
    {
        question: "What are Namespaces?",
        answer: "Namespaces provide logical isolation for resources within a Kubernetes cluster."
    },
    {
        question: "Resource limits and requests?",
        answer: "Requests: Minimum resources a Pod needs. Limits: Maximum resources a Pod can use."
    },
    {
        question: "Rolling updates and rollbacks?",
        answer: "Kubernetes Deployments support rolling updates to minimize downtime. Rollbacks revert to a previous version."
    },
    {
        question: "Troubleshooting a failing Pod?",
        answer: "Check logs, describe the Pod, check events, ensure image pull secrets are correct, and verify resource limits."
    },
    {
        question: "Kubernetes security best practices?",
        answer: "Use RBAC, secure etcd, use network policies, scan images, and regularly update Kubernetes."
    },
    {
        question: "Monitoring and logging tools?",
        answer: "Common tools: Prometheus, Grafana, EFK stack, and cloud-provider specific solutions."
    }
]; 