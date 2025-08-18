-- This file should undo anything in `up.sql`
-- Drop the trigger from the tokens table
DROP TRIGGER IF EXISTS token_insert_update_last_login ON tokens;
-- Drop the trigger function
DROP FUNCTION IF EXISTS update_last_login_at();
