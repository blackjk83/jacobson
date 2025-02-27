#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Script variables
TIMEOUT=300
NAMESPACE_PREFIX="flash-cards"
HOME_DIR='/home/angel/k3s/k8s-project-poc/flash-cards'
BACKUP_DIR="${HOME_DIR}/k8s/.backup"
LAST_STATE_FILE="${HOME_DIR}/k8s/.last_state"
MAX_BACKUPS=3
SUPPORTED_ARCHS="amd64,arm64,arm"
VALID_COMMANDS="deploy, status, update, delete, help"
VALID_ENVS="dev, prod"
BUILD_PLATFORM="linux/amd64"
TARGET_PLATFORM="linux/arm64"
YQ_CMD=""

# Function to print status
print_status() {
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ $1${NC}"
    else
        echo -e "${RED}✗ $1${NC}"
        return 1
    fi
}

# Function to log messages
log_message() {
    local level=$1
    local message=$2
    local color=""
    
    case $level in
        "INFO") color=$BLUE ;;
        "WARNING") color=$YELLOW ;;
        "ERROR") color=$RED ;;
        "SUCCESS") color=$GREEN ;;
    esac
    
    echo -e "${color}[$level] $message${NC}"
}

# Function to backup current state
backup_state() {
    mkdir -p "${BACKUP_DIR}"
    if kubectl get -n "${NAMESPACE_PREFIX}-${ENV}" all -o yaml > "${BACKUP_DIR}/backup-${ENV}-$(date +%Y%m%d_%H%M%S).yaml" 2>/dev/null; then
        log_message "INFO" "Current state backed up"
    fi
}

# Function to save deployment progress
save_progress() {
    local stage=$1
    echo "$stage" > "$LAST_STATE_FILE"
}

# Function to rollback deployment
rollback() {
    local backup_file=$(ls -t "${BACKUP_DIR}"/backup-${ENV}-*.yaml 2>/dev/null | head -1)
    if [ -n "$backup_file" ]; then
        log_message "WARNING" "Rolling back to previous state..."
        kubectl apply -f "$backup_file"
        if [ $? -eq 0 ]; then
            log_message "SUCCESS" "Rollback successful"
        else
            log_message "ERROR" "Rollback failed"
        fi
    else
        log_message "WARNING" "No backup found to rollback to"
    fi
}

# Function to check pod health
check_pod_health() {
    local namespace=$1
    local pod_name=$2
    
    # Check pod status
    local pod_status=$(kubectl get pod -n "$namespace" "$pod_name" -o jsonpath='{.status.phase}')
    if [ "$pod_status" != "Running" ]; then
        log_message "ERROR" "Pod $pod_name is not running (Status: $pod_status)"
        # Get pod events
        kubectl get events -n "$namespace" --field-selector involvedObject.name="$pod_name"
        # Get pod logs
        kubectl logs -n "$namespace" "$pod_name" --tail=50
        return 1
    fi
    return 0
}

# Function to build image
build_image() {
    local tag=$1
    local target_arch=$2
    
    log_message "INFO" "Building image for $target_arch"
    docker buildx build \
        --platform $target_arch \
        --build-arg BUILDPLATFORM=$BUILD_PLATFORM \
        --build-arg TARGETPLATFORM=$target_arch \
        --push \
        -t yakovav/flash-cards-ops:$tag .
}

# Function to deploy
deploy() {
    local stage=${1:-"start"}
    
    case $stage in
        "start"|"namespace")
            log_message "INFO" "Creating/updating namespace..."
            kubectl create namespace ${NAMESPACE_PREFIX}-${ENV} --dry-run=client -o yaml | kubectl apply -f -
            print_status "Namespace configured" && save_progress "architecture"
            ;&
        "architecture")
            # Get node architecture
            NODE_ARCH=$(kubectl get nodes -o jsonpath='{.items[0].status.nodeInfo.architecture}')
            log_message "INFO" "Detected node architecture: $NODE_ARCH"
            
            # Set target platform based on node architecture
            case $NODE_ARCH in
                "arm64"|"aarch64")
                    TARGET_PLATFORM="linux/arm64"
                    ;;
                "amd64"|"x86_64")
                    TARGET_PLATFORM="linux/amd64"
                    ;;
                *)
                    log_message "ERROR" "Unsupported architecture: $NODE_ARCH"
                    exit 1
                    ;;
            esac
            
            # Build for target architecture
            build_image "latest" $TARGET_PLATFORM
            
            if [ "$NODE_ARCH" = "arm64" ] || [ "$NODE_ARCH" = "arm" ]; then
                log_message "INFO" "Configuring for ARM64 architecture..."
                log_message "INFO" "Using ARM64 node selector from kustomization"
            fi
            save_progress "deploy"
            ;&
        "deploy")
            log_message "INFO" "Applying Kubernetes configurations..."
            backup_state
            
            # Validate configurations
            if ! validate_kustomization "$ENV"; then
                log_message "ERROR" "Configuration validation failed"
                exit 1
            fi
            
            kubectl apply -k "${HOME_DIR}/k8s/overlays/$ENV/"
            print_status "Deployment applied" && save_progress "wait"
            ;&
        "wait")
            log_message "INFO" "Waiting for deployment to be ready..."
            if ! timeout ${TIMEOUT}s kubectl -n ${NAMESPACE_PREFIX}-${ENV} rollout status deployment/flash-cards; then
                log_message "ERROR" "Deployment timed out"
                echo "Checking pod status..."
                kubectl -n ${NAMESPACE_PREFIX}-${ENV} get pods
                
                POD=$(kubectl -n ${NAMESPACE_PREFIX}-${ENV} get pods -l app=flash-cards -o jsonpath='{.items[0].metadata.name}')
                if [ -n "$POD" ]; then
                    check_pod_health "${NAMESPACE_PREFIX}-${ENV}" "$POD"
                    if [ $? -ne 0 ]; then
                        log_message "WARNING" "Attempting rollback..."
                        rollback
                        exit 1
                    fi
                fi
            fi
            print_status "Deployment ready" && save_progress "complete"
            ;&
        "complete")
            # Get the ingress URL
            INGRESS_HOST=$(kubectl -n ${NAMESPACE_PREFIX}-${ENV} get ingress flash-cards -o jsonpath='{.spec.rules[0].host}')
            # Get the cluster IP
            CLUSTER_IP=$(kubectl get nodes -o jsonpath='{.items[0].status.addresses[?(@.type=="InternalIP")].address}')
            log_message "SUCCESS" "Application deployed successfully!"
            echo "Access the application at: http://$INGRESS_HOST"
            echo -e "\nTo access the application, add this line to your /etc/hosts file:"
            echo -e "${YELLOW}$CLUSTER_IP    $INGRESS_HOST${NC}"
            echo -e "\nYou can do this by running:"
            echo -e "sudo sh -c 'echo \"$CLUSTER_IP    $INGRESS_HOST\" >> /etc/hosts'"
            
            # Print deployment status
            echo -e "\nDeployment Status:"
            kubectl -n ${NAMESPACE_PREFIX}-${ENV} get all
            
            # Clean up
            rm -f "$LAST_STATE_FILE"
            ;;
    esac
}

# Function to print help
print_help() {
    echo -e "${BLUE}Flash Cards Deployment Tool${NC}"
    echo -e "\nUsage: $0 <command> [environment] [options]"
    echo -e "\nCommands:"
    echo -e "  deploy  <env>        Deploy the application"
    echo -e "  status  <env>        Show deployment status"
    echo -e "  update  <env>        Update existing deployment"
    echo -e "  delete  <env>        Remove deployment"
    echo -e "  help                 Show this help message"
    echo -e "\nEnvironments:"
    echo -e "  dev                  Development environment"
    echo -e "  prod                 Production environment"
    echo -e "\nOptions:"
    echo -e "  --resume             Resume from last failed state"
    echo -e "  --force              Force the operation"
}

# Function to show status
show_status() {
    local env=$1
    local ns="${NAMESPACE_PREFIX}-${env}"
    
    # Check if namespace exists
    if ! kubectl get namespace "$ns" >/dev/null 2>&1; then
        log_message "ERROR" "Deployment not found: Environment '$env' does not exist"
        return 1
    fi
    
    # Check if deployment exists
    local deployment_exists=$(kubectl get deployment -n "$ns" flash-cards -o name 2>/dev/null)
    if [ -z "$deployment_exists" ]; then
        echo -e "\n${RED}Deployment Status: NOT RUNNING${NC}"
        echo -e "${YELLOW}No resources found for flash-cards in environment: $env${NC}"
        return 1
    fi

    # Get deployment status
    local replicas=$(kubectl get deployment -n "$ns" flash-cards -o jsonpath='{.status.availableReplicas}')
    if [ -z "$replicas" ] || [ "$replicas" -eq 0 ]; then
        echo -e "\n${RED}Deployment Status: DOWN${NC}"
        echo -e "\n${YELLOW}Quick Status:${NC}"
        kubectl get deployment,pods,svc,ingress -n "$ns" --no-headers
        
        echo -e "\n${YELLOW}Recent Events:${NC}"
        kubectl get events -n "$ns" --sort-by='.lastTimestamp' | tail -5
        return 1
    fi

    echo -e "\n${BLUE}Deployment Status for ${env}${NC}"
    
    # Show architecture information
    echo -e "\n${YELLOW}Node Architecture:${NC}"
    kubectl get nodes -o custom-columns=NAME:.metadata.name,ARCH:.status.nodeInfo.architecture
    
    echo -e "\n${YELLOW}Pods:${NC}"
    kubectl get pods -n "$ns" -o wide
    
    echo -e "\n${YELLOW}Services:${NC}"
    kubectl get services -n "$ns"
    
    echo -e "\n${YELLOW}Ingress:${NC}"
    kubectl get ingress -n "$ns"
    
    # Show network configuration
    show_network_config "$ns"
    
    echo -e "\n${YELLOW}Resource Usage:${NC}"
    kubectl top pods -n "$ns" 2>/dev/null || echo "Metrics not available"
    
    # Show deployment configuration
    echo -e "\n${YELLOW}Deployment Configuration:${NC}"
    kubectl get deployment -n "$ns" flash-cards -o yaml | grep -A 5 nodeSelector || echo "No node selector configured"
    
    # Show available backups
    echo -e "\n${YELLOW}Available Backups:${NC}"
    ls -1t "${BACKUP_DIR}"/backup-${env}-*.yaml 2>/dev/null || echo "No backups found"
}

# Function to update deployment
update_deployment() {
    local env=$1
    local ns="${NAMESPACE_PREFIX}-${env}"
    
    log_message "INFO" "Checking for updates..."
    
    # Backup current state
    backup_state
    
    # Update kustomization if needed
    if [ -f "${HOME_DIR}/k8s/overlays/${env}/kustomization.yaml" ]; then
        log_message "INFO" "Applying configuration updates..."
        kubectl apply -k "${HOME_DIR}/k8s/overlays/${env}/"
        kubectl -n "$ns" rollout restart deployment flash-cards
        
        # Wait for rollout
        log_message "INFO" "Waiting for rollout to complete..."
        kubectl -n "$ns" rollout status deployment/flash-cards
        
        log_message "SUCCESS" "Update completed"
        show_status "$env"
    else
        log_message "ERROR" "Environment configuration not found"
        exit 1
    fi
}

# Function to delete deployment
delete_deployment() {
    local env=$1
    local ns="${NAMESPACE_PREFIX}-${env}"
    
    log_message "WARNING" "Preparing to delete deployment in ${env} environment"
    echo -e "${RED}This will delete all resources in namespace: $ns${NC}"
    read -p "Are you sure you want to continue? (y/N) " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Backup before deletion
        backup_state
        
        log_message "INFO" "Deleting resources..."
        kubectl delete -k "${HOME_DIR}/k8s/overlays/${env}/"
        
        log_message "INFO" "Checking if namespace is empty..."
        if ! kubectl get all -n "$ns" 2>/dev/null | grep -q .; then
            log_message "INFO" "Deleting namespace..."
            kubectl delete namespace "$ns"
        fi
        
        log_message "SUCCESS" "Deployment deleted"
    else
        log_message "INFO" "Deletion cancelled"
    fi
}

# Function to check and set yq command
check_yq() {
    if command -v yq &> /dev/null; then
        YQ_CMD="yq"
    elif command -v yq.v4 &> /dev/null; then
        YQ_CMD="yq.v4"
    else
        log_message "WARNING" "yq not found, falling back to kubectl for validation"
        return 1
    fi
    return 0
}

# Function to validate YAML file
validate_yaml() {
    local file=$1
    local type=$2
    
    if [ ! -f "$file" ]; then
        log_message "ERROR" "File not found: $file"
        return 1
    fi
    
    # Check YAML syntax using kubectl
    if ! kubectl apply --dry-run=client -f "$file" > /dev/null 2>&1; then
        log_message "ERROR" "Invalid YAML syntax in $type: $file"
        return 1
    fi
    
    # Check required fields based on type
    case $type in
        "service")
            # Validate service fields
            if ! grep -q "apiVersion: v1" "$file"; then
                log_message "ERROR" "Invalid apiVersion in service"
                return 1
            fi
            if ! grep -q "kind: Service" "$file"; then
                log_message "ERROR" "Invalid kind in service"
                return 1
            fi
            if ! grep -q "selector:" "$file"; then
                log_message "ERROR" "Missing selector in service"
                return 1
            fi
            if ! grep -q "ports:" "$file"; then
                log_message "ERROR" "Missing ports in service"
                return 1
            fi
            ;;
        "ingress")
            # Validate ingress fields
            if ! grep -q "apiVersion: networking.k8s.io/v1" "$file"; then
                log_message "ERROR" "Invalid apiVersion in ingress"
                return 1
            fi
            if ! grep -q "rules:" "$file"; then
                log_message "ERROR" "Missing rules in ingress"
                return 1
            fi
            ;;
        "deployment")
            # Validate deployment fields
            if ! grep -q "apiVersion: apps/v1" "$file"; then
                log_message "ERROR" "Invalid apiVersion in deployment"
                return 1
            fi
            if ! grep -q "selector:" "$file"; then
                log_message "ERROR" "Missing selector in deployment"
                return 1
            fi
            ;;
    esac
    
    return 0
}

# Function to validate service configuration
validate_service() {
    local file=$1
    local service_type=$(grep "type:" "$file" | awk '{print $2}')
    
    # Validate service type
    case $service_type in
        "NodePort")
            local node_port=$(grep "nodePort:" "$file" | awk '{print $2}')
            if [ -n "$node_port" ]; then
                if [ "$node_port" -lt 30000 ] || [ "$node_port" -gt 32767 ]; then
                    log_message "ERROR" "NodePort must be between 30000-32767"
                    return 1
                fi
            fi
            ;;
        "LoadBalancer"|"ClusterIP")
            ;;
        *)
            log_message "ERROR" "Invalid service type: $service_type"
            return 1
            ;;
    esac
    
    # Validate port configuration
    local port=$(grep "port:" "$file" | awk '{print $2}')
    local target_port=$(grep "targetPort:" "$file" | awk '{print $2}')
    
    if [ -z "$port" ] || [ -z "$target_port" ]; then
        log_message "ERROR" "Missing port or targetPort in service"
        return 1
    fi
    
    # Validate selector matches deployment
    local svc_selector=$(grep "selector:" "$file" | awk '{print $2}')
    local deployment_file="${HOME_DIR}/k8s/base/deployment.yaml"
    if [ -f "$deployment_file" ]; then
        local dep_selector=$(grep "matchLabels:" "$deployment_file" | awk '{print $2}')
        if [ "$svc_selector" != "$dep_selector" ]; then
            log_message "WARNING" "Service selector might not match deployment"
        fi
    fi
    
    return 0
}

# Function to validate kustomization
validate_kustomization() {
    local env=$1
    local kustomization_file="${HOME_DIR}/k8s/overlays/${env}/kustomization.yaml"
    
    # First check if file exists
    if [ ! -f "$kustomization_file" ]; then
        log_message "ERROR" "Kustomization file not found: $kustomization_file"
        return 1
    fi
    
    # Validate kustomization using kustomize
    if ! kubectl kustomize "${HOME_DIR}/k8s/overlays/${env}/" > /dev/null 2>&1; then
        log_message "ERROR" "Invalid kustomization configuration"
        return 1
    fi
    
    # Try a dry-run of the complete configuration
    if ! kubectl apply -k "${HOME_DIR}/k8s/overlays/${env}/" --dry-run=client > /dev/null 2>&1; then
        log_message "ERROR" "Invalid deployment configuration"
        return 1
    fi
    
    # Validate base resources
    if ! validate_yaml "${HOME_DIR}/k8s/base/service.yaml" "service"; then
        log_message "ERROR" "Invalid service configuration"
        return 1
    fi
    if ! validate_service "${HOME_DIR}/k8s/base/service.yaml"; then
        return 1
    fi
    
    if ! validate_yaml "${HOME_DIR}/k8s/base/ingress.yaml" "ingress"; then
        log_message "ERROR" "Invalid ingress configuration"
        return 1
    fi
    
    if ! validate_yaml "${HOME_DIR}/k8s/base/deployment.yaml" "deployment"; then
        log_message "ERROR" "Invalid deployment configuration"
        return 1
    fi
    
    return 0
}

# Function to validate node architecture
validate_architecture() {
    local target_arch=$1
    local valid=false
    
    # Get all node architectures
    local node_archs=$(kubectl get nodes -o jsonpath='{.items[*].status.nodeInfo.architecture}')
    
    # Check if target architecture is available
    for arch in $node_archs; do
        if [ "$arch" = "$target_arch" ]; then
            valid=true
            break
        fi
    done
    
    if ! $valid; then
        log_message "ERROR" "No nodes found with architecture: $target_arch"
        log_message "INFO" "Available architectures: $node_archs"
        return 1
    fi
    
    return 0
}

# Function to manage backups
manage_backups() {
    # Keep only the last MAX_BACKUPS backups
    local backup_count=$(ls -1 "${BACKUP_DIR}"/backup-${ENV}-*.yaml 2>/dev/null | wc -l)
    if [ "$backup_count" -gt "$MAX_BACKUPS" ]; then
        ls -t "${BACKUP_DIR}"/backup-${ENV}-*.yaml | tail -n +$(($MAX_BACKUPS + 1)) | xargs rm -f
    fi
}

# Function to restore backup
restore_backup() {
    local env=$1
    local backup_num=${2:-1}  # Default to most recent backup
    
    local backups=($(ls -t "${BACKUP_DIR}"/backup-${env}-*.yaml 2>/dev/null))
    
    if [ ${#backups[@]} -eq 0 ]; then
        log_message "ERROR" "No backups found"
        return 1
    fi
    
    if [ "$backup_num" -gt ${#backups[@]} ]; then
        log_message "ERROR" "Backup number $backup_num not found. Available backups: ${#backups[@]}"
        return 1
    fi
    
    local backup_file="${backups[$((backup_num-1))]}"
    log_message "INFO" "Restoring from backup: $(basename "$backup_file")"
    
    kubectl apply -f "$backup_file"
}

# Function to show network configuration
show_network_config() {
    local ns=$1
    
    echo -e "\n${YELLOW}Network Configuration:${NC}"
    
    echo -e "\n${BLUE}Network Flow:${NC}"
    echo "External → NodePort (30080) → Service → Pod (8080)"
    
    echo -e "\n${BLUE}Access Methods:${NC}"
    echo "1. NodePort: http://<node-ip>:30080"
    echo "2. Ingress: http://<ingress-host>"
    
    # Get Service details
    local svc_type=$(kubectl get svc -n "$ns" flash-cards -o jsonpath='{.spec.type}')
    local svc_port=$(kubectl get svc -n "$ns" flash-cards -o jsonpath='{.spec.ports[0].port}')
    echo -e "\n${YELLOW}Service Configuration:${NC}"
    echo "Type: $svc_type"
    echo "Port: $svc_port"
    
    # Get Node IPs
    echo -e "\n${YELLOW}Available Nodes:${NC}"
    kubectl get nodes -o custom-columns=NAME:.metadata.name,IP:.status.addresses[0].address,ARCH:.status.nodeInfo.architecture
    
    # Show external IP if LoadBalancer
    if [ "$svc_type" = "LoadBalancer" ]; then
        local external_ip=$(kubectl get svc -n "$ns" flash-cards -o jsonpath='{.status.loadBalancer.ingress[0].ip}')
        echo -e "\n${YELLOW}LoadBalancer:${NC}"
        echo "External IP: $external_ip"
    fi
}

# Main script execution
if [ -z "$1" ] || [ "$1" = "help" ]; then
    print_help
    exit 1
fi

COMMAND=$1
ENV=$2

# Validate command
case $COMMAND in
    deploy|status|update|delete)
        if [ -z "$ENV" ]; then
            log_message "ERROR" "Environment required for $COMMAND command"
            print_help
            exit 1
        fi
        if [[ ! $VALID_ENVS =~ $ENV ]]; then
            log_message "ERROR" "Invalid environment. Use: $VALID_ENVS"
            exit 1
        fi
        ;;
    help)
        print_help
        exit 0
        ;;
    *)
        log_message "ERROR" "Invalid command. Valid commands: $VALID_COMMANDS"
        print_help
        exit 1
        ;;
esac

# Execute command
case $COMMAND in
    deploy)
        if [ "$3" = "--resume" ] && [ -f "$LAST_STATE_FILE" ]; then
            RESUME_STAGE=$(cat "$LAST_STATE_FILE")
            log_message "INFO" "Resuming deployment from stage: $RESUME_STAGE"
            deploy "$RESUME_STAGE"
        else
            deploy "start"
        fi
        ;;
    status)
        show_status "$ENV"
        ;;
    update)
        update_deployment "$ENV"
        ;;
    delete)
        delete_deployment "$ENV"
        ;;
esac 
