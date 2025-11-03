# Wallet Service Migration Guide

## Overview
This guide documents the process of migrating wallet functionality from `settings-service` to a new `wallet-service` microservice.

## Directory Structure to Create

```
backend/wallet-service/
├── Cargo.toml
├── build.rs
├── Dockerfile
├── migrations/
│   ├── 20251103000000_create_wallets.up.sql
│   └── 20251103000000_create_wallets.down.sql
└── src/
    ├── main.rs
    ├── config.rs
    ├── app_state.rs
    ├── grpc.rs
    ├── database.rs
    ├── models.rs
    ├── routes.rs
    ├── database/
    │   └── wallet.rs
    ├── models/
    │   ├── response.rs
    │   └── wallet.rs
    ├── routes/
    │   ├── health.rs
    │   ├── middlewares/
    │   │   └── auth_guard.rs
    │   ├── middlewares.rs
    │   └── wallet.rs
    └── grpc/
        └── auth_service/
```

## Files to Create

### Backend Wallet Service Files

All files should be copied from the current implementation in settings-service with appropriate modifications.

### Files to Revert in Settings Service

The following files in `settings-service` should be reverted to their state before wallet functionality was added:

1. `backend/settings-service/src/database.rs` - Remove wallet module export
2. `backend/settings-service/src/models.rs` - Remove wallet module export  
3. `backend/settings-service/src/routes.rs` - Remove wallet router, revert CORS to only GET/POST
4. Delete `backend/settings-service/src/database/wallet.rs`
5. Delete `backend/settings-service/src/models/wallet.rs`
6. Delete `backend/settings-service/src/routes/wallet.rs`
7. Delete `backend/settings-service/migrations/20251103000000_create_wallets.up.sql`
8. Delete `backend/settings-service/migrations/20251103000000_create_wallets.down.sql`

### Configuration Updates

1. **backend/Cargo.toml**:
   - Add `wallet-service` to workspace members

2. **.env.example**:
   - Add `WALLET_HTTP_PORT=8003`
   - Add `WALLET_PG_DATABASE=brewget_wallets`
   - Add `DATABASE_URL_WALLET=postgres://brewget:brewget_dev_password@localhost:5432/brewget_wallets`

3. **Procfile**:
   - Add wallet service entry

4. **frontend/src/services/api.ts**:
   - Add walletApi axios instance pointing to port 8003

5. **frontend/src/services/wallet.ts**:
   - Update to use walletApi instead of settingsApi

## Database Setup

Create new database:
```sql
CREATE DATABASE brewget_wallets;
GRANT ALL PRIVILEGES ON DATABASE brewget_wallets TO brewget;
```

## Steps to Complete Migration

1. Create directory structure for wallet-service
2. Copy all wallet-related files from settings-service to wallet-service
3. Update wallet-service config to use WALLET_HTTP_PORT and WALLET_PG_DATABASE
4. Revert settings-service files
5. Update workspace configuration
6. Update frontend to use new wallet API endpoint
7. Update Procfile and environment variables
8. Test the new service

## Port Allocations

- auth-service: 8000 (HTTP), 9000 (gRPC)
- email-service: 8001 (HTTP), 9001 (gRPC)
- settings-service: 8002 (HTTP)
- **wallet-service: 8003 (HTTP)** ← NEW
