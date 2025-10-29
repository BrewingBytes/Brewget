#!/bin/bash
# Manual PostgreSQL Backup Script for BrewGet
# This script creates a backup of all PostgreSQL databases

set -e

echo "🗄️  Manual PostgreSQL Backup"
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found. Please install kubectl first."
    exit 1
fi

# Check if postgres pod is running
if ! kubectl get pod postgres-0 -n brewget &> /dev/null; then
    echo "❌ PostgreSQL pod not found. Make sure the application is deployed."
    exit 1
fi

# Check if pod is ready
POD_STATUS=$(kubectl get pod postgres-0 -n brewget -o jsonpath='{.status.phase}')
if [ "$POD_STATUS" != "Running" ]; then
    echo "❌ PostgreSQL pod is not running (status: $POD_STATUS)"
    exit 1
fi

echo "✓ PostgreSQL pod is running"
echo ""

# Get PostgreSQL username
POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)
echo "📋 PostgreSQL user: $POSTGRES_USER"
echo ""

# Trigger backup using the backup script
echo "🔄 Executing backup..."
kubectl exec postgres-0 -n brewget -- /bin/sh /backup-scripts/backup.sh 2>&1

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Backup completed successfully!"
    echo ""
    echo "📂 Backup location: /backup/latest in postgres-0 pod"
    echo ""
    echo "To list backups:"
    echo "  kubectl exec postgres-0 -n brewget -- ls -lah /backup"
    echo ""
    echo "To download backup to local machine:"
    echo "  kubectl cp brewget/postgres-0:/backup/latest ./postgres-backup-\$(date +%Y%m%d)"
else
    echo ""
    echo "❌ Backup failed!"
    exit 1
fi
