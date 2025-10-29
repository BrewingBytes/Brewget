# Integration Tests for Backend Services

This directory contains comprehensive integration tests for all backend services in the Brewget application.

## Overview

The backend consists of three microservices:
- **auth-service**: Authentication and authorization service
- **settings-service**: User settings management service
- **email-service**: Email notification service

Each service has its own integration test suite that validates both HTTP endpoints and gRPC interfaces.

## Test Infrastructure

The integration tests use:
- **testcontainers**: To spin up isolated PostgreSQL instances for database-dependent tests
- **axum-test**: For testing HTTP endpoints
- **tonic**: For testing gRPC interfaces
- **Mock services**: To simulate dependencies between services

## Running the Tests

### Prerequisites

- Docker must be running (required for testcontainers)
- Rust toolchain (specified in `rust-toolchain.toml`)
- Protocol Buffers compiler (`protoc`)

### Run All Tests

From the `backend` directory:

```bash
cargo test
```

### Run Tests for a Specific Service

```bash
# Auth service tests
cd auth-service && cargo test --test integration_test

# Settings service tests
cd settings-service && cargo test --test integration_test

# Email service tests
cd email-service && cargo test --test integration_test
```

### Run Tests Sequentially

Some tests may benefit from running sequentially to avoid resource contention:

```bash
cargo test -- --test-threads=1
```

## Test Coverage

### Auth Service (11 tests)

Tests cover:
- Health check endpoint
- User registration (success, validation errors, duplicates)
- User login (success, wrong password, inactive user)
- Logout functionality
- Forgot password flow
- gRPC token verification

### Settings Service (8 tests)

Tests cover:
- Health check endpoint
- Get user settings (authorized and unauthorized)
- Update user settings (authorized and unauthorized)
- Settings persistence
- Invalid token handling
- CORS headers

### Email Service (10 tests)

Tests cover:
- Account activation emails (success and validation)
- Password reset emails (success and validation)
- Email format validation
- Multiple concurrent requests
- Template generation

## Test Architecture

Each service's integration tests follow a similar pattern:

1. **Test Fixture**: Sets up isolated test environment including:
   - PostgreSQL database (via testcontainers)
   - Mock gRPC services for dependencies
   - HTTP test server
   - Test data

2. **Dynamic Ports**: All services use dynamically allocated ports to avoid conflicts

3. **Cleanup**: Resources are automatically cleaned up when tests complete

## CI/CD Integration

These tests are designed to run in CI/CD pipelines. They:
- Are fully isolated and don't require external services
- Use Docker containers that are automatically cleaned up
- Can run in parallel (though `--test-threads=1` is recommended for stability)
- Provide clear error messages for debugging

## Troubleshooting

### Docker Not Running

If you see errors about Docker not being available:
```
Error: docker daemon is not running
```

Solution: Start Docker Desktop or the Docker daemon.

### Port Conflicts

If tests fail with "Address already in use" errors:
- Run tests sequentially with `--test-threads=1`
- Ensure no other services are bound to dynamically allocated ports
- Wait a moment between test runs for ports to be released

### Test Timeouts

If tests timeout:
- Increase the timeout with `cargo test -- --test-threads=1`
- Check Docker resource allocation (CPU/memory)
- Ensure your system has sufficient resources

## Development

### Adding New Tests

1. Follow the existing test patterns in each service
2. Use the `TestFixture` struct for consistent setup
3. Ensure tests are independent and can run in any order
4. Add appropriate assertions for both success and failure cases

### Mock Services

When adding tests that require inter-service communication:
- Create mock implementations of gRPC services
- Use dynamic port allocation
- Implement minimal logic needed for test scenarios

## Performance

Typical test execution times:
- Email service: ~1 second (no Docker containers)
- Settings service: ~15-20 seconds (includes PostgreSQL startup)
- Auth service: ~30-35 seconds (includes PostgreSQL startup)

Total test suite: ~45-60 seconds on typical hardware.
