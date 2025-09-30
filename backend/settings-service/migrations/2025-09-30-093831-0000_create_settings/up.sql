-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE user_settings (
    user_id UUID PRIMARY KEY NOT NULL,
    language VARCHAR(20) NOT NULL DEFAULT 'en-us',
    currency VARCHAR(20) NOT NULL DEFAULT 'eur',
    alarm_time TIME WITH TIME ZONE,
    night_mode BOOLEAN NOT NULL DEFAULT FALSE
);
