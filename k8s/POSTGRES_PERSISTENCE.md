# PostgreSQL Data Persistence in Minikube

This guide explains how PostgreSQL data persistence works in the Brewget Kubernetes deployment, especially for minikube.

## Overview

PostgreSQL data is persisted using a hostPath PersistentVolume that stores data on the minikube node. This ensures data survives:
- ✅ `minikube stop` and `minikube start`
- ✅ Pod restarts and crashes
- ✅ Kubernetes redeployments

However, data is **NOT** preserved when:
- ❌ `minikube delete` is executed

## How It Works

### PersistentVolume with hostPath

The PostgreSQL deployment uses a PersistentVolume with hostPath:

```yaml
apiVersion: v1
kind: PersistentVolume
metadata:
  name: brewget-postgres-pv
spec:
  storageClassName: manual
  capacity:
    storage: 2Gi
  accessModes:
    - ReadWriteOnce
  hostPath:
    path: "/data/brewget-postgres"
    type: DirectoryOrCreate
  persistentVolumeReclaimPolicy: Retain
```

The path `/data/brewget-postgres` is created on the minikube VM and persists across stop/start cycles.

## Testing Data Persistence

### Test 1: Verify Data Persists After Minikube Stop

1. **Start minikube and deploy:**
   ```bash
   minikube start
   cd k8s
   ./deploy.sh
   ```

2. **Insert test data:**
   ```bash
   # Wait for postgres to be ready
   kubectl wait --for=condition=ready pod/postgres-0 -n brewget --timeout=120s
   
   # Get postgres username
   POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)
   
   # Create a test table and insert data
   kubectl exec -it postgres-0 -n brewget -- psql -U $POSTGRES_USER -d brewget_auth -c "
   CREATE TABLE IF NOT EXISTS test_persistence (
     id SERIAL PRIMARY KEY,
     message TEXT,
     created_at TIMESTAMP DEFAULT NOW()
   );
   INSERT INTO test_persistence (message) VALUES ('Data before minikube stop');
   "
   
   # Verify data
   kubectl exec -it postgres-0 -n brewget -- psql -U $POSTGRES_USER -d brewget_auth -c "
   SELECT * FROM test_persistence;
   "
   ```

3. **Stop minikube:**
   ```bash
   minikube stop
   ```

4. **Start minikube again:**
   ```bash
   minikube start
   ```

5. **Verify data still exists:**
   ```bash
   # Wait for postgres to be ready
   kubectl wait --for=condition=ready pod/postgres-0 -n brewget --timeout=120s
   
   # Check if data is still there
   POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)
   kubectl exec -it postgres-0 -n brewget -- psql -U $POSTGRES_USER -d brewget_auth -c "
   SELECT * FROM test_persistence;
   "
   ```

   ✅ **Expected Result**: The test data should still be present.

### Test 2: Backup and Restore

1. **Create a backup:**
   ```bash
   cd k8s
   ./backup-postgres.sh
   ```

   This creates backups in `./postgres-backups/` directory.

2. **Simulate data loss (optional):**
   ```bash
   # Delete and recreate the cluster
   minikube delete
   minikube start
   cd k8s
   ./deploy.sh
   ```

3. **Restore from backup:**
   ```bash
   # Wait for postgres to be ready
   kubectl wait --for=condition=ready pod/postgres-0 -n brewget --timeout=120s
   
   # List available backups
   ./restore-postgres.sh --list
   
   # Restore from the most recent archive
   ./restore-postgres.sh --archive ./postgres-backups/brewget_postgres_backup_YYYYMMDD_HHMMSS.tar.gz
   ```

   Replace `YYYYMMDD_HHMMSS` with the actual timestamp from your backup.

4. **Verify restored data:**
   ```bash
   POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)
   kubectl exec -it postgres-0 -n brewget -- psql -U $POSTGRES_USER -d brewget_auth -c "
   SELECT * FROM test_persistence;
   "
   ```

   ✅ **Expected Result**: The restored data should be present.

## Best Practices

### 1. Regular Backups

Schedule regular backups using cron:

```bash
# Add to crontab (run every day at 2 AM)
0 2 * * * cd /path/to/Brewget/k8s && ./auto-backup-postgres.sh >> /var/log/brewget-backup.log 2>&1
```

### 2. Backup Before Minikube Delete

Always backup before deleting minikube:

```bash
cd k8s
./backup-postgres.sh
minikube delete
```

### 3. Keep Backups on Host Machine

Backups are stored on your actual host machine (not in minikube), providing safety against:
- Minikube cluster deletion
- VM corruption
- Accidental data loss

### 4. Test Your Backups

Regularly test that backups can be restored:

```bash
cd k8s
./restore-postgres.sh --list
./restore-postgres.sh --archive ./postgres-backups/brewget_postgres_backup_LATEST.tar.gz
```

## Troubleshooting

### Issue: Data Lost After Minikube Stop

**Solution**: Check PersistentVolume status:

```bash
kubectl get pv
kubectl get pvc -n brewget
kubectl describe pv brewget-postgres-pv
```

Ensure the PV has `Retain` reclaim policy and is bound to the PVC.

### Issue: Backup Script Fails

**Possible causes:**
1. PostgreSQL pod not running: `kubectl get pods -n brewget`
2. Pod not ready: `kubectl wait --for=condition=ready pod/postgres-0 -n brewget --timeout=30s`
3. Wrong credentials: Verify secrets with `kubectl get secret brewget-secrets -n brewget -o yaml`

**Solution**: Ensure the PostgreSQL pod is running and ready before backing up.

### Issue: Restore Script Fails

**Possible causes:**
1. Backup file not found: Check path and filename
2. Database doesn't exist: The restore script expects databases to be already created
3. Connection issues: Verify pod is ready

**Solution**: 
```bash
# Ensure databases exist
POSTGRES_USER=$(kubectl get secret brewget-secrets -n brewget -o jsonpath='{.data.postgres-user}' | base64 -d)
kubectl exec -it postgres-0 -n brewget -- psql -U $POSTGRES_USER -l
```

### Issue: PersistentVolume Not Binding

**Solution**: Check if another PV is already using the hostPath:

```bash
kubectl get pv
minikube ssh "ls -la /data/brewget-postgres"
```

If needed, clean up old PV:

```bash
kubectl delete pv brewget-postgres-pv
kubectl apply -f k8s/04-postgres.yaml
```

## Data Location

### On Minikube Node

Data is stored at `/data/brewget-postgres` on the minikube VM:

```bash
# SSH into minikube to inspect
minikube ssh

# Inside minikube VM
sudo ls -la /data/brewget-postgres
```

### On Host Machine (Backups)

Backups are stored in the `k8s/postgres-backups/` directory on your host machine, outside of minikube.

## Migration to Production

When moving to a production Kubernetes cluster:

1. **Use cloud-provider storage classes** instead of hostPath
2. **Enable automated backups** to cloud storage (S3, GCS, etc.)
3. **Set up monitoring** for backup success/failure
4. **Implement point-in-time recovery** if needed
5. **Consider PostgreSQL operators** for advanced features (replication, failover)

For production, replace the hostPath PV with a cloud provider's PersistentVolume (e.g., AWS EBS, GCP Persistent Disk, Azure Disk).
