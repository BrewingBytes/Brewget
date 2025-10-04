# Shared Types

This crate contains common error and response types used across all microservices in the Brewget backend.

## Purpose

Instead of duplicating error handling and response structures across multiple services, this shared library provides a centralized location for:

- Common error types with proper HTTP status code mapping
- Standardized response structures
- Consistent error handling across all services

## Types

### Error Types

- `Error` - Main error type that implements `IntoResponse` for Axum
- Automatic conversions from common error types:
  - `jsonwebtoken::errors::Error`
  - `diesel::result::Error`
  - `deadpool::managed::PoolError`
  - `uuid::Error`
  - `tonic::Status`
  - `Box<dyn std::error::Error>`

### Response Types

- `Message` - Generic message response with a single `message` field
- `Health` - Health check response with status, database connection, and version
- `HealthStatus` - Enum for service health status (Healthy/Unhealthy)
- `DatabaseConnection` - Enum for database connection status (Connected/Disconnected)
- `Token` - JWT token response structure

## Usage

Add this crate to your service's `Cargo.toml`:

```toml
[dependencies]
shared-types = { path = "../shared-types" }
```

Then import the types you need:

```rust
use shared_types::{Error, Message, Health, HealthStatus, DatabaseConnection, Token};
```

## Example

```rust
use axum::{Json, response::IntoResponse};
use shared_types::{Error, Message};

async fn example_handler() -> Result<impl IntoResponse, Error> {
    // Return a success message
    Ok(Json(Message {
        message: "Operation completed successfully".to_string(),
    }))
}

async fn error_handler() -> Result<impl IntoResponse, Error> {
    // Return an error with status code
    Err(Error::new(StatusCode::BAD_REQUEST, "Invalid input"))
}
```
