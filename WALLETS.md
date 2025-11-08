# Wallets Feature - Implementation Summary

## Overview
The Wallets feature allows users to create and manage multiple wallets with independent currencies for organizing budgets and transactions. This feature is implemented as a dedicated microservice called **transaction-service**.

## Key Features
- ✅ Create, Read, Update, Delete wallets
- ✅ Multiple currency support (USD, EUR, RON)
- ✅ Wallet types (General, Savings, Business, Personal)
- ✅ User authentication and authorization
- ✅ Multi-language support (en, es, fr, de, ro)
- ✅ Responsive UI with glassmorphism design
- ✅ Dedicated microservice architecture

## Implementation

### Backend (transaction-service)
**Location:** `backend/transaction-service/`
- Main: `src/main.rs`
- Database migration: `migrations/20251108000000_create_wallets.up.sql`
- Models: `src/wallet_model.rs`
- Database ops: `src/wallet_db.rs`
- Routes: `src/wallet_routes.rs`
- Auth middleware: `src/auth_guard.rs`
- Enums: `backend/shared-types/src/enums.rs` (Currency, WalletType)

### Frontend
**Location:** `frontend/src/`
- View: `views/WalletsView.vue`
- Service: `services/wallet.ts`
- Router: Updated `router/index.ts`
- Translations: `locales/{en,es,fr,de,ro}.json`
- API: `services/api.ts` (walletApi instance pointing to port 8003)

## API Endpoints
Base URL: `http://localhost:8003` (dev) or `/api/wallets` (prod)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | `/`      | Get all user wallets |
| POST   | `/`      | Create new wallet |
| GET    | `/:id`   | Get wallet by ID |
| PUT    | `/:id`   | Update wallet |
| DELETE | `/:id`   | Delete wallet |

All endpoints require JWT authentication.

## Database Schema
```sql
CREATE TABLE wallets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    name VARCHAR(100) NOT NULL,
    balance DECIMAL(15,2) DEFAULT 0.00,
    currency VARCHAR(20) NOT NULL CHECK (currency IN ('usd', 'eur', 'ron')),
    wallet_type VARCHAR(50) NOT NULL CHECK (wallet_type IN ('general', 'savings', 'business', 'personal')),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
```

**Note:** The wallets table is in the `brewget_transactions` database, separate from `brewget_settings` and `brewget_auth`.

## Architecture
The wallet functionality is implemented as a dedicated **transaction-service** microservice:
- **Service:** transaction-service
- **Port:** 8003 (HTTP)
- **Database:** brewget_transactions
- **Dependencies:** auth-service (for JWT verification via gRPC)

This microservice architecture allows for:
- Independent scaling of wallet/transaction operations
- Separate database for transaction data
- Future expansion to include transaction history and analytics
- Better separation of concerns

## Environment Configuration
Required variables (in `.env`):
```bash
TRANSACTION_HTTP_PORT=8003
TRANSACTION_PG_DATABASE=brewget_transactions
PG_URL=localhost:5432
PG_USERNAME=brewget
PG_PASSWORD=brewget_dev_password
AUTH_HOSTNAME=localhost
AUTH_GRPC_PORT=9000
CORS_URL=http://localhost:5173
```

## Documentation
- Changelogs: See `changelogs/transaction-service-CHANGELOG.md` and `changelogs/frontend-CHANGELOG.md`
- Environment: Updated `.env.example` with transaction service variables
- README: Updated with transaction service mention
- Procfile: Updated with transaction service process

## Testing
Run tests: `cd backend && cargo test --package transaction-service`
Format code: `cargo fmt --all`
Lint: `cargo clippy --all-targets --all-features`
Frontend format: `cd frontend && bun format`

## Running Locally
```bash
# Start all services with overmind
overmind start

# Or manually start transaction service
cd backend && cargo watch -w transaction-service/src -w shared-types/src -w proto -x "run --package transaction-service"
```

The transaction service will be available at `http://localhost:8003`.
