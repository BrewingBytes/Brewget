#!/bin/bash
# Backup BrewGet databases
# This script backs up all databases before deployment or maintenance

set -e

echo "💾 Backing up BrewGet databases..."
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo "❌ kubectl not found. Please install kubectl first."
    exit 1
fi

# Check if namespace exists
if ! kubectl get namespace brewget &> /dev/null; then
    echo "ℹ️  BrewGet namespace not found. No databases to backup."
    exit 0
fi

# Check if postgres pod is running
if ! kubectl get pod postgres-0 -n brewget &> /dev/null 2>&1; then
    echo "ℹ️  PostgreSQL pod not found. No databases to backup."
    exit 0
fi

# Check if postgres pod is ready
POD_STATUS=$(kubectl get pod postgres-0 -n brewget -o jsonpath='{.status.phase}')
if [ "$POD_STATUS" != "Running" ]; then
    echo "⚠️  PostgreSQL pod is not running (status: $POD_STATUS). Cannot backup."
    exit 1
fi

# Create backup directory with timestamp
BACKUP_DIR="./backups/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$BACKUP_DIR"

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
kubectl wait --for=condition=ready pod/postgres-0 -n brewget --timeout=30s || {
    echo "⚠️  PostgreSQL pod not ready after 30 seconds. Attempting backup anyway..."
}

echo ""
echo "💾 Backing up databases..."

# Backup brewget_auth database
echo "  📦 Backing up brewget_auth..."
if kubectl exec postgres-0 -n brewget -- pg_dump -U "$POSTGRES_USER" brewget_auth > "$BACKUP_DIR/brewget_auth.sql" 2>/dev/null; then
    echo "  ✅ brewget_auth backed up successfully"
else
    echo "  ⚠️  Failed to backup brewget_auth (may not exist yet)"
fi

# Backup brewget_settings database
echo "  📦 Backing up brewget_settings..."
if kubectl exec postgres-0 -n brewget -- pg_dump -U "$POSTGRES_USER" brewget_settings > "$BACKUP_DIR/brewget_settings.sql" 2>/dev/null; then
    echo "  ✅ brewget_settings backed up successfully"
else
    echo "  ⚠️  Failed to backup brewget_settings (may not exist yet)"
fi

# Create a metadata file
cat > "$BACKUP_DIR/backup_info.txt" << EOF
Backup Date: $(date)
Kubernetes Namespace: brewget
PostgreSQL User: $POSTGRES_USER
Databases: brewget_auth, brewget_settings
EOF

echo ""
echo "✅ Backup complete!"
echo "   Location: $BACKUP_DIR"
echo ""
echo "To restore this backup later, run:"
echo "  ./restore-db.sh $BACKUP_DIR"
