#!/bin/bash
# Restore PostgreSQL databases from backup files to Kubernetes
# This script restores SQL dumps from the host machine to the Postgres pod

set -e

# Configuration
NAMESPACE="brewget"
POD_NAME="postgres-0"
BACKUP_DIR="${BACKUP_DIR:-./postgres-backups}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to display usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Restore PostgreSQL databases from backup files."
    echo ""
    echo "Options:"
    echo "  -f, --file FILE       Restore from a specific backup file (SQL dump)"
    echo "  -a, --archive FILE    Restore from a backup archive (tar.gz)"
    echo "  -d, --database DB     Database name to restore (required with --file)"
    echo "  -l, --list           List available backups"
    echo "  -h, --help           Display this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --list"
    echo "  $0 --file ./postgres-backups/brewget_auth_20231030_120000.sql --database brewget_auth"
    echo "  $0 --archive ./postgres-backups/brewget_postgres_backup_20231030_120000.tar.gz"
    echo ""
    exit 1
}

# Function to list available backups
list_backups() {
    echo "📋 Available backups in $BACKUP_DIR:"
    echo ""
    
    if [ ! -d "$BACKUP_DIR" ]; then
        echo -e "${YELLOW}⚠️  Backup directory does not exist: $BACKUP_DIR${NC}"
        exit 0
    fi
    
    # List SQL files
    echo "SQL Backup Files:"
    if ls "$BACKUP_DIR"/*.sql &> /dev/null; then
        ls -lh "$BACKUP_DIR"/*.sql | awk '{print "  " $9 " (" $5 ")"}'
    else
        echo "  None found"
    fi
    echo ""
    
    # List archive files
    echo "Archive Files:"
    if ls "$BACKUP_DIR"/*.tar.gz &> /dev/null; then
        ls -lh "$BACKUP_DIR"/*.tar.gz | awk '{print "  " $9 " (" $5 ")"}'
    else
        echo "  None found"
    fi
    echo ""
    
    exit 0
}

# Function to restore from a SQL file
restore_from_file() {
    local backup_file=$1
    local db_name=$2
    
    if [ ! -f "$backup_file" ]; then
        echo -e "${RED}❌ Backup file not found: $backup_file${NC}"
        exit 1
    fi
    
    if [ -z "$db_name" ]; then
        echo -e "${RED}❌ Database name is required when restoring from a file.${NC}"
        echo "Use: $0 --file $backup_file --database DATABASE_NAME"
        exit 1
    fi
    
    echo "🔄 Restoring database '$db_name' from: $backup_file"
    
    # Copy backup file to pod
    local tmp_file="/tmp/restore_$(basename "$backup_file")"
    echo "📤 Copying backup file to pod..."
    if ! kubectl cp "$backup_file" "$NAMESPACE/$POD_NAME:$tmp_file"; then
        echo -e "${RED}❌ Failed to copy backup file to pod${NC}"
        exit 1
    fi
    
    # Restore database
    echo "🔄 Restoring database..."
    if kubectl exec "$POD_NAME" -n "$NAMESPACE" -- env PGPASSWORD="$POSTGRES_PASSWORD" psql -U "$POSTGRES_USER" -d "$db_name" -f "$tmp_file" &> /dev/null; then
        echo -e "${GREEN}✅ Database '$db_name' restored successfully${NC}"
        
        # Clean up temp file
        kubectl exec "$POD_NAME" -n "$NAMESPACE" -- rm -f "$tmp_file" &> /dev/null
        return 0
    else
        echo -e "${RED}❌ Failed to restore database '$db_name'${NC}"
        kubectl exec "$POD_NAME" -n "$NAMESPACE" -- rm -f "$tmp_file" &> /dev/null
        return 1
    fi
}

# Function to restore from an archive
restore_from_archive() {
    local archive_file=$1
    
    if [ ! -f "$archive_file" ]; then
        echo -e "${RED}❌ Archive file not found: $archive_file${NC}"
        exit 1
    fi
    
    echo "🔄 Restoring from archive: $archive_file"
    
    # Extract archive to temporary directory
    local temp_dir=$(mktemp -d)
    echo "📦 Extracting archive..."
    if ! tar -xzf "$archive_file" -C "$temp_dir"; then
        echo -e "${RED}❌ Failed to extract archive${NC}"
        rm -rf "$temp_dir"
        exit 1
    fi
    
    # Restore each database
    local failed=0
    for sql_file in "$temp_dir"/*.sql; do
        if [ -f "$sql_file" ]; then
            # Extract database name from filename (format: dbname_timestamp.sql)
            local db_name=$(basename "$sql_file" .sql | sed 's/_[0-9]*_[0-9]*$//')
            
            echo ""
            if ! restore_from_file "$sql_file" "$db_name"; then
                failed=$((failed + 1))
            fi
        fi
    done
    
    # Clean up temp directory
    rm -rf "$temp_dir"
    
    echo ""
    if [ $failed -eq 0 ]; then
        echo -e "${GREEN}✅ All databases restored successfully from archive!${NC}"
        return 0
    else
        echo -e "${YELLOW}⚠️  Some databases failed to restore. Please check the output above.${NC}"
        return 1
    fi
}

# Parse command line arguments
BACKUP_FILE=""
ARCHIVE_FILE=""
DATABASE_NAME=""
LIST_BACKUPS=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -f|--file)
            BACKUP_FILE="$2"
            shift 2
            ;;
        -a|--archive)
            ARCHIVE_FILE="$2"
            shift 2
            ;;
        -d|--database)
            DATABASE_NAME="$2"
            shift 2
            ;;
        -l|--list)
            LIST_BACKUPS=true
            shift
            ;;
        -h|--help)
            usage
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            usage
            ;;
    esac
done

# Check if kubectl is available
if ! command -v kubectl &> /dev/null; then
    echo -e "${RED}❌ kubectl not found. Please install kubectl first.${NC}"
    exit 1
fi

# Handle list command
if [ "$LIST_BACKUPS" = true ]; then
    list_backups
fi

# Check if at least one restore option is provided
if [ -z "$BACKUP_FILE" ] && [ -z "$ARCHIVE_FILE" ]; then
    echo -e "${RED}❌ No backup file or archive specified.${NC}"
    echo ""
    usage
fi

# Check if both options are provided
if [ -n "$BACKUP_FILE" ] && [ -n "$ARCHIVE_FILE" ]; then
    echo -e "${RED}❌ Cannot specify both --file and --archive options.${NC}"
    echo ""
    usage
fi

echo "🔄 Starting PostgreSQL restore..."
echo ""

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
if ! kubectl wait --for=condition=ready pod/"$POD_NAME" -n "$NAMESPACE" --timeout=30s &> /dev/null; then
    echo -e "${YELLOW}⚠️  PostgreSQL pod is not ready. Restore may fail.${NC}"
fi

# Get database credentials from secrets
echo "🔐 Retrieving database credentials..."
POSTGRES_USER=$(kubectl get secret brewget-secrets -n "$NAMESPACE" -o jsonpath='{.data.postgres-user}' | base64 -d)
POSTGRES_PASSWORD=$(kubectl get secret brewget-secrets -n "$NAMESPACE" -o jsonpath='{.data.postgres-password}' | base64 -d)

if [ -z "$POSTGRES_USER" ] || [ -z "$POSTGRES_PASSWORD" ]; then
    echo -e "${RED}❌ Failed to retrieve database credentials from secrets.${NC}"
    exit 1
fi

echo ""

# Perform restore
if [ -n "$BACKUP_FILE" ]; then
    restore_from_file "$BACKUP_FILE" "$DATABASE_NAME"
elif [ -n "$ARCHIVE_FILE" ]; then
    restore_from_archive "$ARCHIVE_FILE"
fi

echo ""
echo -e "${GREEN}✅ Restore operation completed!${NC}"
