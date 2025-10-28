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

## Documentation

- [Kubernetes Deployment Guide](k8s/README.md)
- [Database Migrations](backend/MIGRATIONS.md)
- [Separate Databases Configuration](backend/SEPARATE_DATABASES.md)
