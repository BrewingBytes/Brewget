# Changelog - Transaction Service

All notable changes to the Transaction Service will be documented in this file.

## [0.0.1] - 2025-11-08
### Added
- Initial transaction service with wallet management
- Wallet CRUD operations (GET, POST, PUT, DELETE)
- Database migration for wallets table with constraints
- Currency and WalletType enum support from shared-types
- JWT authentication via auth-service gRPC
- Health check endpoint
- CORS support for all HTTP methods
- Database connection pooling with SQLx
- Comprehensive logging with tracing
