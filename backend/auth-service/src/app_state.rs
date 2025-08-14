use deadpool::managed::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

use crate::Config;

pub struct AppState {
    pub config: Config,
    pub db: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
}
