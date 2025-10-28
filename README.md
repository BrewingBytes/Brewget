# BrewGet

A microservices-based application for managing brew-related tasks.

## Architecture

BrewGet consists of the following services:

- **frontend**: Vue.js web application
- **auth-service**: Authentication and authorization (Rust)
- **settings-service**: User settings management (Rust)
- **email-service**: Email notifications (Rust)
- **nginx**: Reverse proxy and API gateway
- **postgres**: PostgreSQL database

## Deployment

### Kubernetes (Recommended)

For production deployments, use Kubernetes:

```bash
cd k8s
./deploy.sh
```

See [k8s/README.md](k8s/README.md) for detailed instructions.

### Docker Compose (Development)

For local development, use Docker Compose:

```bash
docker-compose up
```

## Documentation

- [Kubernetes Deployment Guide](k8s/README.md)
- [Database Migrations](backend/MIGRATIONS.md)
- [Separate Databases Configuration](backend/SEPARATE_DATABASES.md)
