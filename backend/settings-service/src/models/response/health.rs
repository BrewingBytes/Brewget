use serde::Serialize;

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
