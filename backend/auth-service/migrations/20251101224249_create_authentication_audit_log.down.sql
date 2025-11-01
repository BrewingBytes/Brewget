-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_auth_log_user_attempted;
DROP INDEX IF EXISTS idx_auth_log_user_id;
DROP TABLE IF EXISTS authentication_audit_log;
DROP TYPE IF EXISTS auth_method_enum;
