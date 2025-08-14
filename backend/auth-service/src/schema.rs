// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        #[max_length = 50]
        token_type -> Varchar,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        username -> Varchar,
        password -> Text,
        #[max_length = 255]
        email -> Varchar,
        is_verified -> Bool,
        #[max_length = 20]
        role -> Varchar,
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_login_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(_sqlx_migrations, tokens, users,);
