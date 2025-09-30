// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "timetz", schema = "pg_catalog"))]
    pub struct Timetz;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Timetz;

    user_settings (user_id) {
        user_id -> Uuid,
        #[max_length = 20]
        language -> Varchar,
        #[max_length = 20]
        currency -> Varchar,
        alarm_time -> Nullable<Timetz>,
        night_mode -> Bool,
    }
}
