#!/bin/bash
# Deploy BrewGet to Kubernetes
# This script applies all Kubernetes manifests in the correct order
#
# Usage: ./deploy.sh [--no-backup]
#   --no-backup: Skip automatic database backup before deployment

set -e

echo "🚀 Deploying BrewGet to Kubernetes..."
echo ""

# Parse command line arguments
SKIP_BACKUP=false
while [[ $# -gt 0 ]]; do
    case $1 in
        --no-backup)
            SKIP_BACKUP=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--no-backup]"
            exit 1
            ;;
    esac
done

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found. Please install kubectl first."
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Automatically backup databases if they exist (unless --no-backup is specified)
if [ "$SKIP_BACKUP" = false ]; then
    if kubectl get namespace brewget &> /dev/null && kubectl get pod postgres-0 -n brewget &> /dev/null 2>&1; then
        POD_STATUS=$(kubectl get pod postgres-0 -n brewget -o jsonpath='{.status.phase}' 2>/dev/null || echo "Unknown")
        if [ "$POD_STATUS" = "Running" ]; then
            echo "💾 Existing deployment detected. Creating automatic backup..."
            echo "   (Use --no-backup to skip this step)"
            echo ""
            
            # Give user a chance to cancel
            echo "⏳ Starting backup in 3 seconds... (Press Ctrl+C to cancel)"
            sleep 1
            echo "⏳ Starting backup in 2 seconds... (Press Ctrl+C to cancel)"
            sleep 1
            echo "⏳ Starting backup in 1 second... (Press Ctrl+C to cancel)"
            sleep 1
            echo ""
            
            "$SCRIPT_DIR/backup-db.sh" || echo "⚠️  Backup failed, continuing with deployment..."
            echo ""
        else
            echo "ℹ️  PostgreSQL pod exists but is not running (status: $POD_STATUS), skipping backup."
            echo ""
        fi
    else
        echo "ℹ️  No existing deployment found, skipping backup."
        echo ""
    fi
else
    echo "⏩ Skipping automatic backup (--no-backup flag provided)."
    echo ""
fi

# Set up persistent storage on host machine if minikube is available
if command -v minikube &> /dev/null; then
    # Define the host path for persistent storage
    HOST_DATA_PATH="${HOME}/.brewget-data/postgres"
    
    # Create the directory on host if it doesn't exist
    if [ ! -d "$HOST_DATA_PATH" ]; then
        echo "📁 Creating host data directory at $HOST_DATA_PATH..."
        mkdir -p "$HOST_DATA_PATH"
    fi
    
    # Check if minikube is running
    if ! minikube status &> /dev/null; then
        echo "🚀 Starting minikube..."
        minikube start --force --mount --mount-string="$HOST_DATA_PATH:/data/brewget-postgres"
        echo "✅ Minikube started successfully"
    else
        echo "📦 Minikube is already running"
        
        # Check if mount is already running
        if ! pgrep -f "minikube mount.*$HOST_DATA_PATH" > /dev/null; then
            echo "💾 Mounting host folder for persistent storage..."
            echo "   Host path: $HOST_DATA_PATH"
            echo "   Minikube path: /data/brewget-postgres"
            minikube mount "$HOST_DATA_PATH:/data/brewget-postgres" &
            MOUNT_PID=$!
            sleep 5  # Give the mount time to establish
            
            # Verify mount process is still running
            if ps -p $MOUNT_PID > /dev/null 2>&1; then
                echo "✅ Host folder mounted successfully"
            else
                echo "⚠️  Mount process may have failed, but continuing deployment"
            fi
        else
            echo "✅ Host folder already mounted"
        fi
    fi
    
    echo ""
    
    # Set up minikube tunnel as a systemd service
    echo "🌐 Setting up minikube tunnel as a systemd service..."
    
    # Detect minikube path
    MINIKUBE_PATH=$(which minikube)
    if [ -z "$MINIKUBE_PATH" ]; then
        echo "⚠️  Warning: Could not detect minikube path, using default /usr/local/bin/minikube"
        MINIKUBE_PATH="/usr/local/bin/minikube"
    fi
    
    # Create service file with the correct minikube path
    sed "s|MINIKUBE_PATH|$MINIKUBE_PATH|g" "$SCRIPT_DIR/minikube-tunnel.service" > /tmp/minikube-tunnel.service
    
    # Copy the service file to systemd
    sudo cp /tmp/minikube-tunnel.service /etc/systemd/system/minikube-tunnel.service
    
    # Clean up temporary file
    rm -f /tmp/minikube-tunnel.service
    
    # Reload systemd to recognize the new service
    sudo systemctl daemon-reload
    
    # Enable the service to start on boot
    sudo systemctl enable minikube-tunnel.service
    
    # Start the service
    sudo systemctl start minikube-tunnel.service
    
    # Check if service started successfully
    if sudo systemctl is-active --quiet minikube-tunnel.service; then
        echo "✅ Minikube tunnel service started successfully"
    else
        echo "⚠️  Warning: Minikube tunnel service may not have started correctly"
        echo "   Check status with: sudo systemctl status minikube-tunnel.service"
    fi
    echo ""
fi

# Apply manifests in order
echo "📦 Creating namespace..."
kubectl apply -f "$SCRIPT_DIR/00-namespace.yaml"

echo "📝 Creating shared configuration..."
kubectl apply -f "$SCRIPT_DIR/01-shared-config.yaml"

echo "🔐 Creating secrets..."
kubectl apply -f "$SCRIPT_DIR/02-secrets.yaml"

echo "📝 Creating configmaps..."
kubectl apply -f "$SCRIPT_DIR/03-configmaps.yaml"

echo "🗄️  Deploying PostgreSQL..."
echo "  Creating PersistentVolume first..."
kubectl apply -f "$SCRIPT_DIR/04-postgres-pv.yaml"

echo "  Waiting for PV to be available..."
sleep 2  # Give PV time to be created

echo "  Creating PostgreSQL service, PVC, and StatefulSet..."
kubectl apply -f "$SCRIPT_DIR/04-postgres.yaml"

echo "⏳ Waiting for PostgreSQL PVC to be bound..."
kubectl wait --for=jsonpath='{.status.phase}'=Bound pvc/brewget-postgres-pvc -n brewget --timeout=120s || echo "⚠️  PVC binding may take some time, continuing..."

echo "📧 Deploying email service..."
kubectl apply -f "$SCRIPT_DIR/05-email-service.yaml"

echo "🔑 Deploying auth service..."
kubectl apply -f "$SCRIPT_DIR/06-auth-service.yaml"

echo "⚙️  Deploying settings service..."
kubectl apply -f "$SCRIPT_DIR/07-settings-service.yaml"

echo "💳 Deploying transaction service..."
kubectl apply -f "$SCRIPT_DIR/07.5-transaction-service.yaml"

echo "🎨 Deploying frontend..."
kubectl apply -f "$SCRIPT_DIR/08-frontend.yaml"

echo "🌐 Deploying nginx..."
kubectl apply -f "$SCRIPT_DIR/09-nginx.yaml"

echo ""
echo "✅ All manifests applied successfully!"
echo ""
echo "⏳ Waiting for pods to be ready..."
kubectl wait --for=condition=ready pod --all -n brewget --timeout=300s || true

echo ""
echo "📊 Current status:"
kubectl get pods -n brewget

echo ""
echo "🌐 Services:"
kubectl get services -n brewget

echo ""
echo "✅ Deployment complete!"
echo ""
echo "To access the application:"
echo "  1. Get the nginx service external IP:"
echo "     kubectl get service nginx -n brewget"
echo ""
echo "  2. Or use port forwarding for local access:"
echo "     kubectl port-forward -n brewget service/nginx 8080:80"
echo "     Then access at http://localhost:8080"
echo ""
echo "To check logs:"
echo "  kubectl logs -n brewget <pod-name>"
echo ""
echo "To check all resources:"
echo "  kubectl get all -n brewget"
