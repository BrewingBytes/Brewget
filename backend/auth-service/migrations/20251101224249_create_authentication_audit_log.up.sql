-- Create ENUM type for authentication methods
DO $$ BEGIN IF NOT EXISTS (
    SELECT 1
    FROM pg_type
    WHERE typname = 'auth_method_enum'
) THEN CREATE TYPE auth_method_enum AS ENUM ('password', 'passkey', 'otp');
END IF;
END $$;
-- Create authentication_audit_log table for tracking authentication events
CREATE TABLE authentication_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    -- Authentication details
    auth_method auth_method_enum NOT NULL,
    success BOOLEAN NOT NULL,
    -- Request context
    ip_address INET,
    user_agent TEXT,
    -- Timestamps
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Additional data (JSON for flexibility)
    metadata JSONB
);
-- Indexes for audit queries
CREATE INDEX idx_auth_log_user_id ON authentication_audit_log(user_id);
CREATE INDEX idx_auth_log_user_attempted ON authentication_audit_log(user_id, attempted_at DESC);
