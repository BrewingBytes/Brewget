# BrewGet

A microservices-based application for managing your budgets.

## Architecture

BrewGet consists of the following services:

- **frontend**: Vue.js web application
- **auth-service**: Authentication and authorization (Rust)
- **settings-service**: User settings management (Rust)
- **email-service**: Email notifications (Rust)
- **nginx**: Reverse proxy and API gateway
- **postgres**: PostgreSQL database

## Deployment

### Kubernetes

For production deployments, use Kubernetes:

```bash
cd k8s
./deploy.sh
```

See [k8s/README.md](k8s/README.md) for detailed instructions.

#### Data Persistence for Minikube

When using minikube, PostgreSQL data is automatically backed up and restored:
- **Automatic backups** run every 6 hours via CronJob
- **Backups persist** on the host filesystem even when minikube is stopped
- **Automatic restore** happens when PostgreSQL starts with empty databases

This means your data survives minikube restarts! See the [Backup and Restore documentation](k8s/README.md#automatic-backup-and-restore) for more details.

## Documentation

- [Kubernetes Deployment Guide](k8s/README.md)
- [Database Migrations](backend/MIGRATIONS.md)
- [Separate Databases Configuration](backend/SEPARATE_DATABASES.md)
