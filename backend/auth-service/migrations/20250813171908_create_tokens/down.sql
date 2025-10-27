-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_tokens_user_type;
DROP INDEX IF EXISTS idx_tokens_token;
DROP TABLE IF EXISTS tokens;
