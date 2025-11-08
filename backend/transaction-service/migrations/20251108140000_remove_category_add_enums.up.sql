-- Drop category column and its index
DROP INDEX IF EXISTS idx_wallets_category;
ALTER TABLE wallets DROP COLUMN IF EXISTS category;

-- Update currency column to use enum values from shared-types
-- Currency enum: USD, EUR, GBP, CAD, JPY, RON
ALTER TABLE wallets ALTER COLUMN currency TYPE VARCHAR(3);
COMMENT ON COLUMN wallets.currency IS 'Currency code matching shared-types Currency enum (USD, EUR, GBP, CAD, JPY, RON)';

-- Update wallet_type column to match shared-types WalletType enum
-- WalletType enum: Account, Savings, Deposit, CreditCard, Loan
ALTER TABLE wallets ALTER COLUMN wallet_type TYPE VARCHAR(20);
COMMENT ON COLUMN wallets.wallet_type IS 'Type of wallet matching shared-types WalletType enum (Account, Savings, Deposit, CreditCard, Loan)';

-- Create index for wallet type grouping
CREATE INDEX idx_wallets_type ON wallets(user_id, wallet_type);
