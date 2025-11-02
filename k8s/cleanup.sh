#!/bin/bash
# Cleanup BrewGet Kubernetes deployment
# This script removes all BrewGet resources from the cluster
#
# Usage: ./cleanup.sh [--no-backup]
#   --no-backup: Skip automatic database backup before cleanup

set -e

echo "🧹 Cleaning up BrewGet from Kubernetes..."
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

# Check if namespace exists
if ! kubectl get namespace brewget &> /dev/null; then
    echo "ℹ️  BrewGet namespace not found. Nothing to clean up."
    exit 0
fi

# Automatically backup databases before cleanup (unless --no-backup is specified)
if [ "$SKIP_BACKUP" = false ]; then
    if kubectl get pod postgres-0 -n brewget &> /dev/null 2>&1; then
        POD_STATUS=$(kubectl get pod postgres-0 -n brewget -o jsonpath='{.status.phase}' 2>/dev/null || echo "Unknown")
        if [ "$POD_STATUS" = "Running" ]; then
            echo "💾 Creating automatic backup before cleanup..."
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
            
            "$SCRIPT_DIR/backup-db.sh" || echo "⚠️  Backup failed, continuing with cleanup..."
            echo ""
        else
            echo "ℹ️  PostgreSQL pod exists but is not running (status: $POD_STATUS), skipping backup."
            echo ""
        fi
    else
        echo "ℹ️  PostgreSQL pod not found, skipping backup."
        echo ""
    fi
else
    echo "⏩ Skipping automatic backup (--no-backup flag provided)."
    echo ""
fi

# Ask for confirmation
echo "⚠️  WARNING: This will delete all BrewGet resources including the database!"
echo "   All data will be permanently lost."
echo ""
read -p "Are you sure you want to continue? (yes/no): " -r
echo ""

if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo "❌ Cleanup cancelled."
    exit 0
fi

echo "🗑️  Deleting all resources in brewget namespace..."
kubectl delete namespace brewget

echo ""
echo "🗑️  Deleting PersistentVolume..."
kubectl delete pv brewget-postgres-pv --ignore-not-found=true

# Stop and remove minikube tunnel service if it exists
if systemctl list-unit-files | grep -q minikube-tunnel.service; then
    echo ""
    echo "🗑️  Stopping and removing minikube tunnel service..."
    
    # Stop the service
    sudo systemctl stop minikube-tunnel.service 2>/dev/null || true
    
    # Disable the service
    sudo systemctl disable minikube-tunnel.service 2>/dev/null || true
    
    # Remove the service file
    sudo rm -f /etc/systemd/system/minikube-tunnel.service
    
    # Reload systemd
    sudo systemctl daemon-reload
    
    echo "✅ Minikube tunnel service removed"
fi

# Delete minikube if it exists
if command -v minikube &> /dev/null; then
    if minikube status &> /dev/null; then
        echo ""
        echo "🗑️  Deleting minikube cluster..."
        if minikube delete; then
            echo "✅ Minikube cluster deleted"
        else
            echo "⚠️  Failed to delete minikube cluster. You may need to run 'minikube delete' manually."
        fi
    else
        echo ""
        echo "ℹ️  Minikube cluster not running, skipping deletion"
    fi
fi

echo ""
echo "✅ Cleanup complete!"
echo "   All BrewGet resources have been removed."
