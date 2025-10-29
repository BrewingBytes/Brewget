# Local Development Setup

This guide will help you set up and run BrewGet locally with hot-reloading for all microservices using cargo-watch.

## Prerequisites

- **Rust** (latest stable version)
- **Node.js** (v20 or later) and npm
- **PostgreSQL** (v13 or later)
- **cargo-watch** (will be installed automatically by dev script)
- **protobuf-compiler** (libprotoc)
- Git

### Installing Prerequisites

**Rust:**
```bash
# Visit https://rustup.rs for the latest installation instructions
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Node.js:**
```bash
# Using nvm (recommended) - visit https://github.com/nvm-sh/nvm for latest version
# Verify the script before running or use your system's package manager
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
```

**PostgreSQL:**
```bash
# Ubuntu/Debian
sudo apt-get install postgresql postgresql-contrib

# macOS
brew install postgresql@16
brew services start postgresql@16

# Verify installation
psql --version
```

**protobuf-compiler:**
```bash
# Ubuntu/Debian
sudo apt-get install protobuf-compiler

# macOS
brew install protobuf

# Verify installation
protoc --version
```

## Infrastructure Setup

Before running the microservices, you need to set up the infrastructure components.

### 1. PostgreSQL Database

Create the databases needed by the services:

```bash
# Connect to PostgreSQL
psql -U postgres

# Create databases
CREATE DATABASE brewget_auth;
CREATE DATABASE brewget_settings;

# Create user (optional, if not using default postgres user)
CREATE USER brewget WITH PASSWORD 'brewget_dev_password';
GRANT ALL PRIVILEGES ON DATABASE brewget_auth TO brewget;
GRANT ALL PRIVILEGES ON DATABASE brewget_settings TO brewget;

# Exit psql
\q
```

### 2. (Optional) Nginx Reverse Proxy

If you want to run nginx as a reverse proxy (optional for development):

```bash
# Ubuntu/Debian
sudo apt-get install nginx

# macOS
brew install nginx

# Copy the nginx config
sudo cp nginx/nginx.conf /etc/nginx/nginx.conf
sudo cp nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf

# Start nginx
sudo systemctl start nginx  # Linux
brew services start nginx   # macOS
```

### 3. (Optional) Email Testing with MailHog

For testing email functionality:

```bash
# macOS
brew install mailhog
mailhog &

# Linux - download from official releases
# Visit https://github.com/mailhog/MailHog/releases for latest version
# Verify checksums after download
wget https://github.com/mailhog/MailHog/releases/download/v1.0.1/MailHog_linux_amd64
# Optional: verify checksum before running
chmod +x MailHog_linux_amd64
./MailHog_linux_amd64 &
```

Access MailHog UI at http://localhost:8025

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
   Edit `.env` to match your local setup (especially database connection strings).

3. **Install frontend dependencies**:
   ```bash
   cd frontend
   npm install
   cd ..
   ```

4. **Run database migrations**:
   ```bash
   # Install sqlx-cli if not already installed
   cargo install sqlx-cli --features postgres

   # Run migrations
   cd backend/auth-service
   sqlx migrate run
   cd ../settings-service
   sqlx migrate run
   cd ../..
   ```

5. **Start all services with watch mode**:
   ```bash
   ./dev.sh
   ```

   This script will:
   - Start all three Rust microservices with `cargo-watch` (automatic rebuild on changes)
   - Start the Vue.js frontend with Vite's hot-reloading

6. **Access the application**:
   - **Frontend**: http://localhost:5173
   - **Auth Service API**: http://localhost:8000
   - **Settings Service API**: http://localhost:8001
   - **MailHog (if running)**: http://localhost:8025

7. **Stop all services**:
   ```bash
   ./dev-stop.sh
   ```
   Or press `Ctrl+C` in the terminal running `./dev.sh`

## Development Workflow

### Watch Mode (Automatic Reloading)

The `./dev.sh` script uses:
- **cargo-watch** for Rust services - automatically rebuilds and restarts when source files change
- **Vite dev server** for frontend - hot module replacement (HMR) for instant updates

**Watched paths:**
- **auth-service**: `backend/auth-service/src`, `backend/shared-types/src`, `backend/proto`
- **email-service**: `backend/email-service/src`, `backend/proto`
- **settings-service**: `backend/settings-service/src`, `backend/shared-types/src`, `backend/proto`
- **frontend**: All files in `frontend/src`

### Manual Mode (Individual Services)

If you prefer to run services individually in separate terminals:

1. **Start Email Service** (terminal 1):
   ```bash
   cd backend
   cargo watch -w email-service/src -w proto -x "run --package email-service"
   ```

2. **Start Auth Service** (terminal 2):
   ```bash
   cd backend
   cargo watch -w auth-service/src -w shared-types/src -w proto -x "run --package auth-service"
   ```

3. **Start Settings Service** (terminal 3):
   ```bash
   cd backend
   cargo watch -w settings-service/src -w shared-types/src -w proto -x "run --package settings-service"
   ```

4. **Start Frontend** (terminal 4):
   ```bash
   cd frontend
   npm run dev
   ```

## Service Details

### Rust Microservices

The backend uses `cargo-watch` to automatically rebuild when source files change:

- **auth-service**: 
  - HTTP Port: 8000
  - gRPC Port: 9000
  - Database: brewget_auth
  
- **email-service**: 
  - gRPC Port: 9001
  
- **settings-service**: 
  - HTTP Port: 8001
  - Database: brewget_settings

### Frontend

The frontend uses Vite's built-in dev server with hot module replacement (HMR):

- **Port**: 5173
- **Hot-reload**: Automatic on file changes

## Environment Variables

The `.env` file contains all configuration. Key variables:

```bash
# Database connections
DATABASE_URL_AUTH=postgresql://brewget:brewget_dev_password@localhost:5432/brewget_auth
DATABASE_URL_SETTINGS=postgresql://brewget:brewget_dev_password@localhost:5432/brewget_settings

# Service ports
AUTH_HTTP_PORT=8000
AUTH_GRPC_PORT=9000
EMAIL_GRPC_PORT=9001
SETTINGS_HTTP_PORT=8001

# Service hostnames (for inter-service communication)
EMAIL_GRPC_HOSTNAME=localhost:9001
FRONTEND_HOSTNAME=http://localhost:5173
CORS_URL=http://localhost:5173

# JWT secret
JWT_SECRET=dev_jwt_secret_change_in_production

# SMTP (for email service)
SMTP_EMAIL=noreply@localhost
SMTP_RELAY=localhost:1025  # MailHog SMTP port

# Turnstile (Cloudflare captcha)
TURNSTILE_SECRET=1x0000000000000000000000000000000AA  # Test key
```

## Logs

When using `./dev.sh`, logs are written to `.dev-logs/` directory:

```bash
# View logs in real-time
tail -f .dev-logs/auth-service.log
tail -f .dev-logs/email-service.log
tail -f .dev-logs/settings-service.log
tail -f .dev-logs/frontend.log

# View all logs
tail -f .dev-logs/*.log
```

## Useful Commands

### Build and Test

```bash
# Build all services
cd backend
cargo build

# Build specific service
cargo build --package auth-service

# Run tests
cargo test

# Run tests for specific service
cargo test --package auth-service

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Database Management

```bash
# Install sqlx-cli (if not already installed)
cargo install sqlx-cli --features postgres

# Run migrations
cd backend/auth-service
sqlx migrate run

cd backend/settings-service
sqlx migrate run

# Create new migration
cd backend/auth-service
sqlx migrate add <migration_name>

# Revert last migration
sqlx migrate revert

# Access database directly
psql -U brewget -d brewget_auth
psql -U brewget -d brewget_settings
```

### Frontend Commands

```bash
cd frontend

# Install dependencies
npm install

# Run dev server
npm run dev

# Build for production
npm run build

# Lint and format
npm run lint
npm run format

# Type check
npm run type-check
```

## Troubleshooting

### Port already in use

If you get port conflicts, check which processes are using the ports:

```bash
# Check ports
lsof -i :5173 -i :8000 -i :8001 -i :9000 -i :9001

# Kill specific process
kill -9 <PID>
```

### cargo-watch not found

Install cargo-watch manually:
```bash
cargo install cargo-watch
```

### Database connection issues

1. **Check PostgreSQL is running**:
   ```bash
   # Linux
   sudo systemctl status postgresql
   
   # macOS
   brew services list | grep postgresql
   ```

2. **Verify databases exist**:
   ```bash
   psql -U postgres -c "\l" | grep brewget
   ```

3. **Test connection**:
   ```bash
   psql -U brewget -d brewget_auth -c "SELECT 1;"
   ```

4. **Check environment variables**:
   Ensure `DATABASE_URL_AUTH` and `DATABASE_URL_SETTINGS` in `.env` match your PostgreSQL setup.

### protobuf-compiler not found

The Rust services require protobuf-compiler to build:

```bash
# Ubuntu/Debian
sudo apt-get install protobuf-compiler

# macOS
brew install protobuf

# Verify
protoc --version
```

### Frontend not hot-reloading

1. **Check if Vite is running**:
   ```bash
   tail -f .dev-logs/frontend.log
   ```

2. **Check for errors in browser console**

3. **Restart frontend**:
   ```bash
   # If using dev.sh, stop and restart
   ./dev-stop.sh
   ./dev.sh

   # If running manually
   cd frontend
   npm run dev
   ```

### Services not restarting on code changes

1. **Verify cargo-watch is installed**:
   ```bash
   cargo watch --version
   ```

2. **Check logs for errors**:
   ```bash
   tail -f .dev-logs/auth-service.log
   ```

3. **Manually restart the service**:
   ```bash
   # Find and kill the process
   ps aux | grep "cargo watch"
   kill <PID>
   
   # Start it again
   cd backend
   cargo watch -w auth-service/src -w shared-types/src -w proto -x "run --package auth-service"
   ```

### Build errors

1. **Clean build cache**:
   ```bash
   cd backend
   cargo clean
   cargo build
   ```

2. **Update dependencies**:
   ```bash
   cargo update
   ```

3. **Check Rust version**:
   ```bash
   rustc --version
   # Should be 1.90 or compatible
   ```

## Architecture

```
┌─────────────────┐       ┌──────────────────────┐
│   Frontend      │       │   Backend Services   │
│   (Vue + Vite)  │       │   (Cargo Watch)      │
│   Port 5173     │       │                      │
│   npm run dev   │       │  • Auth (8000/9000)  │
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
