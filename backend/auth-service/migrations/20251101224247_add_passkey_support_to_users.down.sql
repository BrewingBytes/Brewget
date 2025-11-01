-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_users_has_passkey;
ALTER TABLE users DROP COLUMN IF EXISTS has_passkey;

-- Note: We cannot easily restore the NOT NULL constraint on password
-- as there might be users without passwords at this point.
-- Manual intervention would be required if rollback is needed.
