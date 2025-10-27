-- This file should undo anything in `up.sql`

-- Drop the passkey_credentials table
DROP TABLE IF EXISTS passkey_credentials;

-- Make password field NOT NULL again (this will fail if there are passkey-only users)
-- We need to handle this carefully - this is a destructive operation
-- For safety, we'll just comment it out. If you need to roll back completely,
-- you'll need to manually clean up any passkey-only users first.
-- ALTER TABLE users ALTER COLUMN password SET NOT NULL;
