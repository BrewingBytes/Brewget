-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS passkey_user_sync ON passkey_credentials;
DROP FUNCTION IF EXISTS update_user_has_passkey();
DROP INDEX IF EXISTS idx_passkey_active;
DROP INDEX IF EXISTS idx_passkey_credential_id;
DROP INDEX IF EXISTS idx_passkey_user_id;
DROP TABLE IF EXISTS passkey_credentials;
