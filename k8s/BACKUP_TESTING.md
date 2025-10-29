# PostgreSQL Backup and Restore Testing Guide

This document provides step-by-step instructions to test the automatic PostgreSQL backup and restore functionality for minikube.

## Test Scenario: Data Persistence Across Minikube Restarts

### Prerequisites
- minikube installed and running
- kubectl configured
- BrewGet deployed to minikube

### Test Steps

#### 1. Deploy BrewGet and Create Test Data

```bash
# Start minikube (if not running)
minikube start

# Deploy BrewGet
cd k8s
./deploy.sh

# Wait for all pods to be ready
kubectl wait --for=condition=ready pod --all -n brewget --timeout=300s

# Get PostgreSQL username
POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)

# Connect to PostgreSQL and create test data
kubectl exec -it postgres-0 -n brewget -- psql -U "$POSTGRES_USER" -d brewget_auth -c "
  CREATE TABLE IF NOT EXISTS test_data (
    id SERIAL PRIMARY KEY,
    message TEXT,
    created_at TIMESTAMP DEFAULT NOW()
  );
  INSERT INTO test_data (message) VALUES ('Data before minikube stop');
"

# Verify data exists
kubectl exec -it postgres-0 -n brewget -- psql -U "$POSTGRES_USER" -d brewget_auth -c "SELECT * FROM test_data;"
```

Expected output: You should see the test record.

#### 2. Create Manual Backup

```bash
# Create a manual backup
cd k8s
./backup-postgres.sh

# Verify backup was created
kubectl exec postgres-0 -n brewget -- ls -lh /backup/
```

Expected output: You should see a `backup_YYYYMMDD_HHMMSS` directory and a `latest` symlink.

#### 3. Stop Minikube and Delete the Namespace

```bash
# Delete the BrewGet namespace (this removes all pods and their data volumes)
kubectl delete namespace brewget

# Verify namespace is gone
kubectl get namespace brewget
# Expected: Error from server (NotFound)

# Stop minikube
minikube stop
```

#### 4. Restart Minikube and Redeploy

```bash
# Start minikube again
minikube start

# Verify backup directory still exists on the host
minikube ssh
ls -la /mnt/data/brewget-postgres-backups/
exit

# Redeploy BrewGet
cd k8s
./deploy.sh

# Wait for all pods to be ready
kubectl wait --for=condition=ready pod --all -n brewget --timeout=300s
```

#### 5. Verify Data Was Restored

```bash
# Get PostgreSQL username
POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)

# Check if test data was restored
kubectl exec -it postgres-0 -n brewget -- psql -U "$POSTGRES_USER" -d brewget_auth -c "SELECT * FROM test_data;"
```

**Expected Result**: ✅ You should see the test record "Data before minikube stop" that you created in step 1!

#### 6. Verify Automatic Backups Are Running

```bash
# Check CronJob status
kubectl get cronjob postgres-backup -n brewget

# View CronJob details
kubectl describe cronjob postgres-backup -n brewget

# Check for completed backup jobs
kubectl get jobs -n brewget | grep backup

# Manually trigger a backup job
kubectl create job --from=cronjob/postgres-backup manual-test-backup -n brewget

# Watch the job
kubectl get job manual-test-backup -n brewget --watch

# View job logs
kubectl logs -n brewget job/manual-test-backup
```

## Test Scenario 2: Complete Minikube Deletion

### Steps

```bash
# Stop and delete minikube completely
minikube stop
minikube delete

# Start fresh minikube
minikube start

# SSH into minikube and check if backup directory exists
minikube ssh
ls -la /mnt/data/brewget-postgres-backups/
# Note: Directory may not exist after complete deletion, depending on minikube version

# If directory doesn't exist, you can upload a backup
exit

# Deploy BrewGet
cd k8s
./deploy.sh

# If you have a backup on your local machine, upload it:
# 1. Wait for postgres pod to be ready
kubectl wait --for=condition=ready pod postgres-0 -n brewget --timeout=300s

# 2. Upload backup
kubectl cp ./your-local-backup-directory brewget/postgres-0:/backup/manual

# 3. Create latest symlink
kubectl exec postgres-0 -n brewget -- ln -sfn /backup/manual /backup/latest

# 4. Restart postgres to trigger restore
kubectl delete pod postgres-0 -n brewget
kubectl wait --for=condition=ready pod postgres-0 -n brewget --timeout=300s
```

## Test Scenario 3: Manual Backup and Restore

### Creating Manual Backup

```bash
# Create a manual backup using the script
cd k8s
./backup-postgres.sh

# Or manually trigger from kubectl
kubectl exec postgres-0 -n brewget -- /bin/sh /backup-scripts/backup.sh

# Download backup to local machine
kubectl cp brewget/postgres-0:/backup/latest ./local-backup-$(date +%Y%m%d)

# List contents
ls -lh ./local-backup-$(date +%Y%m%d)/
```

### Restoring from Backup

```bash
# Restore using the script
cd k8s
./restore-postgres.sh

# Or manually
kubectl exec postgres-0 -n brewget -- /bin/sh /backup-scripts/restore.sh
```

## Troubleshooting

### Check if PostgreSQL has data

```bash
POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)

# Check brewget_auth database
kubectl exec postgres-0 -n brewget -- psql -U "$POSTGRES_USER" -d brewget_auth -c "\dt"

# Check brewget_settings database
kubectl exec postgres-0 -n brewget -- psql -U "$POSTGRES_USER" -d brewget_settings -c "\dt"
```

### View PostgreSQL logs

```bash
kubectl logs postgres-0 -n brewget | grep -i "restore\|backup"
```

### Check backup CronJob logs

```bash
# Get the latest backup job
LATEST_JOB=$(kubectl get jobs -n brewget --sort-by=.metadata.creationTimestamp -o name | grep postgres-backup | tail -1)

# View logs
kubectl logs -n brewget $LATEST_JOB
```

### Verify backup volume is mounted

```bash
kubectl exec postgres-0 -n brewget -- df -h | grep backup
kubectl exec postgres-0 -n brewget -- mount | grep backup
```

## Expected Behavior

1. **Fresh Deployment**: When deploying to a fresh minikube with no backups, PostgreSQL starts with empty databases
2. **With Existing Backup**: When deploying and a backup exists in `/backup/latest`, PostgreSQL automatically restores from it
3. **Automatic Backups**: Every 6 hours, a CronJob creates a new backup
4. **Manual Backup**: Can be triggered anytime using `./backup-postgres.sh`
5. **Manual Restore**: Can be triggered using `./restore-postgres.sh` (only restores to empty databases)
6. **Minikube Restart**: Data persists across minikube stops/starts because backups are stored on the host
7. **Namespace Delete**: Even when you delete the namespace, backups remain on the host
8. **Complete Minikube Delete**: Backups may be lost when completely deleting minikube (depends on minikube version and storage driver)

## Notes

- Backups are stored in `/mnt/data/brewget-postgres-backups` inside the minikube VM
- The retention policy keeps the last 10 backups
- Restore only works on empty databases to prevent accidental data loss
- For production environments, consider using a different storage solution (NFS, cloud storage, etc.)
