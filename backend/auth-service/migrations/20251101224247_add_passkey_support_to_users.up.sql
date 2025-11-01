-- Make password optional for passkey-only accounts
ALTER TABLE users 
ALTER COLUMN password DROP NOT NULL;

-- Add column to track authentication methods
ALTER TABLE users 
ADD COLUMN has_passkey BOOLEAN NOT NULL DEFAULT FALSE;

-- Add index for faster passkey lookups
CREATE INDEX idx_users_has_passkey ON users(has_passkey);
