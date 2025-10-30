#!/bin/bash
# Cleanup BrewGet Kubernetes deployment
# This script removes all BrewGet resources from the cluster

set -e

echo "🧹 Cleaning up BrewGet from Kubernetes..."
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found. Please install kubectl first."
    exit 1
fi

# Check if namespace exists
if ! kubectl get namespace brewget &> /dev/null; then
    echo "ℹ️  BrewGet namespace not found. Nothing to clean up."
    exit 0
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
