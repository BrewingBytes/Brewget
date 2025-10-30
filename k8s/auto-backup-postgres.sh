#!/bin/bash
# Automated PostgreSQL backup script with retention policy
# This script can be run as a cron job to automatically backup databases

set -e

# Configuration
NAMESPACE="brewget"
POD_NAME="postgres-0"
BACKUP_DIR="${BACKUP_DIR:-./postgres-backups}"
RETENTION_DAYS="${RETENTION_DAYS:-7}"  # Keep backups for 7 days by default

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "🔄 Starting automated PostgreSQL backup..."
echo ""

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Run the backup script
if "$SCRIPT_DIR/backup-postgres.sh"; then
    echo ""
    echo "🧹 Cleaning up old backups (older than $RETENTION_DAYS days)..."
    
    # Remove old backup files
    if [ -d "$BACKUP_DIR" ]; then
        # Find and delete SQL files older than retention period
        find "$BACKUP_DIR" -name "*.sql" -type f -mtime +$RETENTION_DAYS -delete 2>/dev/null || true
        
        # Find and delete archive files older than retention period
        find "$BACKUP_DIR" -name "*.tar.gz" -type f -mtime +$RETENTION_DAYS -delete 2>/dev/null || true
        
        echo -e "${GREEN}✅ Cleanup completed${NC}"
    fi
    
    echo ""
    echo -e "${GREEN}✅ Automated backup completed successfully!${NC}"
    exit 0
else
    echo ""
    echo -e "${RED}❌ Automated backup failed${NC}"
    exit 1
fi
