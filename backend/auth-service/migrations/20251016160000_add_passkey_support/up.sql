-- Your SQL goes here

-- Make password field nullable in users table to support passkey-only accounts
ALTER TABLE users ALTER COLUMN password DROP NOT NULL;

-- Create passkey_credentials table to store WebAuthn credentials
CREATE TABLE passkey_credentials (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    credential_id BYTEA NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,
    counter BIGINT NOT NULL DEFAULT 0,
    transports TEXT[],
    backup_eligible BOOLEAN NOT NULL DEFAULT FALSE,
    backup_state BOOLEAN NOT NULL DEFAULT FALSE,
    attestation_type TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ
);

-- Create index on user_id for faster lookups
CREATE INDEX idx_passkey_credentials_user_id ON passkey_credentials(user_id);
