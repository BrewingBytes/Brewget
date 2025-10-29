# PostgreSQL Backup and Restore Solution Architecture

## Overview

This solution ensures PostgreSQL data persists across minikube restarts by implementing an automatic backup and restore system.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Minikube Environment                         │
│                                                                       │
│  ┌──────────────────────────────────────────────────────────────┐  │
│  │                    BrewGet Namespace                          │  │
│  │                                                                │  │
│  │  ┌────────────────────┐         ┌───────────────────────┐   │  │
│  │  │  PostgreSQL Pod    │         │  Backup CronJob       │   │  │
│  │  │  (StatefulSet)     │         │  (Every 6 hours)      │   │  │
│  │  │                    │         │                       │   │  │
│  │  │  ┌──────────────┐  │         │  Executes:           │   │  │
│  │  │  │ Init Script  │  │         │  - pg_dump auth      │   │  │
│  │  │  │ (Auto Restore)│  │         │  - pg_dump settings  │   │  │
│  │  │  └──────────────┘  │         │  - Keep last 10      │   │  │
│  │  │        │            │         │                       │   │  │
│  │  │        ▼            │         └───────────┬───────────┘   │  │
│  │  │  ┌──────────────┐  │                     │               │  │
│  │  │  │  PostgreSQL  │  │                     │               │  │
│  │  │  │   Server     │  │◄────────────────────┘               │  │
│  │  │  │  port: 5432  │  │         Writes                      │  │
│  │  │  └──────────────┘  │                                     │  │
│  │  │        │            │                                     │  │
│  │  └────────┼────────────┘                                     │  │
│  │           │                                                   │  │
│  │           │ Mounts                                           │  │
│  │           ▼                                                   │  │
│  │  ┌────────────────────┐                                     │  │
│  │  │  postgres-data PVC │                                     │  │
│  │  │   (1Gi - Ephemeral)│                                     │  │
│  │  └────────────────────┘                                     │  │
│  │           │                                                   │  │
│  │           │ Also Mounts                                      │  │
│  │           ▼                                                   │  │
│  │  ┌─────────────────────────────────────────────┐            │  │
│  │  │  postgres-backup-pvc (2Gi)                  │            │  │
│  │  │  Mounted at: /backup                        │            │  │
│  │  └─────────────────────────────────────────────┘            │  │
│  │           │                                                   │  │
│  └───────────┼───────────────────────────────────────────────────┘  │
│              │                                                       │
│              │ Backed by                                            │
│              ▼                                                       │
│  ┌─────────────────────────────────────────────┐                   │
│  │  postgres-backup-pv (hostPath)               │                   │
│  │  Storage Class: manual                       │                   │
│  └─────────────────────────────────────────────┘                   │
│              │                                                       │
└──────────────┼───────────────────────────────────────────────────────┘
               │
               ▼
┌──────────────────────────────────────────────────────────────────────┐
│                      Host Filesystem (Persistent)                     │
│                                                                        │
│  /mnt/data/brewget-postgres-backups/                                 │
│  ├── backup_20241029_120000/                                         │
│  │   ├── postgres.sql                                                │
│  │   ├── brewget_auth.sql                                            │
│  │   └── brewget_settings.sql                                        │
│  ├── backup_20241029_180000/                                         │
│  │   ├── postgres.sql                                                │
│  │   ├── brewget_auth.sql                                            │
│  │   └── brewget_settings.sql                                        │
│  └── latest -> backup_20241029_180000/  (symlink)                    │
│                                                                        │
│  ⚠️  This directory persists across:                                  │
│     - minikube stop/start                                             │
│     - namespace deletion                                              │
│     - pod restarts                                                    │
└──────────────────────────────────────────────────────────────────────┘
```

## Component Breakdown

### 1. PostgreSQL StatefulSet (`k8s/04-postgres.yaml`)
- **Primary database server**
- Runs PostgreSQL 16-alpine
- Mounts:
  - `/var/lib/postgresql/data` - Main data directory (ephemeral)
  - `/backup` - Backup storage (persistent)
  - `/backup-scripts` - Backup/restore scripts from ConfigMap
  - `/docker-entrypoint-initdb.d` - Init scripts

### 2. Backup PersistentVolume (`k8s/04-postgres-backup.yaml`)
- **Type**: hostPath (for minikube)
- **Path**: `/mnt/data/brewget-postgres-backups`
- **Size**: 2Gi
- **Storage Class**: manual
- **Key Feature**: Survives minikube restarts

### 3. Backup Scripts ConfigMap (`k8s/04-postgres-backup.yaml`)
- **backup.sh**: Creates backups of all databases
  - Uses `pg_dump` to export each database
  - Creates timestamped directories
  - Maintains `latest` symlink
  - Keeps last 10 backups
  
- **restore.sh**: Restores from latest backup
  - Only restores to empty databases
  - Checks table count before restore
  - Safe against accidental data loss

### 4. Init Script (`k8s/03-configmaps.yaml`)
- **init-databases.sh**: Runs during PostgreSQL startup
  - Creates required databases (brewget_auth, brewget_settings)
  - Checks for existing backups in `/backup/latest`
  - Automatically restores if databases are empty
  - Logs all operations

### 5. Backup CronJob (`k8s/04-postgres-backup.yaml`)
- **Schedule**: Every 6 hours (`0 */6 * * *`)
- **Action**: Executes backup.sh script
- **Retention**: Last 3 successful jobs, 3 failed jobs
- **Environment**: Uses same credentials as PostgreSQL pod

### 6. Manual Scripts
- **`backup-postgres.sh`**: Trigger manual backup
- **`restore-postgres.sh`**: Trigger manual restore

## Data Flow

### Backup Flow
```
1. CronJob triggers (every 6 hours)
2. Creates Job pod with PostgreSQL client
3. Connects to PostgreSQL service (db.brewget.svc.cluster.local)
4. Runs pg_dump for each database
5. Saves to /backup/backup_TIMESTAMP/
6. Updates /backup/latest symlink
7. Cleans up old backups (keeps last 10)
```

### Restore Flow (Automatic)
```
1. PostgreSQL pod starts
2. Init script (init-databases.sh) runs
3. Creates databases if they don't exist
4. Checks if /backup/latest exists
5. Checks if databases are empty (no tables)
6. If empty and backup exists:
   - Restores brewget_auth.sql
   - Restores brewget_settings.sql
7. PostgreSQL server starts normally
```

### Minikube Restart Scenario
```
1. User runs: minikube stop
2. All pods terminate
3. PVCs are deleted
4. Backup directory (/mnt/data/brewget-postgres-backups) persists on host

5. User runs: minikube start
6. User deploys BrewGet: ./deploy.sh
7. PostgreSQL pod starts
8. Init script finds backup in /backup/latest
9. Databases are empty (fresh deployment)
10. Init script restores from backup
11. ✅ Data is recovered!
```

## Key Design Decisions

### 1. hostPath for Minikube
- **Why**: Simple, works out-of-box with minikube
- **Trade-off**: Not suitable for multi-node clusters
- **Alternative for production**: NFS, Cloud Storage (EBS, GCS, Azure Disk)

### 2. Init Script Auto-Restore
- **Why**: Zero-touch recovery
- **Safety**: Only restores to empty databases
- **Benefit**: Users don't need to manually restore

### 3. CronJob Frequency (6 hours)
- **Why**: Balance between data freshness and resource usage
- **Configurable**: Users can easily change schedule
- **Trade-off**: Up to 6 hours of data could be lost

### 4. Backup Retention (10 backups)
- **Why**: Balance between storage and recovery options
- **Configurable**: Can be changed in backup.sh
- **Benefit**: Multiple restore points available

### 5. SQL Dump Format
- **Why**: Simple, portable, human-readable
- **Trade-off**: Slower than binary formats for large databases
- **Benefit**: Easy to inspect and modify if needed

## Security Considerations

1. **Credentials**: Pulled from Kubernetes Secrets
2. **PGPASSWORD**: Set via environment variable in CronJob
3. **File Permissions**: Scripts have 0755 permissions
4. **Network**: Backup job connects via Kubernetes service DNS
5. **Access Control**: Namespace isolation, RBAC applies

## Scalability Considerations

### Current Design
- ✅ Works for single-node minikube
- ✅ Works for small databases (<100MB)
- ✅ Works for infrequent changes

### For Production/Scale
- Use PostgreSQL streaming replication for HA
- Use pg_basebackup for large databases
- Use WAL archiving for point-in-time recovery
- Use cloud-native backup solutions (Velero, Stash)
- Consider managed PostgreSQL services (RDS, Cloud SQL)

## Monitoring and Observability

### Check Backup Status
```bash
kubectl get cronjob postgres-backup -n brewget
kubectl get jobs -n brewget | grep backup
```

### View Backup Logs
```bash
kubectl logs -n brewget job/postgres-backup-xxxxx
```

### Check Available Backups
```bash
kubectl exec postgres-0 -n brewget -- ls -lh /backup/
```

### Verify PostgreSQL Logs
```bash
kubectl logs postgres-0 -n brewget | grep -i "restore\|backup"
```

## Testing Strategy

See `BACKUP_TESTING.md` for comprehensive testing guide covering:
- Data persistence across minikube restarts
- Complete minikube deletion scenarios
- Manual backup and restore operations
- Troubleshooting procedures

## Future Enhancements

1. **Encryption**: Encrypt backups at rest
2. **Compression**: Compress backup files to save space
3. **Remote Storage**: Support S3/GCS for cloud deployments
4. **Differential Backups**: Only backup changes since last backup
5. **Notifications**: Alert on backup failures
6. **Metrics**: Expose backup metrics to Prometheus
7. **Web UI**: Dashboard for backup management
8. **Automated Testing**: CI/CD pipeline to test backup/restore

## Conclusion

This solution provides a robust, automated backup and restore system for PostgreSQL in minikube environments. It ensures data persists across minikube restarts with minimal user intervention while maintaining safety through smart restore logic.
