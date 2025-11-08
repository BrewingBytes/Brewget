-- Remove wallet_type column from wallets table
ALTER TABLE wallets DROP COLUMN IF EXISTS wallet_type;
