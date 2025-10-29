#!/bin/bash
# Manual PostgreSQL Restore Script for BrewGet
# This script restores PostgreSQL databases from the latest backup

set -e

echo "♻️  Manual PostgreSQL Restore"
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

# Check if backup exists
echo "🔍 Checking for available backups..."
BACKUP_CHECK=$(kubectl exec postgres-0 -n brewget -- ls /backup/latest 2>&1 || echo "not found")

if [[ "$BACKUP_CHECK" == *"not found"* ]] || [[ "$BACKUP_CHECK" == *"No such file"* ]]; then
    echo "❌ No backup found at /backup/latest"
    echo ""
    echo "Available backups:"
    kubectl exec postgres-0 -n brewget -- ls -lh /backup/ 2>&1 || echo "No backups available"
    exit 1
fi

echo "✓ Backup found at /backup/latest"
echo ""

# Warning
echo "⚠️  WARNING: This will restore data from the backup."
echo "   If databases already contain data, this operation will be skipped to prevent data loss."
echo "   To force restore, you need to manually drop the databases first."
echo ""
read -p "Do you want to continue? (yes/no): " -r
echo ""

if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo "❌ Restore cancelled."
    exit 0
fi

# Trigger restore using the restore script
echo "🔄 Executing restore..."
kubectl exec postgres-0 -n brewget -- /bin/sh /backup-scripts/restore.sh 2>&1

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Restore process completed!"
    echo ""
    echo "Note: The restore script only restores to empty databases."
    echo "Check the output above to see which databases were restored."
else
    echo ""
    echo "❌ Restore failed!"
    exit 1
fi
