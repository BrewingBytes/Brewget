-- Create authentication_audit_log table for tracking authentication events
CREATE TABLE authentication_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Authentication details
    auth_method VARCHAR(20) NOT NULL, -- 'password', 'passkey', 'otp'
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
CREATE INDEX idx_auth_log_attempted_at ON authentication_audit_log(attempted_at DESC);
CREATE INDEX idx_auth_log_method ON authentication_audit_log(auth_method);
CREATE INDEX idx_auth_log_user_attempted ON authentication_audit_log(user_id, attempted_at DESC);
