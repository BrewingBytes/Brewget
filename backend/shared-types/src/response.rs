use serde::Serialize;

/// A generic message response structure
///
/// This struct is used to serialize response messages into JSON format
///
/// # Fields
/// * `message` - The message content to be sent in the response
///
/// # Example
/// ```json
/// {
///     "message": "Operation completed successfully"
/// }
/// ```
#[derive(Serialize)]
pub struct Message {
    pub message: String,
}

/// The response for the /health route
///
/// # Fields
/// * `status` - The current status of the service
/// * `database` - The current status of the connection to the db
/// * `version` - The current version of the service
///
/// # Example
/// ```json
/// {
///     "status": "healthy",
///     "database": "connected",
///     "version": "0.0.2"
/// }
/// ```
#[derive(Serialize)]
pub struct Health {
    pub status: HealthStatus,
    pub database: DatabaseConnection,
    pub version: String,
}

/// The enum for the Health Status
#[derive(Serialize)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
}

/// The enum for the Database Connection Status
#[derive(Serialize)]
pub enum DatabaseConnection {
    Connected,
    Disconnected,
}

/// A JWT response structure
///
/// This struct is used to serialize response JWT into JSON format
///
/// # Fields
/// * `token` - The JWT to be sent in the response after login
///
/// # Example
/// ```json
/// {
///     "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
/// }
/// ```
#[derive(Serialize)]
pub struct Token {
    pub token: String,
}
