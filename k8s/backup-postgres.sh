#!/bin/bash
# Backup PostgreSQL databases from Kubernetes to host machine
# This script creates SQL dumps of all databases and saves them to the host

set -e

# Configuration
NAMESPACE="brewget"
POD_NAME="postgres-0"
BACKUP_DIR="${BACKUP_DIR:-./postgres-backups}"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔄 Starting PostgreSQL backup..."
echo ""

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo -e "${RED}❌ kubectl not found. Please install kubectl first.${NC}"
    exit 1
fi

# Check if namespace exists
if ! kubectl get namespace "$NAMESPACE" &> /dev/null; then
    echo -e "${RED}❌ Namespace '$NAMESPACE' not found.${NC}"
    exit 1
fi

# Check if postgres pod is running
if ! kubectl get pod "$POD_NAME" -n "$NAMESPACE" &> /dev/null; then
    echo -e "${RED}❌ PostgreSQL pod '$POD_NAME' not found in namespace '$NAMESPACE'.${NC}"
    exit 1
fi

# Check if pod is ready
if ! kubectl wait --for=condition=ready pod/"$POD_NAME" -n "$NAMESPACE" --timeout=10s &> /dev/null; then
    echo -e "${YELLOW}⚠️  PostgreSQL pod is not ready. Backup may fail.${NC}"
fi

# Get database credentials from secrets
echo "🔐 Retrieving database credentials..."
POSTGRES_USER=$(kubectl get secret brewget-secrets -n "$NAMESPACE" -o jsonpath='{.data.postgres-user}' | base64 -d)
POSTGRES_PASSWORD=$(kubectl get secret brewget-secrets -n "$NAMESPACE" -o jsonpath='{.data.postgres-password}' | base64 -d)

if [ -z "$POSTGRES_USER" ] || [ -z "$POSTGRES_PASSWORD" ]; then
    echo -e "${RED}❌ Failed to retrieve database credentials from secrets.${NC}"
    exit 1
fi

# Create backup directory
mkdir -p "$BACKUP_DIR"
echo "📁 Backup directory: $BACKUP_DIR"
echo ""

# Function to backup a database
backup_database() {
    local db_name=$1
    local backup_file="$BACKUP_DIR/${db_name}_${TIMESTAMP}.sql"
    
    echo "📦 Backing up database: $db_name"
    
    # Run pg_dump inside the pod and save to local file
    if kubectl exec "$POD_NAME" -n "$NAMESPACE" -- env PGPASSWORD="$POSTGRES_PASSWORD" pg_dump -U "$POSTGRES_USER" "$db_name" > "$backup_file" 2>/dev/null; then
        # Get file size
        local file_size=$(du -h "$backup_file" | cut -f1)
        echo -e "${GREEN}✅ Backup successful: $backup_file ($file_size)${NC}"
        return 0
    else
        echo -e "${RED}❌ Failed to backup $db_name${NC}"
        rm -f "$backup_file"
        return 1
    fi
}

# Backup all brewget databases
DATABASES=("postgres" "brewget_auth" "brewget_settings")
FAILED=0

for db in "${DATABASES[@]}"; do
    if ! backup_database "$db"; then
        FAILED=$((FAILED + 1))
    fi
    echo ""
done

# Create a combined backup archive
echo "📦 Creating backup archive..."
ARCHIVE_NAME="brewget_postgres_backup_${TIMESTAMP}.tar.gz"
ARCHIVE_PATH="$BACKUP_DIR/$ARCHIVE_NAME"

if tar -czf "$ARCHIVE_PATH" -C "$BACKUP_DIR" $(ls "$BACKUP_DIR" | grep "_${TIMESTAMP}.sql") 2>/dev/null; then
    ARCHIVE_SIZE=$(du -h "$ARCHIVE_PATH" | cut -f1)
    echo -e "${GREEN}✅ Archive created: $ARCHIVE_PATH ($ARCHIVE_SIZE)${NC}"
    
    # Optionally remove individual SQL files after archiving
    # Uncomment the next line if you want to keep only the archive
    # rm -f "$BACKUP_DIR"/*_${TIMESTAMP}.sql
else
    echo -e "${YELLOW}⚠️  Failed to create archive, but individual backups are available${NC}"
fi

echo ""
echo "📊 Backup Summary:"
echo "  - Total databases: ${#DATABASES[@]}"
echo "  - Successful: $((${#DATABASES[@]} - FAILED))"
echo "  - Failed: $FAILED"
echo "  - Location: $BACKUP_DIR"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}✅ All backups completed successfully!${NC}"
    exit 0
else
    echo -e "${YELLOW}⚠️  Some backups failed. Please check the output above.${NC}"
    exit 1
fi
