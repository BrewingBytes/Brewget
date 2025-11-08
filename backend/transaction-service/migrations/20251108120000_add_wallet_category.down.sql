-- Remove category from wallets table
DROP INDEX IF EXISTS idx_wallets_category;
ALTER TABLE wallets DROP COLUMN IF EXISTS category;
