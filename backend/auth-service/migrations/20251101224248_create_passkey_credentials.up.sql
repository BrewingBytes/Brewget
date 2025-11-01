-- Create passkey_credentials table for WebAuthn credentials
CREATE TABLE passkey_credentials (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- WebAuthn credential data
    credential_id BYTEA NOT NULL UNIQUE,
    public_key BYTEA NOT NULL,
    counter BIGINT NOT NULL DEFAULT 0 CHECK (counter >= 0),
    
    -- Credential metadata
    aaguid BYTEA,
    credential_device_type TEXT,
    credential_backed_up BOOLEAN DEFAULT FALSE,
    
    -- User-facing information
    device_name TEXT,
    user_agent TEXT,
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ,
    
    -- Additional security
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Indexes for performance
CREATE INDEX idx_passkey_user_id ON passkey_credentials(user_id);
CREATE INDEX idx_passkey_active ON passkey_credentials(is_active) WHERE is_active = TRUE;

-- Trigger to update users.has_passkey when passkeys are added or removed
CREATE OR REPLACE FUNCTION update_user_has_passkey()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE users SET has_passkey = TRUE WHERE id = NEW.user_id;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE users 
        SET has_passkey = EXISTS(
            SELECT 1 FROM passkey_credentials 
            WHERE user_id = OLD.user_id AND is_active = TRUE
        )
        WHERE id = OLD.user_id;
    ELSIF TG_OP = 'UPDATE' AND OLD.is_active IS DISTINCT FROM NEW.is_active THEN
        UPDATE users 
        SET has_passkey = EXISTS(
            SELECT 1 FROM passkey_credentials 
            WHERE user_id = NEW.user_id AND is_active = TRUE
        )
        WHERE id = NEW.user_id;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER passkey_user_sync
AFTER INSERT OR DELETE OR UPDATE ON passkey_credentials
FOR EACH ROW
EXECUTE FUNCTION update_user_has_passkey();
