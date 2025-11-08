#!/bin/bash
# Restore BrewGet databases
# This script restores databases from a backup

set -e

echo "♻️  Restoring BrewGet databases..."
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found. Please install kubectl first."
    exit 1
fi

# Check if backup directory is provided
if [ -z "$1" ]; then
    echo "❌ No backup directory specified."
    echo ""
    echo "Usage: $0 <backup-directory>"
    echo ""
    echo "Available backups:"
    if [ -d "./backups" ]; then
        ls -1t ./backups/ | head -5
    else
        echo "  No backups found in ./backups/"
    fi
    exit 1
fi

BACKUP_DIR="$1"

# Check if backup directory exists
if [ ! -d "$BACKUP_DIR" ]; then
    echo "❌ Backup directory not found: $BACKUP_DIR"
    exit 1
fi

# Check if namespace exists
if ! kubectl get namespace brewget &> /dev/null; then
    echo "❌ BrewGet namespace not found. Please deploy BrewGet first."
    exit 1
fi

# Check if postgres pod is running
if ! kubectl get pod postgres-0 -n brewget &> /dev/null 2>&1; then
    echo "❌ PostgreSQL pod not found. Please deploy BrewGet first."
    exit 1
fi

# Check if postgres pod is ready
POD_STATUS=$(kubectl get pod postgres-0 -n brewget -o jsonpath='{.status.phase}')
if [ "$POD_STATUS" != "Running" ]; then
    echo "❌ PostgreSQL pod is not running (status: $POD_STATUS). Cannot restore."
    exit 1
fi

echo "📁 Backup directory: $BACKUP_DIR"
echo ""

# Get postgres credentials
echo "🔑 Retrieving database credentials..."
POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)
POSTGRES_PASSWORD=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-password}' | base64 -d)

if [ -z "$POSTGRES_USER" ] || [ -z "$POSTGRES_PASSWORD" ]; then
    echo "❌ Failed to retrieve database credentials from secrets."
    exit 1
fi

# Wait for postgres to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
kubectl wait --for=condition=ready pod/postgres-0 -n brewget --timeout=60s

echo ""
echo "⚠️  WARNING: This will overwrite existing databases!"
read -p "Are you sure you want to continue? (yes/no): " -r
echo ""

if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo "❌ Restore cancelled."
    exit 0
fi

echo "♻️  Restoring databases..."

# Restore brewget_auth database
if [ -f "$BACKUP_DIR/brewget_auth.sql" ]; then
    echo "  📦 Restoring brewget_auth..."
    if kubectl exec -i postgres-0 -n brewget -- psql -U "$POSTGRES_USER" -d brewget_auth < "$BACKUP_DIR/brewget_auth.sql" > /dev/null 2>&1; then
        echo "  ✅ brewget_auth restored successfully"
    else
        echo "  ⚠️  Failed to restore brewget_auth"
    fi
else
    echo "  ⚠️  brewget_auth.sql not found in backup directory"
fi

# Restore brewget_settings database
if [ -f "$BACKUP_DIR/brewget_settings.sql" ]; then
    echo "  📦 Restoring brewget_settings..."
    if kubectl exec -i postgres-0 -n brewget -- psql -U "$POSTGRES_USER" -d brewget_settings < "$BACKUP_DIR/brewget_settings.sql" > /dev/null 2>&1; then
        echo "  ✅ brewget_settings restored successfully"
    else
        echo "  ⚠️  Failed to restore brewget_settings"
    fi
else
    echo "  ⚠️  brewget_settings.sql not found in backup directory"
fi

# Restore brewget_transactions database
if [ -f "$BACKUP_DIR/brewget_transactions.sql" ]; then
    echo "  📦 Restoring brewget_transactions..."
    if kubectl exec -i postgres-0 -n brewget -- psql -U "$POSTGRES_USER" -d brewget_transactions < "$BACKUP_DIR/brewget_transactions.sql" > /dev/null 2>&1; then
        echo "  ✅ brewget_transactions restored successfully"
    else
        echo "  ⚠️  Failed to restore brewget_transactions"
    fi
else
    echo "  ⚠️  brewget_transactions.sql not found in backup directory"
fi

echo ""
echo "✅ Restore complete!"

# Show backup info if available
if [ -f "$BACKUP_DIR/backup_info.txt" ]; then
    echo ""
    echo "📋 Backup information:"
    cat "$BACKUP_DIR/backup_info.txt"
fi
