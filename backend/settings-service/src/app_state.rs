use sqlx::PgPool;

use crate::Config;

/// Application state shared across all routes
///
/// Contains configuration and database connection pool
/// that can be accessed by route handlers
///
/// # Fields
/// * `config` - Application configuration settings
/// * `db` - PostgreSQL connection pool for async database operations
/// * `email_service` - A mutex for the EmailServiceClient GRPC
///
/// # Usage
/// ```rust
/// use axum::extract::State;
///
/// async fn handler(State(state): State<Arc<AppState>>) {
///     let mut conn = state.db.get().await?;
///     // Use connection...
/// }
/// ```
pub struct AppState {
    pub config: Config,
    db: PgPool,
}

impl AppState {
    /// Creates a new AppState
    ///
    /// # Returns
    /// * `AppState` - the AppState that contains all the necessary configs
    pub fn new(config: Config, db: PgPool) -> Self {
        Self { config, db }
    }

    /// Gets a reference to the database pool
    ///
    /// # Returns
    /// * `&PgPool` - A reference to the database pool
    ///
    /// # Example
    /// ```rust
    /// let pool = state.get_database_pool();
    /// // Use pool for database operations
    /// ```
    pub fn get_database_pool(&self) -> &PgPool {
        &self.db
    }
}
