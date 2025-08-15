use deadpool::managed::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::Config;

/// Application state shared across all routes
///
/// Contains configuration and database connection pool
/// that can be accessed by route handlers
///
/// # Fields
/// * `config` - Application configuration settings
/// * `db` - PostgreSQL connection pool for async database operations
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
    pub db: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}

impl AppState {
    /// Gets a connection from the database pool
    ///
    /// # Returns
    /// * `Ok(Object)` - A connection from the pool
    /// * `Err(PoolError)` - If connection acquisition fails
    ///
    /// # Example
    /// ```rust
    /// let conn = state.get_database_connection().await?;
    /// // Use connection for database operations
    /// ```
    ///
    /// # Errors
    /// Returns error if:
    /// * Pool is exhausted (too many connections)
    /// * Connection establishment fails
    /// * Database is unreachable
    pub async fn get_database_connection(
        &self,
    ) -> Result<
        deadpool::managed::Object<AsyncDieselConnectionManager<AsyncPgConnection>>,
        deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>,
    > {
        self.db.get().await
    }
}
