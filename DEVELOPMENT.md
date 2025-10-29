# Local Development Setup

This guide will help you set up and run BrewGet locally with hot-reloading for all microservices.

## Prerequisites

- Docker and Docker Compose (with Compose v2.22+ for watch mode support)
- Git

## Quick Start

1. **Clone the repository**:
   ```bash
   git clone https://github.com/BrewingBytes/Brewget.git
   cd Brewget
   ```

2. **Create environment file**:
   ```bash
   cp .env.example .env
   ```
   Edit `.env` if you need to customize any settings.

3. **Start all services with watch mode**:
   ```bash
   docker compose watch
   ```

   This will:
   - Start PostgreSQL database
   - Start all three Rust microservices (auth, email, settings) with automatic rebuilding on code changes
   - Start the Vue.js frontend with Vite's hot-reloading
   - Start nginx as a reverse proxy
   - Start MailHog for email testing

4. **Access the application**:
   - **Application**: http://localhost
   - **Frontend (direct)**: http://localhost:5173
   - **Auth Service API**: http://localhost:8000
   - **Settings Service API**: http://localhost:8001
   - **MailHog (Email UI)**: http://localhost:8025

## Development Workflow

### Watch Mode (Recommended)

The `docker compose watch` command provides automatic rebuilding when you make changes:

- **Rust services**: Changes to `src/` directories trigger automatic rebuild and restart
- **Frontend**: Changes are hot-reloaded instantly via Vite

### Standard Mode

If you prefer to run without watch mode:

```bash
docker compose up
```

To rebuild after making changes:
```bash
docker compose up --build
```

## Service Details

### Rust Microservices

The backend uses `cargo-watch` to automatically rebuild when source files change:

- **auth-service**: Port 8000 (HTTP), 9000 (gRPC)
- **email-service**: Port 9001 (gRPC)
- **settings-service**: Port 8001 (HTTP)

**Watch paths**:
- Service-specific `src/` directory
- `shared-types/src` (for shared code)
- `proto/` (for gRPC definitions)

### Frontend

The frontend uses Vite's built-in dev server with hot module replacement (HMR):

- **Port**: 5173
- **Hot-reload**: Automatic on file changes

### Database

PostgreSQL runs on port 5432 with two databases:
- `brewget_auth`: Authentication data
- `brewget_settings`: Settings data

### Email Testing

MailHog captures all emails sent by the application:
- **SMTP**: Port 1025
- **Web UI**: http://localhost:8025

## Useful Commands

### View logs
```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f auth-service
docker compose logs -f frontend
```

### Restart a service
```bash
docker compose restart auth-service
```

### Stop all services
```bash
docker compose down
```

### Stop and remove volumes (clean slate)
```bash
docker compose down -v
```

### Run database migrations
```bash
# Auth service migrations
docker compose exec auth-service sqlx migrate run

# Settings service migrations
docker compose exec settings-service sqlx migrate run
```

### Access database directly
```bash
docker compose exec postgres psql -U brewget -d brewget_auth
```

## Troubleshooting

### Services fail to start

1. **Check if ports are already in use**:
   ```bash
   # Check port availability
   lsof -i :80 -i :5173 -i :8000 -i :8001 -i :9000 -i :9001
   ```

2. **View service logs**:
   ```bash
   docker compose logs <service-name>
   ```

3. **Rebuild containers**:
   ```bash
   docker compose down
   docker compose build --no-cache
   docker compose up
   ```

### Database connection issues

1. **Wait for PostgreSQL to be ready**: The services have health checks but may need a few seconds.

2. **Check database logs**:
   ```bash
   docker compose logs postgres
   ```

3. **Reset database**:
   ```bash
   docker compose down -v
   docker compose up -d postgres
   # Wait for postgres to be ready, then start other services
   docker compose up
   ```

### Watch mode not working

Ensure you have Docker Compose v2.22 or later:
```bash
docker compose version
```

If your version is older, you can still use standard mode:
```bash
docker compose up --build
```

### Frontend not hot-reloading

Check that the frontend service is running:
```bash
docker compose ps frontend
docker compose logs frontend
```

## Architecture

```
┌─────────────────────────────────────────────────┐
│                   nginx (Port 80)               │
│                  Reverse Proxy                  │
└────────┬─────────────────────────┬──────────────┘
         │                         │
         ▼                         ▼
┌─────────────────┐       ┌──────────────────────┐
│   Frontend      │       │   Backend Services   │
│   (Vue + Vite)  │       │                      │
│   Port 5173     │       │  • Auth (8000/9000)  │
│                 │       │  • Settings (8001)   │
│                 │       │  • Email (9001)      │
└─────────────────┘       └──────────┬───────────┘
                                     │
                          ┌──────────▼───────────┐
                          │   PostgreSQL (5432)  │
                          │  • brewget_auth      │
                          │  • brewget_settings  │
                          └──────────────────────┘
```

## Next Steps

- Check out the [Kubernetes Deployment Guide](k8s/README.md) for production deployments
- Review [Database Migrations](backend/MIGRATIONS.md) for managing schema changes
- See [Separate Databases Configuration](backend/SEPARATE_DATABASES.md) for database setup details
