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
echo "⚠️  WARNING: This will delete all BrewGet resources!"
echo "   The namespace and all pods will be removed."
echo ""
echo "   NOTE: PostgreSQL backups stored in /mnt/data/brewget-postgres-backups"
echo "   on the host will be preserved and can be used when redeploying."
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
echo "✅ Cleanup complete!"
echo "   All BrewGet resources have been removed."
echo ""
echo "💡 Note: PostgreSQL backups on the host are preserved."
echo "   Location: /mnt/data/brewget-postgres-backups (inside minikube)"
echo "   These backups will be automatically restored when you redeploy."
