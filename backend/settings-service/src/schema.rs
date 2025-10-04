// @generated automatically by Diesel CLI.

diesel::table! {
    activation_links (id, user_id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    forgot_password_links (id, user_id) {
        id -> Uuid,
        user_id -> Uuid,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
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
    user_settings (user_id) {
        user_id -> Uuid,
        #[max_length = 20]
        language -> Varchar,
        #[max_length = 20]
        currency -> Varchar,
        alarm_set -> Bool,
        alarm_time -> Time,
        alarm_offset_minutes -> Int4,
        night_mode -> Bool,
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

diesel::joinable!(activation_links -> users (user_id));
diesel::joinable!(forgot_password_links -> users (user_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    activation_links,
    forgot_password_links,
    tokens,
    user_settings,
    users,
);
