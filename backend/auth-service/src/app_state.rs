use crate::Config;

pub struct AppState {
    pub config: Config,
    pub db: sqlx::postgres::PgPool,
}
