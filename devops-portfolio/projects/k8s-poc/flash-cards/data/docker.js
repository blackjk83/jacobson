const dockerFlashcards = [
    {
        question: "What is Docker?",
        answer: "Docker is a platform for developing, shipping, and running applications in containers. It enables you to separate your applications from your infrastructure for fast delivery."
    },
    {
        question: "What is a Docker container?",
        answer: "A Docker container is a lightweight, standalone, executable package that includes everything needed to run an application: code, runtime, system tools, libraries, and settings."
    },
    {
        question: "Difference between Docker Image and Container?",
        answer: "An image is a read-only template with instructions for creating a container. A container is a runnable instance of an image. You can create, start, stop, move, or delete containers using Docker API or CLI."
    },
    {
        question: "What is Dockerfile?",
        answer: "A Dockerfile is a text document containing commands to assemble a Docker image. Docker can automatically build images by reading these instructions."
    },
    {
        question: "What is Docker Compose?",
        answer: "Docker Compose is a tool for defining and running multi-container Docker applications. It uses YAML files to configure application services and creates a isolated environment for your app."
    },
    {
        question: "What are Docker volumes?",
        answer: "Docker volumes are the preferred way to persist data generated and used by Docker containers. They are completely managed by Docker and are independent of the container lifecycle."
    },
    {
        question: "What is Docker Hub?",
        answer: "Docker Hub is a cloud-based registry service that allows you to link to code repositories, build images, store manually pushed images, and links to Docker Cloud for deploying images to your hosts."
    },
    {
        question: "Docker networking types?",
        answer: "Docker supports several networking types: bridge (default), host, none, overlay (for swarm), macvlan, and custom network drivers. Each serves different networking needs."
    },
    {
        question: "What is Docker Swarm?",
        answer: "Docker Swarm is Docker's native clustering and orchestration solution. It turns a pool of Docker hosts into a single, virtual Docker host with built-in orchestration features."
    },
    {
        question: "Best practices for Docker security?",
        answer: "Use official images, scan for vulnerabilities, don't run as root, use secrets management, limit container resources, use security options like --security-opt, and keep Docker updated."
    }
    // ... rest of Docker questions ...
]; 