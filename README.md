# BrewGet

A microservices-based application for managing your budgets.

## Architecture

BrewGet consists of the following services:

- **frontend**: Vue.js web application
- **auth-service**: Authentication and authorization (Rust)
- **settings-service**: User settings management (Rust)
- **transaction-service**: Transaction and wallet management (Rust)
- **email-service**: Email notifications (Rust)
- **nginx**: Reverse proxy and API gateway
- **postgres**: PostgreSQL database

## Local Development

For local development with hot-reloading:

1. Set up infrastructure (PostgreSQL, etc.) - see [DEVELOPMENT.md](DEVELOPMENT.md) for details
2. Install overmind: `brew install overmind` (macOS) or see [DEVELOPMENT.md](DEVELOPMENT.md)
3. Copy environment file: `cp .env.example .env`
4. Run services: `overmind start`

Overmind manages all services with automatic rebuilding and displays unified logs with restart notifications.

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed local development instructions.

## Deployment

### Kubernetes

For production deployments, use Kubernetes:

```bash
cd k8s
./deploy.sh
```

See [k8s/README.md](k8s/README.md) for detailed instructions.

## Documentation

- [Local Development Guide](DEVELOPMENT.md)
- [Kubernetes Deployment Guide](k8s/README.md)

## Changelogs

- [Frontend Changelog](changelogs/frontend-CHANGELOG.md)
- [Auth Service Changelog](changelogs/auth-service-CHANGELOG.md)
- [Settings Service Changelog](changelogs/settings-service-CHANGELOG.md)
- [Transaction Service Changelog](changelogs/transaction-service-CHANGELOG.md)
- [Email Service Changelog](changelogs/email-service-CHANGELOG.md)
- [Infrastructure Changelog](changelogs/infrastructure-CHANGELOG.md)
