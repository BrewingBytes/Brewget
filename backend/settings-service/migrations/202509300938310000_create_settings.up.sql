-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE user_settings (
    user_id UUID PRIMARY KEY NOT NULL,
    language VARCHAR(20) NOT NULL DEFAULT 'en',
    currency VARCHAR(20) NOT NULL DEFAULT 'eur',
    alarm_set BOOLEAN NOT NULL DEFAULT FALSE,
    alarm_time TIME NOT NULL DEFAULT '07:00:00',
    alarm_offset_minutes INT NOT NULL DEFAULT 0,
    night_mode BOOLEAN NOT NULL DEFAULT FALSE
);
